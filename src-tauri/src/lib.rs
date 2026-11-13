mod digikey;
mod lcsc;
mod transistors;
mod boardview;
mod gencad_parser;
mod guides;
mod element14;
mod bilibili;
mod tvw_canifol;

use std::fs;
use std::io::Write;
use tauri::Manager;
use reqwest::Client;
use megalib::public;

#[tauri::command]
fn get_system_locale() -> String {
    sys_locale::get_locale().unwrap_or_else(|| "en".to_string())
}

#[tauri::command]
fn get_cache_size(app: tauri::AppHandle) -> Result<u64, String> {
    let cache_dir = app.path().app_data_dir()
        .map_err(|e| format!("No data dir: {}", e))?
        .join("datasheets");
    if !cache_dir.exists() { return Ok(0); }
    let mut total_size = 0;
    if let Ok(entries) = fs::read_dir(&cache_dir) {
        for entry in entries.flatten() {
            if let Ok(meta) = entry.metadata() {
                if meta.is_file() { total_size += meta.len(); }
            }
        }
    }
    Ok(total_size)
}

#[tauri::command]
fn clear_cache(app: tauri::AppHandle) -> Result<(), String> {
    let cache_dir = app.path().app_data_dir()
        .map_err(|e| format!("No data dir: {}", e))?
        .join("datasheets");
    if cache_dir.exists() { fs::remove_dir_all(&cache_dir).map_err(|e| e.to_string())?; }
    Ok(())
}

#[tauri::command]
fn get_favorites_size(app: tauri::AppHandle) -> Result<u64, String> {
    let dir = app.path().app_data_dir()
        .map_err(|e| format!("No data dir: {}", e))?
        .join("favorites_pdfs");
    if !dir.exists() { return Ok(0); }
    let mut total_size = 0;
    if let Ok(entries) = fs::read_dir(&dir) {
        for entry in entries.flatten() {
            if let Ok(meta) = entry.metadata() {
                if meta.is_file() { total_size += meta.len(); }
            }
        }
    }
    Ok(total_size)
}

#[tauri::command]
fn clear_favorites(app: tauri::AppHandle) -> Result<(), String> {
    let dir = app.path().app_data_dir()
        .map_err(|e| format!("No data dir: {}", e))?
        .join("favorites_pdfs");
    if dir.exists() { fs::remove_dir_all(&dir).map_err(|e| e.to_string())?; }
    Ok(())
}

#[tauri::command]
fn delete_cached_file(app: tauri::AppHandle, filename: String) -> Result<(), String> {
    let cache_dir = app.path().app_data_dir()
        .map_err(|e| format!("No data dir: {}", e))?
        .join("datasheets");
    let safe_filename = filename.replace(|c: char| !c.is_alphanumeric() && c != '.' && c != '-' && c != '_', "_");
    let file_path = cache_dir.join(&safe_filename);
    if file_path.exists() {
        fs::remove_file(&file_path).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
async fn download_gdrive_file(app: tauri::AppHandle, file_id: String, filename: String) -> Result<String, String> {
    let client = Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .cookie_store(true)
        .build()
        .map_err(|e| e.to_string())?;

    let cache_dir = app.path().app_data_dir()
        .map_err(|e| format!("No data dir: {}", e))?
        .join("datasheets");
    if !cache_dir.exists() {
        fs::create_dir_all(&cache_dir).map_err(|e| e.to_string())?;
    }

    let safe_filename = filename.replace(|c: char| !c.is_alphanumeric() && c != '.' && c != '-' && c != '_', "_");
    let dest_path = cache_dir.join(&safe_filename);

    let is_valid_file = |b: &[u8]| -> bool {
        if b.len() < 100 { return false; }
        let head = String::from_utf8_lossy(&b[..std::cmp::min(200, b.len())]);
        if head.contains("<html") || (head.contains("google") && head.contains("accounts")) { return false; }
        true
    };

    if dest_path.exists() {
        if let Ok(bytes) = fs::read(&dest_path) {
            if is_valid_file(&bytes) {
                return Ok(dest_path.to_string_lossy().to_string());
            }
        }
        let _ = fs::remove_file(&dest_path);
    }

    let url = format!("https://drive.google.com/uc?export=download&id={}", file_id);
    let res = client.get(&url).send().await.map_err(|e| format!("Ошибка запроса: {}", e))?;
    let cookies = res.cookies().collect::<Vec<_>>();
    let mut confirm_token = None;
    for cookie in cookies {
        if cookie.name().starts_with("download_warning") {
            confirm_token = Some(cookie.value().to_string());
            break;
        }
    }

    let bytes_res = if let Some(token) = confirm_token {
        let confirm_url = format!("https://drive.google.com/uc?export=download&confirm={}&id={}", token, file_id);
        client.get(&confirm_url).send().await.map_err(|e| format!("Ошибка подтверждения: {}", e))?
    } else {
        res
    };

    let final_bytes = bytes_res.bytes().await.map_err(|e| format!("Ошибка чтения байт: {}", e))?.to_vec();
    if !is_valid_file(&final_bytes) {
        return Err("Google Drive заблокировал скачивание.".to_string());
    }
    fs::write(&dest_path, &final_bytes).map_err(|e| format!("Ошибка записи файла: {}", e))?;
    Ok(dest_path.to_string_lossy().to_string())
}

#[tauri::command]
async fn download_mega_file(app: tauri::AppHandle, url: String, filename: String) -> Result<String, String> {
    let cache_dir = app.path().app_data_dir()
        .map_err(|e| format!("No data dir: {}", e))?
        .join("datasheets");
    if !cache_dir.exists() {
        fs::create_dir_all(&cache_dir).map_err(|e| e.to_string())?;
    }
    let safe_filename = filename.replace(|c: char| !c.is_alphanumeric() && c != '.' && c != '-' && c != '_', "_");
    let dest_path = cache_dir.join(&safe_filename);
    if dest_path.exists() {
        return Ok(dest_path.to_string_lossy().to_string());
    }
    let mut file = std::fs::File::create(&dest_path).map_err(|e| format!("Create file error: {}", e))?;
    public::download_public_file(&url, &mut file).await
        .map_err(|e| format!("MEGA download error: {}", e))?;
    Ok(dest_path.to_string_lossy().to_string())
}

// ==========================================
// НОВАЯ КОМАНДА: скачать все избранные PDF
// ==========================================
#[derive(serde::Serialize)]
pub struct DownloadFavoritesResult {
    downloaded: usize,
    skipped: usize,
    failed: usize,
}

#[tauri::command]
async fn download_all_favorites(
    app: tauri::AppHandle,
    urls: Vec<String>,
) -> Result<DownloadFavoritesResult, String> {
    let dir = app.path().app_data_dir()
        .map_err(|e| format!("No data dir: {}", e))?
        .join("favorites_pdfs");
    if !dir.exists() {
        fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    }

    let client = Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .timeout(std::time::Duration::from_secs(60))
        .build()
        .map_err(|e| e.to_string())?;

    let mut downloaded = 0usize;
    let mut skipped = 0usize;
    let mut failed = 0usize;

    for url in &urls {
        if url.is_empty() { continue; }

        // Имя файла: последняя часть URL, убираем query
        let raw_name = url.rsplit('/').next().unwrap_or("datasheet.pdf");
        let clean_name = raw_name.split('?').next().unwrap_or("datasheet.pdf");
        let mut filename = clean_name.to_string();
        if !filename.to_lowercase().ends_with(".pdf") {
            filename.push_str(".pdf");
        }
        let safe_filename = filename.replace(|c: char| !c.is_alphanumeric() && c != '.' && c != '-' && c != '_', "_");
        let dest = dir.join(&safe_filename);

        // Уже есть валидный PDF — пропускаем
        if dest.exists() {
            if let Ok(meta) = fs::metadata(&dest) {
                if meta.len() > 1024 {
                    skipped += 1;
                    continue;
                }
            }
        }

        // Качаем
        match client.get(url).send().await {
            Ok(resp) => {
                if !resp.status().is_success() {
                    failed += 1;
                    continue;
                }
                match resp.bytes().await {
                    Ok(bytes) => {
                        if bytes.len() > 1024 && bytes.starts_with(b"%PDF") {
                            match fs::File::create(&dest) {
                                Ok(mut f) => {
                                    if f.write_all(&bytes).is_ok() {
                                        downloaded += 1;
                                    } else {
                                        failed += 1;
                                    }
                                }
                                Err(_) => failed += 1,
                            }
                        } else {
                            failed += 1;
                        }
                    }
                    Err(_) => failed += 1,
                }
            }
            Err(_) => failed += 1,
        }
    }

    Ok(DownloadFavoritesResult { downloaded, skipped, failed })
}

#[tauri::command]
fn read_text_file(path: String) -> Result<String, String> {
    fs::read_to_string(&path).map_err(|e| e.to_string())
}

#[tauri::command]
fn read_binary_file(path: String) -> Result<Vec<u8>, String> {
    fs::read(&path).map_err(|e| e.to_string())
}

#[tauri::command]
async fn open_pdf(path: String) -> Result<(), String> {
    tauri_plugin_opener::open_path(&path, None::<&str>).map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init());

    #[cfg(target_os = "android")]
    {
        builder = builder.plugin(tauri_plugin_android_fs::init());
    }

    builder
        .invoke_handler(tauri::generate_handler![
            digikey::search_datasheets,
            digikey::download_pdf,
            digikey::read_pdf,
            digikey::read_favorite_pdf,
            digikey::find_analogs_digikey,
            digikey::find_by_params_digikey,
            lcsc::search_lcsc,
            lcsc::find_analogs_lcsc,
            lcsc::find_by_params_lcsc,
            element14::search_element14,
            transistors::search_soviet_analog,
            get_system_locale,
            get_cache_size,
            clear_cache,
            get_favorites_size,
            clear_favorites,
            delete_cached_file,
            download_gdrive_file,
            download_mega_file,
            download_all_favorites,
            read_text_file,
            read_binary_file,
            open_pdf,
            bilibili::fetch_bilibili_videos,
            boardview::read_boardview,
            guides::fetch_guides,
            guides::get_guides_cache_size,
            guides::clear_guides_cache,
            #[cfg(target_os = "android")]
            boardview::read_boardview_uri
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}