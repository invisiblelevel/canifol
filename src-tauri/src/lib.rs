mod digikey;
mod lcsc;
mod transistors;
mod boardview;
mod gencad_parser;

use std::fs;
use tauri::Manager;

#[tauri::command]
fn get_system_locale() -> String {
    sys_locale::get_locale().unwrap_or_else(|| "en".to_string())
}

#[tauri::command]
fn get_cache_size(app: tauri::AppHandle) -> Result<u64, String> {
    // Считаем ТОЛЬКО обычный кэш (datasheets), не трогая избранное
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
            transistors::search_soviet_analog,
            get_system_locale,
            get_cache_size,
            clear_cache,
            get_favorites_size,
            clear_favorites,
            boardview::read_boardview,
            #[cfg(target_os = "android")]
            boardview::read_boardview_uri
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}