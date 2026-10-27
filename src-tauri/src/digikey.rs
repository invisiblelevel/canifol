use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;
use std::fs;
use std::path::PathBuf;
use tauri::Manager;

// Ключи читаются из .env (или переменных окружения)
fn get_credentials() -> (String, String) {
    let id = env::var("DIGIKEY_CLIENT_ID").expect("DIGIKEY_CLIENT_ID must be set in .env");
    let secret = env::var("DIGIKEY_CLIENT_SECRET").expect("DIGIKEY_CLIENT_SECRET must be set in .env");
    (id, secret)
}

const TOKEN_URL: &str = "https://api.digikey.com/v1/oauth2/token";
const SEARCH_URL: &str = "https://api.digikey.com/products/v4/search/keyword";

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
}

#[derive(Serialize, Clone)]
pub struct DatasheetResult {
    pub name: String,
    pub manufacturer: String,
    pub datasheet_url: String,
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

// Универсальная функция для получения пути к кэшу
fn get_cache_dir(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let mut path = app.path().app_data_dir().map_err(|e| format!("Не удалось найти папку данных: {}", e))?;
    path.push("datasheets");
    if !path.exists() {
        fs::create_dir_all(&path).map_err(|e| format!("Не удалось создать папку кэша: {}", e))?;
    }
    Ok(path)
}

fn sanitize_filename(name: &str) -> String {
    name.replace(|c: char| !c.is_alphanumeric() && c != '-' && c != '_', "_")
}

#[tauri::command]
pub async fn download_pdf(app: tauri::AppHandle, url: String, filename: String) -> Result<String, String> {
    let cache_dir = get_cache_dir(&app)?;
    let safe_name = sanitize_filename(&filename);
    let file_path = cache_dir.join(format!("{}.pdf", safe_name));

    if file_path.exists() {
        return Ok(file_path.to_string_lossy().to_string());
    }

    let client = Client::new();
    let res = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64)")
        .send()
        .await
        .map_err(|e| format!("Ошибка скачивания: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("Сервер вернул ошибку: {}", res.status()));
    }

    let bytes = res.bytes().await.map_err(|e| format!("Ошибка чтения данных: {}", e))?;
    
    if bytes.len() < 4 || &bytes[0..4] != b"%PDF" {
        return Err("Скачанный файл не является PDF".to_string());
    }

    fs::write(&file_path, &bytes).map_err(|e| format!("Ошибка записи в кэш: {}", e))?;
    Ok(file_path.to_string_lossy().to_string())
}

#[tauri::command]
pub fn read_pdf(app: tauri::AppHandle, filename: String) -> Result<Vec<u8>, String> {
    let cache_dir = get_cache_dir(&app)?;
    let safe_name = sanitize_filename(&filename);
    let file_path = cache_dir.join(format!("{}.pdf", safe_name));
    fs::read(&file_path).map_err(|e| format!("Не удалось прочитать файл из кэша: {}", e))
}

async fn get_token() -> Result<String, String> {
    let (client_id, client_secret) = get_credentials();
    let client = Client::new();
    let params = [
        ("client_id", client_id.as_str()),
        ("client_secret", client_secret.as_str()),
        ("grant_type", "client_credentials"),
    ];

    let res = client.post(TOKEN_URL).form(&params).send().await
        .map_err(|e| format!("Ошибка запроса токена: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("DigiKey вернул ошибку при получении токена: {}", res.status()));
    }

    let token_res: TokenResponse = res.json().await.map_err(|e| format!("Ошибка парсинга токена: {}", e))?;
    Ok(token_res.access_token)
}

#[tauri::command]
pub async fn search_datasheets(query: String) -> Result<Vec<DatasheetResult>, String> {
    let (client_id, _) = get_credentials();
    let token = get_token().await?;
    let client = Client::new();

    let body = serde_json::json!({
        "Keywords": query,
        "RecordCount": 20,
        "RecordStartPosition": 0,
        "ExcludeMarketPlaceProducts": false
    });

    let res = client
        .post(SEARCH_URL)
        .header("Authorization", format!("Bearer {}", token))
        .header("X-DIGIKEY-Client-Id", client_id)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Ошибка поиска: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("DigiKey вернул ошибку поиска: {}", res.status()));
    }

    let json: Value = res.json().await.map_err(|e| format!("Ошибка парсинга: {}", e))?;
    let mut results = Vec::new();

    if let Some(products) = json.get("Products").and_then(|p| p.as_array()) {
        for product in products {
            let name = product.get("ManufacturerProductNumber").and_then(|v| v.as_str()).unwrap_or("Без названия").to_string();
            let manufacturer = product.get("Manufacturer").and_then(|m| m.get("Name")).and_then(|v| v.as_str()).unwrap_or("Неизвестен").to_string();
            let raw_url = product.get("DatasheetUrl").and_then(|v| v.as_str()).unwrap_or("").to_string();

            let mut datasheet_url = raw_url.clone();
            if raw_url.contains("gotoUrl=") {
                if let Some(idx) = raw_url.find("gotoUrl=") {
                    let mut extracted = &raw_url[idx + 8..];
                    if let Some(end) = extracted.find('&') { extracted = &extracted[..end]; }
                    datasheet_url = urlencoding::decode(extracted).unwrap_or_default().to_string();
                }
            }

            if !datasheet_url.is_empty() {
                results.push(DatasheetResult { name, manufacturer, datasheet_url });
            }
        }
    }
    Ok(results)
}

fn parse_digikey_params(product: &Value, _tr_type: &str) -> Option<(Option<f64>, Option<f64>, Option<f64>, Option<f64>, Option<f64>, Option<f64>, Option<String>)> {
    let params = product.get("Parameters")?.as_array()?;
    
    let mut vds = None;
    let mut id = None;
    let mut rds_on = None;
    let mut vceo = None;
    let mut ic = None;
    let mut hfe = None;
    let mut ttype = None;

    for p in params {
        let name = p.get("ParameterText")?.as_str()?;
        let val = p.get("ValueText")?.as_str()?;

        match name {
            "Drain to Source Voltage (Vdss)" => {
                vds = val.trim_end_matches(" V").trim().parse::<f64>().ok();
            }
            "Current - Continuous Drain (Id) @ 25°C" => {
                let clean = val.split(' ').next().unwrap_or("").trim_end_matches('A');
                id = clean.parse::<f64>().ok();
            }
            "Rds On (Max) @ Id, Vgs" => {
                let clean = val.split("mOhm").next().unwrap_or("").trim();
                rds_on = clean.parse::<f64>().ok();
            }
            "Collector - Emitter Voltage VCEO" | "Voltage - Collector Emitter Breakdown (Max)" => {
                vceo = val.trim_end_matches(" V").trim().parse::<f64>().ok();
            }
            "Current - Collector (Ic) (Max)" | "Current - Collector (Ic)" => {
                let clean = val.split(' ').next().unwrap_or("").trim_end_matches('A');
                ic = clean.parse::<f64>().ok();
            }
            "DC Current Gain (hFE) (Min) @ Ic, Vce" => {
                let clean = val.split(' ').next().unwrap_or("");
                hfe = clean.parse::<f64>().ok();
            }
            "FET Type" => {
                ttype = Some(if val.contains("N-Channel") { "N" } else { "P" }.to_string());
            }
            "Transistor Type" => {
                ttype = Some(if val.contains("NPN") { "NPN" } else { "PNP" }.to_string());
            }
            _ => {}
        }
    }

    Some((vds, id, rds_on, vceo, ic, hfe, ttype))
}

#[tauri::command]
pub async fn find_analogs_digikey(query: String, tr_type: String) -> Result<Vec<AnalogResult>, String> {
    let (client_id, _) = get_credentials();
    let token = get_token().await?;
    let client = Client::new();

    let body = serde_json::json!({
        "Keywords": query,
        "RecordCount": 1,
        "RecordStartPosition": 0
    });

    let res = client
        .post(SEARCH_URL)
        .header("Authorization", format!("Bearer {}", token))
        .header("X-DIGIKEY-Client-Id", client_id.clone()) // <-- ФИКС: clone()
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Ошибка поиска: {}", e))?;

    let json: Value = res.json().await.map_err(|e| format!("Ошибка парсинга: {}", e))?;
    let product = json.get("Products").and_then(|p| p.as_array()).and_then(|a| a.first())
        .ok_or("Оригинал не найден")?;

    let _params = parse_digikey_params(product, &tr_type).ok_or("Не удалось распарсить параметры")?;

    let search_keyword = if tr_type == "mosfet" { "MOSFET" } else { "Bipolar Transistor" };
    
    let body = serde_json::json!({
        "Keywords": search_keyword,
        "RecordCount": 20,
        "RecordStartPosition": 0
    });

    let res = client
        .post(SEARCH_URL)
        .header("Authorization", format!("Bearer {}", token))
        .header("X-DIGIKEY-Client-Id", client_id) // <-- второй раз, уже без clone
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Ошибка поиска аналогов: {}", e))?;

    let json: Value = res.json().await.map_err(|e| format!("Ошибка парсинга: {}", e))?;
    let mut results = Vec::new();

    if let Some(products) = json.get("Products").and_then(|p| p.as_array()) {
        for p in products {
            let name = p.get("ManufacturerProductNumber").and_then(|v| v.as_str()).unwrap_or("").to_string();
            if name.is_empty() || name == query { continue; }

            let manufacturer = p.get("Manufacturer").and_then(|m| m.get("Name")).and_then(|v| v.as_str()).unwrap_or("").to_string();
            let url = p.get("DatasheetUrl").and_then(|v| v.as_str()).unwrap_or("").to_string();
            
            if let Some((vds, id, rds, vceo, ic, hfe, ttype)) = parse_digikey_params(p, &tr_type) {
                results.push(AnalogResult {
                    name, manufacturer, datasheet_url: url, source: "DigiKey".to_string(),
                    vds, id, rds_on: rds, vceo, ic, hfe, tr_type: ttype,
                });
            }
        }
    }

    Ok(results)
}

#[tauri::command]
pub async fn find_by_params_digikey(params: serde_json::Value) -> Result<Vec<AnalogResult>, String> {
    let (client_id, _) = get_credentials();
    let tr_type = params.get("type").and_then(|v| v.as_str()).unwrap_or("mosfet");
    let vds_min = params.get("vds").and_then(|v| v.as_f64());
    let id_min = params.get("id").and_then(|v| v.as_f64());
    let rds_max = params.get("rdsOn").and_then(|v| v.as_f64());
    let vceo_min = params.get("vceo").and_then(|v| v.as_f64());
    let ic_min = params.get("ic").and_then(|v| v.as_f64());
    let hfe_min = params.get("hfe").and_then(|v| v.as_f64());

    let token = get_token().await?;
    let client = Client::new();

    let keyword = if tr_type == "mosfet" { "MOSFET" } else { "Bipolar Transistor" };
    let body = serde_json::json!({
        "Keywords": keyword,
        "RecordCount": 50,
        "RecordStartPosition": 0
    });

    let res = client
        .post(SEARCH_URL)
        .header("Authorization", format!("Bearer {}", token))
        .header("X-DIGIKEY-Client-Id", client_id)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Ошибка поиска: {}", e))?;

    let json: Value = res.json().await.map_err(|e| format!("Ошибка парсинга: {}", e))?;
    let mut results = Vec::new();

    if let Some(products) = json.get("Products").and_then(|p| p.as_array()) {
        for p in products {
            let name = p.get("ManufacturerProductNumber").and_then(|v| v.as_str()).unwrap_or("").to_string();
            if name.is_empty() { continue; }
            
            let manufacturer = p.get("Manufacturer").and_then(|m| m.get("Name")).and_then(|v| v.as_str()).unwrap_or("").to_string();
            let url = p.get("DatasheetUrl").and_then(|v| v.as_str()).unwrap_or("").to_string();
            
            if let Some((vds, id, rds, vceo, ic, hfe, ttype)) = parse_digikey_params(p, tr_type) {
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
                        name, manufacturer, datasheet_url: url, source: "DigiKey".to_string(),
                        vds, id, rds_on: rds, vceo, ic, hfe, tr_type: ttype,
                    });
                }
            }
        }
    }

    Ok(results)
}