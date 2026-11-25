use reqwest::Client;
use serde::Serialize;
use serde_json::Value;

const JLCPCB_SEARCH_URL: &str = "https://jlcpcb.com/api/overseas-pcb-order/v1/shoppingCart/smtGood/selectSmtComponentList/v2";
const LCSC_DETAIL_URL: &str = "https://wmsc.lcsc.com/ftps/wm/product/detail";

#[derive(Serialize)]
pub struct LcscResult {
    pub name: String,
    pub manufacturer: String,
    pub datasheet_url: String,
    pub lcsc_code: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct AnalogResult {
    pub name: String,
    pub manufacturer: String,
    pub datasheet_url: String,
    pub source: String,
    pub vds: Option<f64>,
    pub id: Option<f64>,
    pub rds_on: Option<f64>,
    pub vceo: Option<f64>,
    pub ic: Option<f64>,
    pub hfe: Option<f64>,
    pub tr_type: Option<String>,
}

fn truncate_for_error(text: &str) -> String {
    text.chars().take(160).collect()
}

fn parse_lcsc_params(comp: &Value, _tr_type: &str) -> Option<(Option<f64>, Option<f64>, Option<f64>, Option<f64>, Option<f64>, Option<f64>, Option<String>)> {
    let attrs = comp.get("attributes")?.as_array()?;

    let mut vds = None;
    let mut id = None;
    let mut rds_on = None;
    let mut vceo = None;
    let mut ic = None;
    let mut hfe = None;
    let mut ttype = None;

    for attr in attrs {
        let (Some(name), Some(value)) = (
            attr.get("attribute_name_en").and_then(|v| v.as_str()),
            attr.get("attribute_value_name").and_then(|v| v.as_str()),
        ) else {
            // Skip malformed attributes instead of dropping the whole product.
            continue;
        };

        match name {
            "Drain to Source Voltage" => {
                vds = value.trim_end_matches('V').parse::<f64>().ok();
            }
            "Current - Continuous Drain(Id)" => {
                id = value.trim_end_matches('A').parse::<f64>().ok();
            }
            "RDS(on)" => {
                if let Some(mohm) = value.split('m').next() {
                    rds_on = mohm.parse::<f64>().ok();
                }
            }
            "Collector - Emitter Voltage VCEO" | "Voltage - Collector Emitter Breakdown (Max)" => {
                vceo = value.trim_end_matches('V').parse::<f64>().ok();
            }
            "Current - Collector(Ic)" => {
                ic = value.trim_end_matches('A').parse::<f64>().ok();
            }
            "DC Current Gain" => {
                hfe = value.parse::<f64>().ok();
            }
            "Type" => {
                ttype = Some(if value.contains("N-Channel") { "N" } else if value.contains("P-Channel") { "P" } else if value.contains("NPN") { "NPN" } else { "PNP" }.to_string());
            }
            _ => {}
        }
    }

    Some((vds, id, rds_on, vceo, ic, hfe, ttype))
}

// Рекурсивно ищем в JSON массив, элементы которого содержат "componentCode"
fn find_components_array(value: &Value) -> Option<&Vec<Value>> {
    if let Some(arr) = value.as_array() {
        if !arr.is_empty() && arr[0].get("componentCode").is_some() {
            return Some(arr);
        }
        for item in arr {
            if let Some(found) = find_components_array(item) {
                return Some(found);
            }
        }
    } else if let Some(obj) = value.as_object() {
        for (_, v) in obj {
            if let Some(found) = find_components_array(v) {
                return Some(found);
            }
        }
    }
    None
}

/// Single entry point for the JLCPCB search API.
///
/// It used to call `res.json()` directly, so an API-level failure
/// (`{"success": false, ...}`) silently looked like "no results".  Now the real
/// reason is returned to the UI.
async fn jlcpcb_search(
    client: &Client,
    keyword: &str,
    page_size: usize,
) -> Result<Vec<Value>, String> {
    let body = serde_json::json!({
        "keyword": keyword,
        "currentPage": 1,
        "pageSize": page_size
    });

    let res = client
        .post(JLCPCB_SEARCH_URL)
        .header("User-Agent", crate::net::BROWSER_USER_AGENT)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .header("Origin", "https://jlcpcb.com")
        .header("Referer", "https://jlcpcb.com/parts")
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("JLCPCB request error: {}", e))?;

    let status = res.status();
    let text = res
        .text()
        .await
        .map_err(|e| format!("JLCPCB response error: {}", e))?;

    if !status.is_success() {
        return Err(format!(
            "JLCPCB API returned HTTP {}: {}",
            status.as_u16(),
            truncate_for_error(&text)
        ));
    }

    let json: Value = serde_json::from_str(&text)
        .map_err(|e| format!("JLCPCB parse error: {} ({})", e, truncate_for_error(&text)))?;

    if json.get("success").and_then(|v| v.as_bool()) == Some(false) {
        let message = json.get("message").and_then(|v| v.as_str()).unwrap_or("unknown error");
        let code = json.get("code").and_then(|v| v.as_i64()).unwrap_or(0);
        return Err(format!("LCSC / JLCPCB API error (code {}): {}", code, message));
    }

    Ok(find_components_array(&json).cloned().unwrap_or_default())
}

/// Resolves the datasheet PDF URLs for all components concurrently.  It used to
/// be one request per component inside a `for` loop (20–50 sequential HTTPS
/// round-trips per search).
async fn fetch_pdf_urls(client: &'static Client, codes: &[String]) -> Vec<String> {
    let mut urls = vec![String::new(); codes.len()];
    let mut tasks = Vec::new();
    for (index, code) in codes.iter().enumerate() {
        if code.is_empty() {
            continue;
        }
        let code = code.clone();
        tasks.push((index, tokio::spawn(async move { fetch_lcsc_pdf_url(client, &code).await })));
    }
    for (index, task) in tasks {
        match task.await {
            Ok(Some(url)) => urls[index] = url,
            _ => {}
        }
    }
    urls
}

#[tauri::command]
pub async fn search_lcsc(query: String) -> Result<Vec<LcscResult>, String> {
    let client = crate::net::api_client()?;
    let components = jlcpcb_search(client, &query, 20).await?;

    let mut items: Vec<(String, String, String)> = Vec::new();
    for comp in &components {
        let lcsc_code = comp.get("componentCode").and_then(|v| v.as_str()).unwrap_or("").to_string();
        if lcsc_code.is_empty() {
            continue;
        }
        let name = comp.get("componentModelEn").and_then(|v| v.as_str()).unwrap_or("Unknown").to_string();
        let manufacturer = comp.get("componentBrandEn").and_then(|v| v.as_str()).unwrap_or("Unknown").to_string();
        items.push((lcsc_code, name, manufacturer));
    }

    let codes: Vec<String> = items.iter().map(|(code, _, _)| code.clone()).collect();
    let urls = fetch_pdf_urls(client, &codes).await;

    let results = items
        .into_iter()
        .zip(urls)
        .map(|((name, manufacturer, lcsc_code), datasheet_url)| LcscResult {
            name,
            manufacturer,
            datasheet_url,
            lcsc_code,
        })
        .collect();

    Ok(results)
}

#[tauri::command]
pub async fn find_analogs_lcsc(query: String, tr_type: String) -> Result<Vec<AnalogResult>, String> {
    let client = crate::net::api_client()?;

    let components = jlcpcb_search(client, &query, 1).await?;
    let comp = components.first().ok_or("Original component not found")?;
    let _params = parse_lcsc_params(comp, &tr_type).ok_or("Failed to parse parameters")?;

    let search_keyword = if tr_type == "mosfet" { "MOSFET" } else { "Bipolar Transistor" };
    let components = jlcpcb_search(client, search_keyword, 30).await?;
    let codes: Vec<String> = components
        .iter()
        .map(|c| c.get("componentCode").and_then(|v| v.as_str()).unwrap_or("").to_string())
        .collect();
    let urls = fetch_pdf_urls(client, &codes).await;

    let mut results = Vec::new();
    for (index, c) in components.iter().enumerate() {
        let name = c.get("componentModelEn").and_then(|v| v.as_str()).unwrap_or("").to_string();
        if name.is_empty() || name == query {
            continue;
        }

        let manufacturer = c.get("componentBrandEn").and_then(|v| v.as_str()).unwrap_or("").to_string();
        let datasheet_url = urls.get(index).cloned().unwrap_or_default();

        if let Some((vds, id, rds, vceo, ic, hfe, ttype)) = parse_lcsc_params(c, &tr_type) {
            results.push(AnalogResult {
                name, manufacturer, datasheet_url, source: "LCSC".to_string(),
                vds, id, rds_on: rds, vceo, ic, hfe, tr_type: ttype,
            });
        }
    }

    Ok(results)
}

#[tauri::command]
pub async fn find_by_params_lcsc(params: serde_json::Value) -> Result<Vec<AnalogResult>, String> {
    let tr_type = params.get("type").and_then(|v| v.as_str()).unwrap_or("mosfet");
    let vds_min = params.get("vds").and_then(|v| v.as_f64());
    let id_min = params.get("id").and_then(|v| v.as_f64());
    let rds_max = params.get("rdsOn").and_then(|v| v.as_f64());
    let vceo_min = params.get("vceo").and_then(|v| v.as_f64());
    let ic_min = params.get("ic").and_then(|v| v.as_f64());
    let hfe_min = params.get("hfe").and_then(|v| v.as_f64());

    let client = crate::net::api_client()?;
    let search_keyword = if tr_type == "mosfet" { "MOSFET" } else { "Bipolar Transistor" };
    let components = jlcpcb_search(client, search_keyword, 50).await?;
    let codes: Vec<String> = components
        .iter()
        .map(|c| c.get("componentCode").and_then(|v| v.as_str()).unwrap_or("").to_string())
        .collect();
    let urls = fetch_pdf_urls(client, &codes).await;

    let mut results = Vec::new();
    for (index, c) in components.iter().enumerate() {
        let name = c.get("componentModelEn").and_then(|v| v.as_str()).unwrap_or("").to_string();
        if name.is_empty() {
            continue;
        }

        let manufacturer = c.get("componentBrandEn").and_then(|v| v.as_str()).unwrap_or("").to_string();
        let datasheet_url = urls.get(index).cloned().unwrap_or_default();

        if let Some((vds, id, rds, vceo, ic, hfe, ttype)) = parse_lcsc_params(c, tr_type) {
            let mut pass = true;
            if tr_type == "mosfet" {
                if let Some(min) = vds_min { if vds.unwrap_or(0.0) < min { pass = false; } }
                if let Some(min) = id_min { if id.unwrap_or(0.0) < min { pass = false; } }
                if let Some(max) = rds_max { if let Some(r) = rds { if r > max { pass = false; } } else { pass = false; } }
            } else {
                if let Some(min) = vceo_min { if vceo.unwrap_or(0.0) < min { pass = false; } }
                if let Some(min) = ic_min { if ic.unwrap_or(0.0) < min { pass = false; } }
                if let Some(min) = hfe_min { if hfe.unwrap_or(0.0) < min { pass = false; } }
            }

            if pass {
                results.push(AnalogResult {
                    name, manufacturer, datasheet_url, source: "LCSC".to_string(),
                    vds, id, rds_on: rds, vceo, ic, hfe, tr_type: ttype,
                });
            }
        }
    }

    Ok(results)
}

async fn fetch_lcsc_pdf_url(client: &Client, lcsc_code: &str) -> Option<String> {
    let res = client
        .get(LCSC_DETAIL_URL)
        .query(&[("productCode", lcsc_code)])
        .header("User-Agent", crate::net::BROWSER_USER_AGENT)
        .header("Accept", "application/json")
        .send()
        .await
        .ok()?;

    if !res.status().is_success() {
        return None;
    }

    let json: Value = res.json().await.ok()?;
    json.get("result")
        .and_then(|r| r.get("pdfUrl"))
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|url| !url.is_empty())
        .map(|s| s.to_string())
}
