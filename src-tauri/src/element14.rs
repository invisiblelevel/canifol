use reqwest::Client;
use serde::Serialize;
use serde_json::Value;

const ELEMENT14_URL: &str = "https://api.element14.com/catalog/products";

#[derive(Serialize)]
pub struct Element14Result {
    pub name: String,
    pub manufacturer: String,
    pub datasheet_url: String,
    pub lcsc_code: String,
}

fn get_api_key() -> Result<String, String> {
    // 1. Пробуем env (для dev)
    dotenvy::dotenv().ok();
    if let Ok(key) = std::env::var("ELEMENT14_API_KEY") {
        if !key.is_empty() {
            return Ok(key);
        }
    }

    // 2. Fallback на compile-time инъекцию (для релиза / CI)
    option_env!("ELEMENT14_API_KEY")
        .map(|s| s.to_string())
        .ok_or("ELEMENT14_API_KEY is not set. Please set it in .env (dev) or GitHub Secrets (CI)".to_string())
}

#[tauri::command]
pub async fn search_element14(query: String) -> Result<Vec<Element14Result>, String> {
    let api_key = get_api_key()?;

    let client = Client::builder()
        .user_agent("Canifol/1.5.0 (repair tool)")
        .timeout(std::time::Duration::from_secs(15))
        .build()
        .map_err(|e| format!("Client error: {}", e))?;

    let res = client
        .get(ELEMENT14_URL)
        .query(&[
            ("term", format!("manuPartNum:{}", query)),
            ("storeInfo.id", "uk.farnell.com".to_string()),
            ("resultsSettings.offset", "0".to_string()),
            ("resultsSettings.numberOfResults", "20".to_string()),
            ("resultsSettings.responseGroup", "medium".to_string()),
            ("callInfo.responseDataFormat", "JSON".to_string()),
            ("callInfo.apiKey", api_key),
        ])
        .send()
        .await
        .map_err(|e| format!("Request error: {}", e))?;

    if !res.status().is_success() {
        let status = res.status();
        let text = res.text().await.unwrap_or_default();
        return Err(format!("Server returned {}: {}", status, &text[..std::cmp::min(200, text.len())]));
    }

    let json: Value = res.json().await.map_err(|e| format!("Parse error: {}", e))?;
    let mut results = Vec::new();

    if let Some(products) = json
        .get("manufacturerPartNumberSearchReturn")
        .and_then(|r| r.get("products"))
        .and_then(|p| p.as_array())
    {
        for prod in products {
            let name = prod.get("translatedManufacturerPartNumber")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown")
                .to_string();

            let manufacturer = prod.get("brandName")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown")
                .to_string();

            let sku = prod.get("sku")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            let datasheet_url = prod.get("datasheets")
                .and_then(|d| d.as_array())
                .and_then(|arr| arr.first())
                .and_then(|ds| ds.get("url"))
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            if name.is_empty() || sku.is_empty() { continue; }

            results.push(Element14Result {
                name,
                manufacturer,
                datasheet_url,
                lcsc_code: sku,
            });
        }
    }

    Ok(results)
}