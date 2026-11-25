mod digikey;
mod lcsc;
mod transistors;
mod boardview;
mod gencad_parser;
mod guides;
mod element14;
mod bilibili;
mod tvw_canifol;
mod tme;
mod net;

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use tauri::ipc::Response;
use tauri::{Emitter, Manager};
use serde::Serialize;

/// Extensions used by the offline knowledge base (PDF schemes + BoardView files).
/// Only these are counted / deleted when the user points the database at a custom
/// folder, so `Clear` can never wipe unrelated files that happen to live there.
const DB_EXTENSIONS: &[&str] = &[
    "pdf", "fz", "cad", "bdv", "brd", "pcb", "tvw", "asc",
];

#[tauri::command]
fn get_system_locale() -> String {
    sys_locale::get_locale().unwrap_or_else(|| "en".to_string())
}

fn app_subdir(app: &tauri::AppHandle, name: &str) -> Result<PathBuf, String> {
    let dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("No data dir: {}", e))?
        .join(name);
    if !dir.exists() {
        fs::create_dir_all(&dir).map_err(|e| format!("Cannot create {}: {}", name, e))?;
    }
    Ok(dir)
}

fn sanitize_filename(name: &str) -> String {
    let mut safe: String = name
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '.' || c == '-' || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect();
    // Never allow "." / ".." or an empty name (path traversal / silent overwrite).
    while safe.starts_with('.') {
        safe.remove(0);
    }
    if safe.is_empty() {
        safe.push_str("file");
    }
    safe
}

fn has_db_extension(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| DB_EXTENSIONS.iter().any(|known| known.eq_ignore_ascii_case(ext)))
        .unwrap_or(false)
}

/// Sums file sizes in a directory.  With `only_db_files` the count is limited to
/// knowledge-base extensions (used for a user-chosen folder).
fn dir_size(dir: &Path, only_db_files: bool) -> u64 {
    if !dir.exists() {
        return 0;
    }
    let mut total = 0u64;
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Ok(meta) = entry.metadata() {
                if meta.is_file() && (!only_db_files || has_db_extension(&path)) {
                    total += meta.len();
                }
            }
        }
    }
    total
}

fn resolve_db_dir(app: &tauri::AppHandle, dest_dir: &str) -> Result<PathBuf, String> {
    let dir = if dest_dir.trim().is_empty() {
        app.path()
            .app_data_dir()
            .map_err(|e| format!("No data dir: {}", e))?
            .join("database")
    } else {
        PathBuf::from(dest_dir.trim())
    };
    if !dir.exists() {
        fs::create_dir_all(&dir)
            .map_err(|e| format!("Cannot create database folder \"{}\": {}", dir.display(), e))?;
    }
    Ok(dir)
}

/// Downloads a Google Drive file into `dir`, reusing an existing **valid** copy.
/// Invalid files left behind by older versions (HTML error pages saved as PDFs)
/// are deleted and downloaded again — otherwise a single bad download would be
/// cached forever.
async fn download_into_dir(
    dir: &Path,
    file_id: &str,
    filename: &str,
    force: bool,
) -> Result<String, String> {
    let safe_name = sanitize_filename(filename);
    let dest_path = dir.join(&safe_name);

    if dest_path.exists() {
        match net::validate_file_on_disk(&dest_path, &safe_name) {
            Ok(()) if !force => return Ok(dest_path.to_string_lossy().to_string()),
            Ok(()) => {
                // Refresh requested: keep the old copy until the new one is
                // validated and atomically replaces it.
            }
            Err(_) => {
                // Cached copy is corrupt/truncated — drop it and retry.
                let _ = fs::remove_file(&dest_path);
            }
        }
    }

    let bytes = net::gdrive_download_with_retry(file_id).await?;
    net::validate_download(&bytes, &safe_name)
        .map_err(|e| format!("Download of \"{}\" failed: {}", filename, e))?;
    net::write_atomic(&dest_path, &bytes)?;
    Ok(dest_path.to_string_lossy().to_string())
}

#[tauri::command]
fn get_cache_size(app: tauri::AppHandle) -> Result<u64, String> {
    Ok(dir_size(&app_subdir(&app, "datasheets")?, false))
}

#[tauri::command]
fn clear_cache(app: tauri::AppHandle) -> Result<(), String> {
    let cache_dir = app_subdir(&app, "datasheets")?;
    if cache_dir.exists() {
        fs::remove_dir_all(&cache_dir).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn get_favorites_size(app: tauri::AppHandle) -> Result<u64, String> {
    let dir = app.path()
        .app_data_dir()
        .map_err(|e| format!("No data dir: {}", e))?
        .join("favorites_pdfs");
    Ok(dir_size(&dir, false))
}

#[tauri::command]
fn clear_favorites(app: tauri::AppHandle) -> Result<(), String> {
    let dir = app.path()
        .app_data_dir()
        .map_err(|e| format!("No data dir: {}", e))?
        .join("favorites_pdfs");
    if dir.exists() {
        fs::remove_dir_all(&dir).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn delete_cached_file(app: tauri::AppHandle, filename: String) -> Result<(), String> {
    let cache_dir = app_subdir(&app, "datasheets")?;
    let file_path = cache_dir.join(sanitize_filename(&filename));
    if file_path.is_file() {
        fs::remove_file(&file_path).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
async fn download_gdrive_file(
    app: tauri::AppHandle,
    file_id: String,
    filename: String,
    force: Option<bool>,
) -> Result<String, String> {
    let cache_dir = app_subdir(&app, "datasheets")?;
    download_into_dir(&cache_dir, &file_id, &filename, force.unwrap_or(false)).await
}

#[derive(Clone, Serialize)]
pub struct DbProgress {
    pub current: usize,
    pub total: usize,
    pub bytes_downloaded: u64,
    pub total_bytes: u64,
    pub current_file: String,
    pub speed_bps: f64,
    pub eta_secs: f64,
}

#[tauri::command]
async fn download_database_file(
    app: tauri::AppHandle,
    file_id: String,
    filename: String,
    dest_dir: String,
) -> Result<String, String> {
    let base_dir = resolve_db_dir(&app, &dest_dir)?;
    download_into_dir(&base_dir, &file_id, &filename, false).await
}

#[derive(Clone, Serialize)]
pub struct DatabaseDownloadResult {
    downloaded: usize,
    skipped: usize,
    failed: usize,
    /// First failure reason, so the UI can explain *why* files failed instead of
    /// silently reporting a number.
    last_error: Option<String>,
}

#[tauri::command]
async fn download_database_batch(
    app: tauri::AppHandle,
    files: Vec<(String, String)>,
    dest_dir: String,
) -> Result<DatabaseDownloadResult, String> {
    let base_dir = resolve_db_dir(&app, &dest_dir)?;
    let total = files.len();

    let mut downloaded = 0usize;
    let mut skipped = 0usize;
    let mut failed = 0usize;
    let mut last_error: Option<String> = None;

    // Pre-pass: validate what is already on disk (and delete corrupt leftovers).
    let mut known_bytes: u64 = 0;
    let mut known_count: usize = 0;
    for (_, name) in &files {
        let safe = sanitize_filename(name);
        let path = base_dir.join(&safe);
        if !path.exists() {
            continue;
        }
        if net::validate_file_on_disk(&path, &safe).is_ok() {
            if let Ok(meta) = fs::metadata(&path) {
                known_bytes += meta.len();
                known_count += 1;
            }
            skipped += 1;
        } else {
            let _ = fs::remove_file(&path);
        }
    }
    let missing = total.saturating_sub(skipped);
    let average_size = if known_count > 0 {
        known_bytes / known_count as u64
    } else {
        1_000_000
    };
    let estimated_total = known_bytes.saturating_add(average_size.saturating_mul(missing as u64));

    let mut bytes_downloaded = known_bytes;
    let start_time = std::time::Instant::now();

    for (idx, (id, name)) in files.iter().enumerate() {
        let safe = sanitize_filename(name);
        let dest = base_dir.join(&safe);

        if !dest.exists() {
            match download_into_dir(&base_dir, id, name, false).await {
                Ok(_) => {
                    downloaded += 1;
                    if let Ok(meta) = fs::metadata(&dest) {
                        bytes_downloaded += meta.len();
                    }
                }
                Err(err) => {
                    failed += 1;
                    if last_error.is_none() {
                        last_error = Some(err);
                    }
                }
            }
        }

        let elapsed = start_time.elapsed().as_secs_f64();
        let speed = if elapsed > 0.0 {
            bytes_downloaded as f64 / elapsed
        } else {
            0.0
        };
        let remaining_bytes = estimated_total.saturating_sub(bytes_downloaded);
        let eta = if speed > 0.0 {
            remaining_bytes as f64 / speed
        } else {
            0.0
        };

        let _ = app.emit(
            "db-download-progress",
            DbProgress {
                current: idx + 1,
                total,
                bytes_downloaded,
                total_bytes: estimated_total.max(bytes_downloaded),
                current_file: name.clone(),
                speed_bps: speed,
                eta_secs: eta,
            },
        );
    }

    Ok(DatabaseDownloadResult {
        downloaded,
        skipped,
        failed,
        last_error,
    })
}

#[tauri::command]
fn get_database_size(app: tauri::AppHandle, path: Option<String>) -> Result<u64, String> {
    let custom = path.unwrap_or_default();
    if custom.trim().is_empty() {
        let dir = app.path()
            .app_data_dir()
            .map_err(|e| format!("No data dir: {}", e))?
            .join("database");
        Ok(dir_size(&dir, false))
    } else {
        Ok(dir_size(Path::new(custom.trim()), true))
    }
}

#[tauri::command]
fn clear_database(app: tauri::AppHandle, path: Option<String>) -> Result<(), String> {
    let custom = path.unwrap_or_default();
    if custom.trim().is_empty() {
        let dir = app.path()
            .app_data_dir()
            .map_err(|e| format!("No data dir: {}", e))?
            .join("database");
        if dir.exists() {
            fs::remove_dir_all(&dir).map_err(|e| e.to_string())?;
        }
        return Ok(());
    }

    // Custom folder: remove only knowledge-base files and our *.part leftovers.
    let dir = Path::new(custom.trim());
    if !dir.is_dir() {
        return Ok(());
    }
    let entries = fs::read_dir(dir).map_err(|e| e.to_string())?;
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let is_partial = path
            .file_name()
            .and_then(|name| name.to_str())
            .map(|name| name.ends_with(".part"))
            .unwrap_or(false);
        if has_db_extension(&path) || is_partial {
            fs::remove_file(&path).map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

#[tauri::command]
async fn download_mega_file(
    app: tauri::AppHandle,
    url: String,
    filename: String,
) -> Result<String, String> {
    if !net::is_public_http_url(&url) {
        return Err(format!("Refusing to download from non-http(s) URL: \"{}\"", url));
    }
    let cache_dir = app_subdir(&app, "datasheets")?;
    let safe_name = sanitize_filename(&filename);
    let dest_path = cache_dir.join(&safe_name);

    if dest_path.exists() && net::validate_file_on_disk(&dest_path, &safe_name).is_ok() {
        return Ok(dest_path.to_string_lossy().to_string());
    }
    let _ = fs::remove_file(&dest_path);

    let unique = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let tmp_path = cache_dir.join(format!(".{}.{}.{}.mega.part", safe_name, std::process::id(), unique));

    let result = async {
        let mut file = std::fs::File::create(&tmp_path)
            .map_err(|e| format!("Cannot create temporary file: {}", e))?;
        let download = megalib::public::download_public_file(&url, &mut file).await;
        let _ = file.flush();
        download.map_err(|e| format!("MEGA download error: {}", e))
    }
    .await;

    if let Err(err) = result {
        let _ = fs::remove_file(&tmp_path);
        return Err(err);
    }

    let bytes = fs::read(&tmp_path).map_err(|e| {
        let _ = fs::remove_file(&tmp_path);
        format!("Cannot read downloaded file: {}", e)
    })?;
    let _ = fs::remove_file(&tmp_path);
    net::validate_download(&bytes, &safe_name)?;
    net::write_atomic(&dest_path, &bytes)?;
    Ok(dest_path.to_string_lossy().to_string())
}

#[derive(serde::Serialize)]
pub struct DownloadFavoritesResult {
    downloaded: usize,
    skipped: usize,
    failed: usize,
    last_error: Option<String>,
}

#[tauri::command]
async fn download_all_favorites(
    app: tauri::AppHandle,
    urls: Vec<String>,
) -> Result<DownloadFavoritesResult, String> {
    let dir = app_subdir(&app, "favorites_pdfs")?;
    let client = net::http_client()?;

    let mut downloaded = 0usize;
    let mut skipped = 0usize;
    let mut failed = 0usize;
    let mut last_error: Option<String> = None;

    for url in &urls {
        if url.is_empty() {
            continue;
        }

        let raw_name = url.rsplit('/').next().unwrap_or("datasheet.pdf");
        let clean_name = raw_name.split('?').next().unwrap_or("datasheet.pdf");
        let mut filename = clean_name.to_string();
        if !filename.to_lowercase().ends_with(".pdf") {
            filename.push_str(".pdf");
        }
        let safe_filename = sanitize_filename(&filename);
        let dest = dir.join(&safe_filename);

        if dest.exists() && net::validate_file_on_disk(&dest, &safe_filename).is_ok() {
            skipped += 1;
            continue;
        }
        let _ = fs::remove_file(&dest);

        let response = client
            .get(url)
            .header("Referer", "https://www.digikey.com/")
            .send()
            .await;

        let result = match response {
            Ok(resp) if resp.status().is_success() => match resp.bytes().await {
                Ok(bytes) => net::validate_pdf_bytes(&bytes)
                    .and_then(|_| net::write_atomic(&dest, &bytes)),
                Err(e) => Err(format!("Read error: {}", e)),
            },
            Ok(resp) => Err(format!("Server returned HTTP {}", resp.status().as_u16())),
            Err(e) => Err(format!("Download error: {}", e)),
        };

        match result {
            Ok(()) => downloaded += 1,
            Err(err) => {
                failed += 1;
                if last_error.is_none() {
                    last_error = Some(err);
                }
            }
        }
    }

    Ok(DownloadFavoritesResult {
        downloaded,
        skipped,
        failed,
        last_error,
    })
}

#[tauri::command]
async fn read_text_file(path: String) -> Result<String, String> {
    tauri::async_runtime::spawn_blocking(move || fs::read_to_string(&path).map_err(|e| e.to_string()))
        .await
        .map_err(|e| format!("Read task failed: {}", e))?
}

/// Returns raw bytes over Tauri's binary IPC channel (`ArrayBuffer` in JS).
/// Returning `Vec<u8>` directly would serialise every byte as a JSON number —
/// a 3 MB PDF became a ~10 MB JSON string to parse on the UI thread.
#[tauri::command]
async fn read_binary_file(path: String) -> Result<Response, String> {
    let bytes = tauri::async_runtime::spawn_blocking(move || {
        let meta = fs::metadata(&path)
            .map_err(|e| format!("Cannot read \"{}\": {}", path, e))?;
        // Guard against a multi-GB file exhausting memory (PDF bytes are copied
        // to the webview anyway).
        const MAX_BINARY_BYTES: u64 = 512 * 1024 * 1024;
        if meta.len() > MAX_BINARY_BYTES {
            return Err(format!(
                "File is too large ({:.0} MB) — the limit is 512 MB",
                meta.len() as f64 / (1024.0 * 1024.0)
            ));
        }
        fs::read(&path).map_err(|e| format!("Cannot read \"{}\": {}", path, e))
    })
    .await
    .map_err(|e| format!("Read task failed: {}", e))??;
    Ok(Response::new(bytes))
}

#[tauri::command]
async fn save_binary_file(dest: String, data: Vec<u8>) -> Result<(), String> {
    if data.is_empty() {
        return Err("There is nothing to save (empty file)".to_string());
    }
    let path = PathBuf::from(dest);
    net::validate_destination(&path)?;
    tauri::async_runtime::spawn_blocking(move || net::write_atomic(&path, &data))
        .await
        .map_err(|e| format!("Save task failed: {}", e))?
}

#[tauri::command]
async fn open_pdf(path: String) -> Result<(), String> {
    tauri_plugin_opener::open_path(&path, None::<&str>).map_err(|e| e.to_string())
}

#[tauri::command]
async fn open_url_in_browser(url: String) -> Result<(), String> {
    if !net::is_public_http_url(&url) {
        return Err(format!("Refusing to open non-http(s) URL: \"{}\"", url));
    }
    tauri_plugin_opener::open_url(&url, None::<&str>).map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_os::init());

    #[cfg(target_os = "android")]
    let builder = builder.plugin(tauri_plugin_android_fs::init());

    builder
        .invoke_handler(tauri::generate_handler![
            digikey::search_datasheets,
            digikey::download_pdf,
            digikey::read_pdf,
            digikey::read_favorite_pdf,
            digikey::save_pdf_copy,
            digikey::find_analogs_digikey,
            digikey::find_by_params_digikey,
            lcsc::search_lcsc,
            lcsc::find_analogs_lcsc,
            lcsc::find_by_params_lcsc,
            element14::search_element14,
            tme::search_tme,
            transistors::search_soviet_analog,
            get_system_locale,
            get_cache_size,
            clear_cache,
            get_favorites_size,
            clear_favorites,
            delete_cached_file,
            download_gdrive_file,
            download_database_file,
            download_database_batch,
            get_database_size,
            clear_database,
            download_mega_file,
            download_all_favorites,
            read_text_file,
            read_binary_file,
            save_binary_file,
            open_pdf,
            open_url_in_browser,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filename_cannot_escape_its_directory() {
        assert_eq!(sanitize_filename(".."), "file");
        assert_eq!(sanitize_filename("../evil.pdf"), "_evil.pdf");
        let windows_style = sanitize_filename("..\\..\\Windows\\system32\\x.pdf");
        assert!(!windows_style.contains('/') && !windows_style.contains('\\'));
        assert!(!sanitize_filename("../../etc/passwd").contains('/'));
        assert_eq!(sanitize_filename("Switch Endurance-HAD-CPU-01 image.pdf"), "Switch_Endurance-HAD-CPU-01_image.pdf");
        assert_eq!(sanitize_filename("catalog.json"), "catalog.json");
    }

    #[test]
    fn database_extensions_are_recognised() {
        assert!(has_db_extension(Path::new("board.pdf")));
        assert!(has_db_extension(Path::new("BOARD.PDF")));
        assert!(has_db_extension(Path::new("board.fz")));
        assert!(has_db_extension(Path::new("board.cad")));
        assert!(!has_db_extension(Path::new("notes.txt")));
        assert!(!has_db_extension(Path::new("board")));
    }
}
