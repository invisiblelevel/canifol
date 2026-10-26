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
        let name = attr.get("attribute_name_en")?.as_str()?;
        let value = attr.get("attribute_value_name")?.as_str()?;

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

#[tauri::command]
pub async fn search_lcsc(query: String) -> Result<Vec<LcscResult>, String> {
    let client = Client::new();

    let body = serde_json::json!({
        "keyword": query,
        "currentPage": 1,
        "pageSize": 20
    });

    let res = client
        .post(JLCPCB_SEARCH_URL)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64)")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Ошибка поиска JLCPCB: {}", e))?;

    let json: Value = res.json().await.map_err(|e| format!("Ошибка парсинга: {}", e))?;
    let mut results = Vec::new();

    if let Some(components) = json
        .get("data")
        .and_then(|d| d.get("componentPageInfo"))
        .and_then(|c| c.get("list"))
        .and_then(|l| l.as_array())
    {
        for comp in components {
            let lcsc_code = comp.get("componentCode").and_then(|v| v.as_str()).unwrap_or("").to_string();
            if lcsc_code.is_empty() { continue; }

            let name = comp.get("componentModelEn").and_then(|v| v.as_str()).unwrap_or("Без названия").to_string();
            let manufacturer = comp.get("componentBrandEn").and_then(|v| v.as_str()).unwrap_or("Неизвестен").to_string();
            let datasheet_url = fetch_lcsc_pdf_url(&client, &lcsc_code).await
                .unwrap_or_else(|| format!("https://datasheet.lcsc.com/lcsc/datasheet_{}.pdf", lcsc_code));

            results.push(LcscResult { name, manufacturer, datasheet_url, lcsc_code });
        }
    }
    Ok(results)
}

#[tauri::command]
pub async fn find_analogs_lcsc(query: String, tr_type: String) -> Result<Vec<AnalogResult>, String> {
    let client = Client::new();

    let body = serde_json::json!({
        "keyword": query,
        "currentPage": 1,
        "pageSize": 1
    });

    let res = client
        .post(JLCPCB_SEARCH_URL)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64)")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Ошибка поиска: {}", e))?;

    let json: Value = res.json().await.map_err(|e| format!("Ошибка парсинга: {}", e))?;
    let comp = json
        .get("data")
        .and_then(|d| d.get("componentPageInfo"))
        .and_then(|c| c.get("list"))
        .and_then(|l| l.as_array())
        .and_then(|a| a.first())
        .ok_or("Оригинал не найден")?;

    let params = parse_lcsc_params(comp, &tr_type).ok_or("Не удалось распарсить параметры")?;
    println!("[DEBUG] Параметры оригинала LCSC: {:?}", params);

    let search_keyword = if tr_type == "mosfet" { "MOSFET" } else { "Bipolar Transistor" };
    
    let body = serde_json::json!({
        "keyword": search_keyword,
        "currentPage": 1,
        "pageSize": 30
    });

    let res = client
        .post(JLCPCB_SEARCH_URL)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64)")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Ошибка поиска аналогов: {}", e))?;

    let json: Value = res.json().await.map_err(|e| format!("Ошибка парсинга: {}", e))?;
    let mut results = Vec::new();

    if let Some(components) = json
        .get("data")
        .and_then(|d| d.get("componentPageInfo"))
        .and_then(|c| c.get("list"))
        .and_then(|l| l.as_array())
    {
        for c in components {
            let name = c.get("componentModelEn").and_then(|v| v.as_str()).unwrap_or("").to_string();
            if name.is_empty() || name == query { continue; }

            let lcsc_code = c.get("componentCode").and_then(|v| v.as_str()).unwrap_or("").to_string();
            let manufacturer = c.get("componentBrandEn").and_then(|v| v.as_str()).unwrap_or("").to_string();
            let datasheet_url = fetch_lcsc_pdf_url(&client, &lcsc_code).await
                .unwrap_or_else(|| format!("https://datasheet.lcsc.com/lcsc/datasheet_{}.pdf", lcsc_code));

            if let Some((vds, id, rds, vceo, ic, hfe, ttype)) = parse_lcsc_params(c, &tr_type) {
                results.push(AnalogResult {
                    name, manufacturer, datasheet_url, source: "LCSC".to_string(),
                    vds, id, rds_on: rds, vceo, ic, hfe, tr_type: ttype,
                });
            }
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

    let client = Client::new();
    let search_keyword = if tr_type == "mosfet" { "MOSFET" } else { "Bipolar Transistor" };
    
    let body = serde_json::json!({
        "keyword": search_keyword,
        "currentPage": 1,
        "pageSize": 50
    });

    let res = client
        .post(JLCPCB_SEARCH_URL)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64)")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Ошибка поиска: {}", e))?;

    let json: Value = res.json().await.map_err(|e| format!("Ошибка парсинга: {}", e))?;
    let mut results = Vec::new();

    if let Some(components) = json
        .get("data")
        .and_then(|d| d.get("componentPageInfo"))
        .and_then(|c| c.get("list"))
        .and_then(|l| l.as_array())
    {
        for c in components {
            let name = c.get("componentModelEn").and_then(|v| v.as_str()).unwrap_or("").to_string();
            if name.is_empty() { continue; }

            let lcsc_code = c.get("componentCode").and_then(|v| v.as_str()).unwrap_or("").to_string();
            let manufacturer = c.get("componentBrandEn").and_then(|v| v.as_str()).unwrap_or("").to_string();
            let datasheet_url = fetch_lcsc_pdf_url(&client, &lcsc_code).await
                .unwrap_or_else(|| format!("https://datasheet.lcsc.com/lcsc/datasheet_{}.pdf", lcsc_code));

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
    }

    Ok(results)
}

async fn fetch_lcsc_pdf_url(client: &Client, lcsc_code: &str) -> Option<String> {
    let res = client
        .get(LCSC_DETAIL_URL)
        .query(&[("productCode", lcsc_code)])
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64)")
        .header("Accept", "application/json")
        .send()
        .await
        .ok()?;

    if !res.status().is_success() { return None; }

    let json: Value = res.json().await.ok()?;
    json.get("result")
        .and_then(|r| r.get("pdfUrl"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
}