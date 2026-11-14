use serde::Deserialize;
use base64::{engine::general_purpose, Engine as _};
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct TmeTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
}

#[derive(Deserialize, Debug)]
struct TmeSearchResponse {
    pub status: String,
    pub data: Option<TmeSearchData>,
}

#[derive(Deserialize, Debug)]
struct TmeSearchData {
    #[serde(rename = "products")]
    pub products: Option<TmeSearchProducts>,
}

#[derive(Deserialize, Debug)]
struct TmeSearchProducts {
    #[serde(rename = "elements")]
    pub elements: Option<Vec<TmeElement>>,
}

#[derive(Deserialize, Debug)]
struct TmeElement {
    #[serde(rename = "symbol")]
    pub symbol: Option<String>,
    #[serde(rename = "description")]
    pub description: Option<String>,
    #[serde(rename = "manufacturer")]
    pub manufacturer: Option<TmeManufacturer>,
}

#[derive(Deserialize, Debug)]
struct TmeManufacturer {
    #[serde(rename = "name")]
    pub name: Option<String>,
}

#[derive(Deserialize, Debug)]
struct TmeFilesResponse {
    pub status: String,
    pub data: Option<TmeFilesData>,
}

#[derive(Deserialize, Debug)]
struct TmeFilesData {
    #[serde(rename = "elements")]
    pub elements: Option<Vec<TmeFileElement>>,
}

#[derive(Deserialize, Debug)]
struct TmeFileElement {
    #[serde(rename = "symbol")]
    pub symbol: Option<String>,
    #[serde(rename = "documents")]
    pub documents: Option<TmeDocumentsWrapper>,
}

#[derive(Deserialize, Debug)]
struct TmeDocumentsWrapper {
    #[serde(rename = "elements")]
    pub elements: Option<Vec<TmeDocumentItem>>,
}

#[derive(Deserialize, Debug)]
struct TmeDocumentItem {
    #[serde(rename = "url")]
    pub url: Option<String>,
    #[serde(rename = "type")]
    pub doc_type: Option<String>,
    #[serde(rename = "content")]
    pub content: Option<String>,
}

// ВАЖНО: option_env! вставляет значение на этапе КОМПИЛЯЦИИ.
// Токен будет захардкожен в бинарнике. Работает и на Android.
fn get_credentials() -> Result<(String, String), String> {
    // Пробуем option_env! (вкомпилированное значение)
    let compiled_token = option_env!("TME_TOKEN");
    let compiled_secret = option_env!("TME_SECRET");

    if let (Some(t), Some(s)) = (compiled_token, compiled_secret) {
        return Ok((t.to_string(), s.to_string()));
    }

    // Fallback на runtime env (для ПК, если запускаешь без скрипта)
    dotenvy::dotenv().ok();
    let token = std::env::var("TME_TOKEN")
        .map_err(|_| "TME_TOKEN is not set (compile-time or runtime)")?;
    let secret = std::env::var("TME_SECRET")
        .map_err(|_| "TME_SECRET is not set (compile-time or runtime)")?;
    Ok((token, secret))
}

async fn get_oauth_token() -> Result<String, String> {
    let (token, secret) = get_credentials()?;
    let url = "https://api.tme.eu/auth/token";

    let credentials = format!("{}:{}", token, secret);
    let encoded_auth = general_purpose::STANDARD.encode(credentials.as_bytes());

    let client = reqwest::Client::new();
    let resp = client.post(url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("Authorization", format!("Basic {}", encoded_auth))
        .body("grant_type=client_credentials")
        .send().await
        .map_err(|e| format!("Auth request network error: {}", e))?;

    let status = resp.status();
    let text = resp.text().await.unwrap_or_default();

    if !status.is_success() {
        return Err(format!("TME Auth HTTP error: {} — {}", status, text));
    }

    let token_resp: TmeTokenResponse = serde_json::from_str(&text)
        .map_err(|e| format!("Failed to parse token response: {}", e))?;

    Ok(token_resp.access_token)
}

#[tauri::command]
pub async fn search_tme(query: String) -> Result<Vec<crate::digikey::DatasheetResult>, String> {
    let access_token = match get_oauth_token().await {
        Ok(t) => t,
        Err(e) => return Err(e),
    };

    let client = reqwest::Client::new();

    let search_url = format!(
        "https://api.tme.eu/products/search?country=PL&scope[]=products&limit=10&phrase={}",
        urlencoding::encode(&query)
    );

    let search_resp = client.get(&search_url)
        .header("Authorization", format!("Bearer {}", access_token))
        .header("Accept-Language", "en")
        .header("Accept", "application/json")
        .send().await
        .map_err(|e| format!("TME search network error: {}", e))?;

    let search_status = search_resp.status();
    let search_body = search_resp.text().await.map_err(|e| e.to_string())?;

    if !search_status.is_success() {
        return Err(format!("TME API error: {} — {}", search_status, search_body));
    }

    let parsed_search: TmeSearchResponse = serde_json::from_str(&search_body)
        .map_err(|e| format!("TME search parse error: {}", e))?;

    let elements = parsed_search.data
        .and_then(|d| d.products)
        .and_then(|p| p.elements)
        .unwrap_or_default();

    if elements.is_empty() {
        return Err(format!("No results found for \"{}\"", query));
    }

    let symbols: Vec<String> = elements.iter()
        .filter_map(|e| e.symbol.clone())
        .collect();

    let mut pdf_map: HashMap<String, String> = HashMap::new();

    if !symbols.is_empty() {
        let symbols_query = symbols.iter()
            .map(|s| format!("symbols[]={}", urlencoding::encode(s)))
            .collect::<Vec<_>>()
            .join("&");

        let files_url = format!("https://api.tme.eu/products/files?country=PL&{}", symbols_query);

        if let Ok(files_resp) = client.get(&files_url)
            .header("Authorization", format!("Bearer {}", access_token))
            .header("Accept-Language", "en")
            .header("Accept", "application/json")
            .send().await
        {
            if let Ok(files_body) = files_resp.text().await {
                if let Ok(parsed_files) = serde_json::from_str::<TmeFilesResponse>(&files_body) {
                    if let Some(file_elems) = parsed_files.data.and_then(|d| d.elements) {
                        for fe in file_elems {
                            let Some(sym) = fe.symbol else { continue };
                            let Some(docs) = fe.documents.and_then(|d| d.elements) else { continue };

                            let dte_url = docs.iter().find_map(|doc| {
                                if doc.doc_type.as_deref() == Some("DTE") {
                                    doc.url.as_ref().map(|u| {
                                        if u.starts_with("//") { format!("https:{}", u) } else { u.clone() }
                                    })
                                } else {
                                    None
                                }
                            });

                            let lnk_url = docs.iter().find_map(|doc| {
                                if doc.doc_type.as_deref() == Some("LNK") {
                                    doc.content.as_ref().map(|c| c.trim().to_string())
                                        .filter(|c| c.starts_with("http"))
                                } else {
                                    None
                                }
                            });

                            if let Some(final_url) = dte_url.or(lnk_url) {
                                pdf_map.insert(sym, final_url);
                            }
                        }
                    }
                }
            }
        }
    }

    let mut results = Vec::new();
    for elem in elements {
        let symbol = elem.symbol.clone().unwrap_or_default();
        let raw_name = elem.description
            .clone()
            .unwrap_or_else(|| symbol.clone());
        
        let name = if !symbol.is_empty() {
            format!("{} [{}]", raw_name, symbol)
        } else {
            raw_name
        };

        let manufacturer = elem.manufacturer
            .as_ref()
            .and_then(|m| m.name.clone())
            .unwrap_or_default();
        let datasheet_url = pdf_map.get(&symbol).cloned().unwrap_or_default();

        results.push(crate::digikey::DatasheetResult {
            name,
            manufacturer,
            datasheet_url,
        });
    }

    Ok(results)
}