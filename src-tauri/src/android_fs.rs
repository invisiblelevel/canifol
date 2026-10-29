use tauri_plugin_android_fs::{AndroidFsExt, FileUri};
use std::io::Read;

#[tauri::command]
pub async fn read_android_file(app: tauri::AppHandle, uri: String) -> Result<Vec<u8>, String> {
    let file_uri = FileUri::from_uri(&uri);

    let api = app.android_fs_async();
    let mut file = api.open_file_readable(&file_uri).await.map_err(|e| e.to_string())?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).map_err(|e| e.to_string())?;
    Ok(buffer)
}