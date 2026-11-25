use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use tauri::ipc::Response;
use tauri::Manager;

const TOKEN_URL: &str = "https://api.digikey.com/v1/oauth2/token";
const SEARCH_URL: &str = "https://api.digikey.com/products/v4/search/keyword";

fn get_credentials() -> Result<(String, String), String> {
    dotenvy::dotenv().ok();
    if let (Ok(id), Ok(secret)) = (
        std::env::var("DIGIKEY_CLIENT_ID"),
        std::env::var("DIGIKEY_CLIENT_SECRET"),
    ) {
        if !id.is_empty() && !secret.is_empty() {
            return Ok((id, secret));
        }
    }

    let id = option_env!("DIGIKEY_CLIENT_ID")
        .ok_or("DIGIKEY_CLIENT_ID is not set. Please set it in .env (dev) or GitHub Secrets (CI)")?;
    let secret = option_env!("DIGIKEY_CLIENT_SECRET")
        .ok_or("DIGIKEY_CLIENT_SECRET is not set. Please set it in .env (dev) or GitHub Secrets (CI)")?;

    Ok((id.to_string(), secret.to_string()))
}

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

#[tauri::command]
pub async fn save_pdf_copy(app: tauri::AppHandle, dest: String, filename: String) -> Result<(), String> {
    let path = PathBuf::from(dest);
    crate::net::validate_destination(&path)?;
    let bytes = tauri::async_runtime::spawn_blocking(move || read_pdf_bytes(&app, &filename))
        .await
        .map_err(|e| format!("Read task failed: {}", e))??;
    tauri::async_runtime::spawn_blocking(move || crate::net::write_atomic(&path, &bytes))
        .await
        .map_err(|e| format!("Save task failed: {}", e))?
}

fn get_cache_dir(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let mut path = app.path().app_data_dir().map_err(|e| format!("Failed to find data dir: {}", e))?;
    path.push("datasheets");
    if !path.exists() {
        fs::create_dir_all(&path).map_err(|e| format!("Failed to create cache dir: {}", e))?;
    }
    Ok(path)
}

fn get_favorites_dir(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let mut path = app.path().app_data_dir().map_err(|e| format!("Failed to find data dir: {}", e))?;
    path.push("favorites_pdfs");
    if !path.exists() {
        fs::create_dir_all(&path).map_err(|e| format!("Failed to create favorites dir: {}", e))?;
    }
    Ok(path)
}

fn sanitize_filename(name: &str) -> String {
    name.replace(|c: char| !c.is_alphanumeric() && c != '-' && c != '_', "_")
}

/// Cache key for a PDF.  The UI sometimes passes `savedFilename` with a `.pdf`
/// suffix and sometimes a bare part number; both must resolve to the same file.
/// (`sanitize_filename` turns dots into underscores, so "LM7805.pdf" used to
/// become "LM7805_pdf.pdf" and the offline copy was never found.)
fn cache_stem(filename: &str) -> String {
    let trimmed = filename.trim();
    let without_ext = trimmed
        .strip_suffix(".pdf")
        .or_else(|| trimmed.strip_suffix(".PDF"))
        .unwrap_or(trimmed);
    sanitize_filename(without_ext)
}

/// Some DigiKey answers contain protocol-relative or site-relative datasheet
/// URLs.  Passing those to `reqwest` fails with a "builder error".
fn normalize_datasheet_url(url: &str) -> String {
    let url = url.trim();
    if url.starts_with("//") {
        format!("https:{}", url)
    } else if url.starts_with('/') {
        format!("https://www.digikey.com{}", url)
    } else {
        url.to_string()
    }
}

/// Returns the cached path if it holds a structurally valid PDF, otherwise
/// removes the corrupt file so the caller can download it again.
fn valid_cached_pdf(path: &Path) -> bool {
    if !path.exists() {
        return false;
    }
    if crate::net::validate_file_on_disk(path, "cached.pdf").is_ok() {
        return true;
    }
    let _ = fs::remove_file(path);
    false
}

#[tauri::command]
pub async fn download_pdf(
    app: tauri::AppHandle,
    url: String,
    filename: String,
    save_to_favorites: Option<bool>,
) -> Result<String, String> {
    let save_to_favorites = save_to_favorites.unwrap_or(false);
    let safe_name = cache_stem(&filename);

    if url.is_empty() {
        return Err("No datasheet available for this component.".to_string());
    }
    if !crate::net::is_public_http_url(&url) {
        return Err(format!("Invalid datasheet URL: \"{}\"", url));
    }

    if save_to_favorites {
        let target_dir = get_favorites_dir(&app)?;
        let file_path = target_dir.join(format!("{}.pdf", safe_name));
        if valid_cached_pdf(&file_path) {
            return Ok(file_path.to_string_lossy().to_string());
        }
        download_to_path(&url, &file_path).await?;
        return Ok(file_path.to_string_lossy().to_string());
    }

    let fav_dir = get_favorites_dir(&app)?;
    let fav_path = fav_dir.join(format!("{}.pdf", safe_name));
    if valid_cached_pdf(&fav_path) {
        return Ok(fav_path.to_string_lossy().to_string());
    }

    let cache_dir = get_cache_dir(&app)?;
    let cache_path = cache_dir.join(format!("{}.pdf", safe_name));
    if valid_cached_pdf(&cache_path) {
        return Ok(cache_path.to_string_lossy().to_string());
    }

    download_to_path(&url, &cache_path).await?;
    Ok(cache_path.to_string_lossy().to_string())
}

async fn download_to_path(url: &str, file_path: &std::path::Path) -> Result<(), String> {
    let client = crate::net::http_client()?;

    // Определяем Referer по домену
    let referer = if url.contains("lcsc.com") || url.contains("wmsc.lcsc.com") {
        "https://www.lcsc.com/"
    } else if url.contains("farnell.com") || url.contains("element14.com") {
        "https://www.farnell.com/"
    } else if url.contains("tme.eu") || url.contains("tme.com") {
        "https://www.tme.eu/"
    } else {
        "https://www.digikey.com/"
    };

    let res = client
        .get(url)
        .header("Referer", referer)
        .header("Accept", "application/pdf, */*")
        .send()
        .await
        .map_err(|e| format!("Download error: {}", e))?;

    let status = res.status();
    if !status.is_success() {
        let body = res.bytes().await.map(|b| b.to_vec()).unwrap_or_default();
        return Err(format!(
            "Server returned HTTP {} ({})",
            status.as_u16(),
            crate::net::describe_payload(&body)
        ));
    }

    let body = res
        .bytes()
        .await
        .map_err(|e| format!("Read error: {}", e))?
        .to_vec();
    crate::net::validate_pdf_bytes(&body)
        .map_err(|e| format!("The datasheet is not a valid PDF: {}", e))?;
    crate::net::write_atomic(file_path, &body)?;
    Ok(())
}

#[tauri::command]
pub async fn read_pdf(app: tauri::AppHandle, filename: String) -> Result<Response, String> {
    let bytes = tauri::async_runtime::spawn_blocking(move || read_pdf_bytes(&app, &filename))
        .await
        .map_err(|e| format!("Read task failed: {}", e))??;
    Ok(Response::new(bytes))
}

fn read_pdf_bytes(app: &tauri::AppHandle, filename: &str) -> Result<Vec<u8>, String> {
    let safe_name = cache_stem(filename);

    if let Ok(fav_dir) = get_favorites_dir(app) {
        let path = fav_dir.join(format!("{}.pdf", safe_name));
        if path.exists() {
            if crate::net::validate_file_on_disk(&path, "cached.pdf").is_ok() {
                return fs::read(&path)
                    .map_err(|e| format!("Failed to read from favorites: {}", e));
            }
            let _ = fs::remove_file(&path);
        }
    }

    let cache_dir = get_cache_dir(app)?;
    let file_path = cache_dir.join(format!("{}.pdf", safe_name));
    if file_path.exists() {
        if crate::net::validate_file_on_disk(&file_path, "cached.pdf").is_ok() {
            return fs::read(&file_path).map_err(|e| format!("Failed to read from cache: {}", e));
        }
        let _ = fs::remove_file(&file_path);
        return Err("The cached PDF was corrupt and has been removed. Please open it again.".to_string());
    }
    Err("PDF is not downloaded yet.".to_string())
}

#[tauri::command]
pub async fn read_favorite_pdf(app: tauri::AppHandle, filename: String) -> Result<Response, String> {
    let bytes = tauri::async_runtime::spawn_blocking(move || read_favorite_pdf_bytes(&app, &filename))
        .await
        .map_err(|e| format!("Read task failed: {}", e))??;
    Ok(Response::new(bytes))
}

fn read_favorite_pdf_bytes(app: &tauri::AppHandle, filename: &str) -> Result<Vec<u8>, String> {
    let fav_dir = get_favorites_dir(app)?;
    let safe_name = cache_stem(filename);
    let file_path = fav_dir.join(format!("{}.pdf", safe_name));
    if !file_path.exists() {
        return Err("Not in the offline favorites.".to_string());
    }
    if crate::net::validate_file_on_disk(&file_path, "cached.pdf").is_err() {
        let _ = fs::remove_file(&file_path);
        return Err("The offline PDF was corrupt and has been removed.".to_string());
    }
    fs::read(&file_path).map_err(|e| format!("Failed to read from favorites: {}", e))
}

async fn get_token() -> Result<String, String> {
    let (client_id, client_secret) = get_credentials()?;
    let client = crate::net::api_client()?;
    let params = [
        ("client_id", client_id.as_str()),
        ("client_secret", client_secret.as_str()),
        ("grant_type", "client_credentials"),
    ];

    let res = client.post(TOKEN_URL).form(&params).send().await
        .map_err(|e| format!("Token request error: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("DigiKey token error: {}", res.status()));
    }

    let token_res: TokenResponse = res.json().await.map_err(|e| format!("Token parse error: {}", e))?;
    Ok(token_res.access_token)
}

#[tauri::command]
pub async fn search_datasheets(query: String) -> Result<Vec<DatasheetResult>, String> {
    let (client_id, _) = get_credentials()?;
    let token = get_token().await?;
    let client = crate::net::api_client()?;

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
        .map_err(|e| format!("Search error: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("DigiKey search error: {}", res.status()));
    }

    let json: Value = res.json().await.map_err(|e| format!("Parse error: {}", e))?;
    let mut results = Vec::new();

    if let Some(products) = json.get("Products").and_then(|p| p.as_array()) {
        for product in products {
            let name = product.get("ManufacturerProductNumber").and_then(|v| v.as_str()).unwrap_or("Untitled").to_string();
            let manufacturer = product.get("Manufacturer").and_then(|m| m.get("Name")).and_then(|v| v.as_str()).unwrap_or("Unknown").to_string();
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
                datasheet_url = normalize_datasheet_url(&datasheet_url);
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
        let (Some(name), Some(val)) = (
            p.get("ParameterText").and_then(|v| v.as_str()),
            p.get("ValueText").and_then(|v| v.as_str()),
        ) else {
            // A parameter without a text/value must not abort parsing of the
            // whole product (previously `?` returned None for the entire item).
            continue;
        };

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
    let (client_id, _) = get_credentials()?;
    let token = get_token().await?;
    let client = crate::net::api_client()?;

    let body = serde_json::json!({
        "Keywords": query,
        "RecordCount": 1,
        "RecordStartPosition": 0
    });

    let res = client
        .post(SEARCH_URL)
        .header("Authorization", format!("Bearer {}", token))
        .header("X-DIGIKEY-Client-Id", client_id.clone())
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Search error: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("DigiKey search error: {}", res.status()));
    }
    let json: Value = res.json().await.map_err(|e| format!("Parse error: {}", e))?;
    let product = json.get("Products").and_then(|p| p.as_array()).and_then(|a| a.first())
        .ok_or("Original component not found")?;

    let _params = parse_digikey_params(product, &tr_type).ok_or("Failed to parse parameters")?;

    let search_keyword = if tr_type == "mosfet" { "MOSFET" } else { "Bipolar Transistor" };

    let body = serde_json::json!({
        "Keywords": search_keyword,
        "RecordCount": 20,
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
        .map_err(|e| format!("Analog search error: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("DigiKey analog search error: {}", res.status()));
    }
    let json: Value = res.json().await.map_err(|e| format!("Parse error: {}", e))?;
    let mut results = Vec::new();

    if let Some(products) = json.get("Products").and_then(|p| p.as_array()) {
        for p in products {
            let name = p.get("ManufacturerProductNumber").and_then(|v| v.as_str()).unwrap_or("").to_string();
            if name.is_empty() || name == query { continue; }

            let manufacturer = p.get("Manufacturer").and_then(|m| m.get("Name")).and_then(|v| v.as_str()).unwrap_or("").to_string();
            let url = normalize_datasheet_url(p.get("DatasheetUrl").and_then(|v| v.as_str()).unwrap_or(""));

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
    let (client_id, _) = get_credentials()?;
    let tr_type = params.get("type").and_then(|v| v.as_str()).unwrap_or("mosfet");
    let vds_min = params.get("vds").and_then(|v| v.as_f64());
    let id_min = params.get("id").and_then(|v| v.as_f64());
    let rds_max = params.get("rdsOn").and_then(|v| v.as_f64());
    let vceo_min = params.get("vceo").and_then(|v| v.as_f64());
    let ic_min = params.get("ic").and_then(|v| v.as_f64());
    let hfe_min = params.get("hfe").and_then(|v| v.as_f64());

    let token = get_token().await?;
    let client = crate::net::api_client()?;

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
        .map_err(|e| format!("Search error: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("DigiKey search error: {}", res.status()));
    }
    let json: Value = res.json().await.map_err(|e| format!("Parse error: {}", e))?;
    let mut results = Vec::new();

    if let Some(products) = json.get("Products").and_then(|p| p.as_array()) {
        for p in products {
            let name = p.get("ManufacturerProductNumber").and_then(|v| v.as_str()).unwrap_or("").to_string();
            if name.is_empty() { continue; }

            let manufacturer = p.get("Manufacturer").and_then(|m| m.get("Name")).and_then(|v| v.as_str()).unwrap_or("").to_string();
            let url = normalize_datasheet_url(p.get("DatasheetUrl").and_then(|v| v.as_str()).unwrap_or(""));

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