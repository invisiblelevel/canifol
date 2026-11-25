//! Shared networking / download helpers.
//!
//! Why this module exists: Google Drive (and other providers) regularly answer a
//! download request with HTTP 200 and a ~1–2 KB HTML page ("quota exceeded",
//! "file not found", "virus scan warning") instead of the actual file.  Older code
//! wrote that page to the cache as-is, so the app happily reported success while
//! the user got a tiny corrupt "PDF" and pdf.js said "Invalid PDF structure".
//!
//! Every download now goes through:
//!   * one shared, connection-pooled HTTP client (no TLS handshake per file);
//!   * Google Drive confirmation handling (cookie **and** HTML form token);
//!   * content validation before anything is written to disk;
//!   * atomic writes (temporary file + rename) so a crash never leaves a
//!     half-written file that is then treated as a valid cache entry.

use reqwest::Client;
use std::fs;
use std::io::{Read, Seek, SeekFrom};
use std::net::IpAddr;
use std::path::Path;
use std::sync::OnceLock;
use std::time::Duration;

pub const BROWSER_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) \
    AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";

static CLIENT: OnceLock<Client> = OnceLock::new();

/// Shared HTTP client.  Cloning a `reqwest::Client` is cheap (it is an `Arc`
/// internally), but sharing one instance also keeps the connection pool and the
/// Google Drive cookie jar alive between files.
pub fn http_client() -> Result<&'static Client, String> {
    if let Some(client) = CLIENT.get() {
        return Ok(client);
    }
    let built = Client::builder()
        .user_agent(BROWSER_USER_AGENT)
        .cookie_store(true)
        .connect_timeout(Duration::from_secs(20))
        // Large boardview scans can take a while; this is a per-request cap.
        .timeout(Duration::from_secs(600))
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {e}"))?;
    let _ = CLIENT.set(built);
    CLIENT
        .get()
        .ok_or_else(|| "Failed to create HTTP client".to_string())
}

static API_CLIENT: OnceLock<Client> = OnceLock::new();

/// Client for JSON APIs and small requests: a hung upstream must fail in tens of
/// seconds, not minutes (the download client allows large files).
pub fn api_client() -> Result<&'static Client, String> {
    if let Some(client) = API_CLIENT.get() {
        return Ok(client);
    }
    let built = Client::builder()
        .user_agent(BROWSER_USER_AGENT)
        .cookie_store(true)
        .connect_timeout(Duration::from_secs(10))
        .timeout(Duration::from_secs(45))
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {e}"))?;
    let _ = API_CLIENT.set(built);
    API_CLIENT
        .get()
        .ok_or_else(|| "Failed to create HTTP client".to_string())
}

fn contains(haystack: &[u8], needle: &[u8]) -> bool {
    !needle.is_empty()
        && haystack.len() >= needle.len()
        && haystack.windows(needle.len()).any(|w| w == needle)
}

fn trimmed_start(bytes: &[u8]) -> &[u8] {
    let mut i = 0;
    while i < bytes.len() && (bytes[i] == b' ' || bytes[i] == b'\t' || bytes[i] == b'\r' || bytes[i] == b'\n') {
        i += 1;
    }
    let bytes = &bytes[i..];
    // Skip a UTF-8 BOM if a provider prepended one.
    bytes.strip_prefix(&[0xEF, 0xBB, 0xBF]).unwrap_or(bytes)
}

pub fn looks_like_html(bytes: &[u8]) -> bool {
    let head = &bytes[..bytes.len().min(1024)];
    let text = String::from_utf8_lossy(head);
    let text = text.trim_start_matches('\u{feff}').trim_start().to_ascii_lowercase();
    text.starts_with("<!doctype html") || text.starts_with("<html") || text.starts_with("<head")
}

fn html_title(bytes: &[u8]) -> Option<String> {
    let text = String::from_utf8_lossy(&bytes[..bytes.len().min(8192)]);
    let lower = text.to_ascii_lowercase();
    let start = lower.find("<title>")? + "<title>".len();
    let end = lower[start..].find("</title>")? + start;
    let title = text.get(start..end)?.trim();
    if title.is_empty() {
        None
    } else {
        Some(title.chars().take(160).collect())
    }
}

/// Short, human readable description of an unexpected payload — used in errors so
/// users can tell "Google quota page" from "server returned a login page".
pub fn describe_payload(bytes: &[u8]) -> String {
    if bytes.is_empty() {
        return "an empty response".to_string();
    }
    if looks_like_html(bytes) {
        return match html_title(bytes) {
            Some(title) => format!("an HTML page (title: \"{title}\")"),
            None => "an HTML page".to_string(),
        };
    }
    let head = String::from_utf8_lossy(&bytes[..bytes.len().min(64)]);
    let printable: String = head.chars().map(|c| if c.is_control() { ' ' } else { c }).collect();
    format!("{} bytes starting with \"{}\"", bytes.len(), printable.trim())
}

fn starts_with_pdf(bytes: &[u8]) -> bool {
    trimmed_start(bytes).starts_with(b"%PDF")
}

/// A valid PDF starts with `%PDF` and ends with the `%%EOF` trailer.  The second
/// check is what catches truncated downloads (the exact "2 KB, Adobe says corrupt"
/// case) instead of silently caching them.
pub fn validate_pdf_bytes(bytes: &[u8]) -> Result<(), String> {
    if bytes.len() < 256 {
        return Err(format!(
            "only {} bytes were downloaded ({})",
            bytes.len(),
            describe_payload(bytes)
        ));
    }
    if !starts_with_pdf(bytes) {
        return Err(format!("the response is not a PDF ({})", describe_payload(bytes)));
    }
    let tail_start = bytes.len().saturating_sub(8192);
    if !contains(&bytes[tail_start..], b"%%EOF") {
        return Err(format!(
            "the PDF is incomplete — only {} bytes were received and the %%EOF trailer is missing (truncated download)",
            bytes.len()
        ));
    }
    Ok(())
}

pub fn validate_json_bytes(bytes: &[u8]) -> Result<(), String> {
    if bytes.len() < 2 {
        return Err(format!("only {} bytes were downloaded ({})", bytes.len(), describe_payload(bytes)));
    }
    serde_json::from_slice::<serde_json::Value>(bytes)
        .map(|_| ())
        .map_err(|e| format!("the response is not valid JSON ({e})"))
}

/// Validates downloaded content according to the expected file type.  HTML and
/// JSON error pages are always rejected, even for boardview files.
pub fn validate_download(bytes: &[u8], filename: &str) -> Result<(), String> {
    if bytes.is_empty() {
        return Err("the server returned an empty response".to_string());
    }
    if looks_like_html(bytes) {
        return Err(format!(
            "the server returned {} instead of the file",
            describe_payload(bytes)
        ));
    }
    let lower = filename.to_ascii_lowercase();
    if lower.ends_with(".pdf") {
        return validate_pdf_bytes(bytes);
    }
    if lower.ends_with(".json") {
        return validate_json_bytes(bytes);
    }
    // Boardview formats: text-based (GenCAD, BRD2, BDV) or binary (FZ, TVW, XZZ).
    // Size alone is enough to tell them from an error page.
    if bytes.len() < 64 {
        return Err(format!(
            "only {} bytes were downloaded ({}) — the file looks truncated",
            bytes.len(),
            describe_payload(bytes)
        ));
    }
    Ok(())
}

/// Cheap validation of an already cached file: reads the head (and the tail for
/// PDFs) instead of loading the whole file into memory.
pub fn validate_file_on_disk(path: &Path, filename: &str) -> Result<(), String> {
    let meta = fs::metadata(path).map_err(|e| format!("cannot read file metadata: {e}"))?;
    if !meta.is_file() {
        return Err("path is not a file".to_string());
    }
    let len = meta.len();
    if len < 64 {
        return Err(format!("file is only {len} bytes — probably an error page left by an earlier failed download"));
    }

    let mut file = fs::File::open(path).map_err(|e| format!("cannot open file: {e}"))?;
    let head_len = std::cmp::min(len, 1024) as usize;
    let mut head = vec![0u8; head_len];
    file.read_exact(&mut head)
        .map_err(|e| format!("cannot read file header: {e}"))?;
    if looks_like_html(&head) {
        return Err("file contains an HTML page instead of data".to_string());
    }

    let lower = filename.to_ascii_lowercase();
    if lower.ends_with(".pdf") {
        if !starts_with_pdf(&head) {
            return Err("file header is not a PDF".to_string());
        }
        let tail_len = std::cmp::min(len, 8192);
        file.seek(SeekFrom::End(-(tail_len as i64)))
            .map_err(|e| format!("cannot seek in file: {e}"))?;
        let mut tail = vec![0u8; tail_len as usize];
        file.read_exact(&mut tail)
            .map_err(|e| format!("cannot read file trailer: {e}"))?;
        if !contains(&tail, b"%%EOF") {
            return Err("PDF is truncated (no %%EOF trailer)".to_string());
        }
    } else if lower.ends_with(".json") {
        let bytes = fs::read(path).map_err(|e| format!("cannot read file: {e}"))?;
        validate_json_bytes(&bytes)?;
    } else if head.first() == Some(&b'{') && !lower.ends_with(".pcb") {
        // A JSON error body saved with a boardview extension.
        return Err("file contains a JSON response instead of boardview data".to_string());
    }
    Ok(())
}

/// Validates a destination path chosen by the user through the save dialog.
pub fn validate_destination(path: &Path) -> Result<(), String> {
    if path.as_os_str().is_empty() {
        return Err("No destination path was selected".to_string());
    }
    let parent = path
        .parent()
        .ok_or_else(|| "The destination path has no parent folder".to_string())?;
    if !parent.exists() {
        return Err(format!("Folder \"{}\" does not exist", parent.display()));
    }
    if path.is_dir() {
        return Err("The selected destination is a folder, not a file".to_string());
    }
    Ok(())
}

/// Writes `bytes` to a sibling temporary file first and only then renames it over
/// `path`, so the destination is never observed half-written.
pub fn write_atomic(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let parent = path
        .parent()
        .ok_or_else(|| "destination path has no parent directory".to_string())?;
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("download");
    let unique = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let tmp = parent.join(format!(".{file_name}.{}.{unique}.part", std::process::id()));

    fs::write(&tmp, bytes).map_err(|e| format!("cannot write temporary file: {e}"))?;
    if path.exists() {
        fs::remove_file(path).map_err(|e| {
            let _ = fs::remove_file(&tmp);
            format!("cannot replace existing file: {e}")
        })?;
    }
    fs::rename(&tmp, path).map_err(|e| {
        let _ = fs::remove_file(&tmp);
        format!("cannot finalize download: {e}")
    })
}

/// Rejects malformed URLs and URLs that point at the local machine / private
/// networks.  Datasheet URLs come from third-party APIs, so without this check a
/// hostile API answer (or a tampered catalog) could make the app fetch
/// `http://127.0.0.1:…` or a cloud metadata endpoint from the user's machine.
pub fn is_public_http_url(url: &str) -> bool {
    let Ok(parsed) = reqwest::Url::parse(url.trim()) else {
        return false;
    };
    if !matches!(parsed.scheme(), "http" | "https") {
        return false;
    }
    let Some(host) = parsed.host_str() else {
        return false;
    };
    let host = host.to_ascii_lowercase();
    if host == "localhost" || host.ends_with(".localhost") || host.ends_with(".local") {
        return false;
    }
    if let Ok(ip) = host.parse::<std::net::IpAddr>() {
        return match ip {
            IpAddr::V4(v4) => {
                !(v4.is_private()
                    || v4.is_loopback()
                    || v4.is_link_local()
                    || v4.is_unspecified()
                    || v4.is_broadcast())
            }
            IpAddr::V6(v6) => {
                !(v6.is_loopback() || v6.is_unspecified() || v6.is_unique_local() || v6.is_unicast_link_local())
            }
        };
    }
    true
}

fn html_input_value(html: &str, name: &str) -> Option<String> {
    let input_re = regex::Regex::new(&format!(
        r#"(?is)<input\b[^>]*\bname\s*=\s*["']{}["'][^>]*>"#,
        regex::escape(name)
    ))
    .ok()?;
    let tag = input_re.find(html)?.as_str();
    let value_re = regex::Regex::new(r#"(?is)\bvalue\s*=\s*["']([^"']*)["']"#).ok()?;
    value_re
        .captures(tag)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().to_string())
}

async fn gdrive_confirm(
    file_id: &str,
    confirm: &str,
    uuid: Option<&str>,
    at: Option<&str>,
) -> Result<Vec<u8>, String> {
    let mut url = format!(
        "https://drive.usercontent.google.com/download?id={}&export=download&confirm={}",
        urlencoding::encode(file_id),
        urlencoding::encode(confirm)
    );
    if let Some(uuid) = uuid {
        url.push_str(&format!("&uuid={}", urlencoding::encode(uuid)));
    }
    if let Some(at) = at {
        url.push_str(&format!("&at={}", urlencoding::encode(at)));
    }

    let client = http_client()?;
    let res = client
        .get(&url)
        .header("Referer", "https://drive.google.com/")
        .send()
        .await
        .map_err(|e| format!("Google Drive confirmation request failed: {e}"))?;

    let status = res.status();
    let body = res
        .bytes()
        .await
        .map_err(|e| format!("Google Drive confirmation was interrupted: {e}"))?
        .to_vec();

    if !status.is_success() {
        return Err(format!(
            "Google Drive returned HTTP {} after confirmation ({})",
            status.as_u16(),
            describe_payload(&body)
        ));
    }
    if looks_like_html(&body) {
        return Err(format!(
            "Google Drive still returned {} after confirmation",
            describe_payload(&body)
        ));
    }
    Ok(body)
}

/// Downloads a file from Google Drive by its file id.
///
/// Handles all three answers Google gives in the wild:
///   1. the file itself (HTTP 200);
///   2. a 303 redirect to `drive.usercontent.google.com` (followed automatically);
///   3. an HTML page with a confirmation form / `download_warning` cookie.
pub async fn gdrive_download(file_id: &str) -> Result<Vec<u8>, String> {
    if file_id.trim().is_empty() {
        return Err("Google Drive file id is empty".to_string());
    }
    let client = http_client()?;
    let url = format!(
        "https://drive.google.com/uc?export=download&id={}",
        urlencoding::encode(file_id)
    );

    let res = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Google Drive is unreachable: {e}"))?;

    let status = res.status();
    let content_type = res
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string();
    let cookies: Vec<String> = res
        .cookies()
        .filter(|c| c.name().starts_with("download_warning"))
        .map(|c| c.value().to_string())
        .collect();

    let body = res
        .bytes()
        .await
        .map_err(|e| format!("Google Drive download was interrupted: {e}"))?
        .to_vec();

    if !status.is_success() {
        return Err(format!(
            "Google Drive returned HTTP {} ({})",
            status.as_u16(),
            describe_payload(&body)
        ));
    }

    let looks_html = looks_like_html(&body)
        || (content_type.contains("text/html") && !body.starts_with(b"%PDF"));
    if !looks_html {
        return Ok(body);
    }

    // New-style confirmation page: the token lives in a hidden <input>.
    let html = String::from_utf8_lossy(&body).to_string();
    let form_confirm = html_input_value(&html, "confirm");
    let form_uuid = html_input_value(&html, "uuid");
    let form_at = html_input_value(&html, "at");

    // Try `confirm=t` first (works for most shared files and bypasses the
    // "can't scan for viruses" page), then the exact token from the page/cookie.
    let mut last_error = String::new();
    if let Ok(bytes) = gdrive_confirm(file_id, "t", form_uuid.as_deref(), form_at.as_deref()).await {
        return Ok(bytes);
    }
    let confirm = form_confirm.or_else(|| cookies.first().cloned());
    if let Some(token) = confirm {
        match gdrive_confirm(file_id, &token, form_uuid.as_deref(), form_at.as_deref()).await {
            Ok(bytes) => return Ok(bytes),
            Err(err) => last_error = err,
        }
    }

    let title = html_title(&body)
        .map(|t| format!(" (\"{t}\")"))
        .unwrap_or_default();
    Err(format!(
        "Google Drive returned an HTML page instead of the file{title}. \
         The file may have been removed from the shared database, the daily download \
         quota may be exceeded, or Google Drive is blocked on this network. \
         Connect through a VPN/proxy (the app uses the system proxy) or download the \
         file in a browser, then try again. {last_error}"
    ))
}

/// Retries transient failures (network errors, 5xx) a few times with a short
/// backoff.  Permanent answers (404, 403, empty id) are returned immediately.
pub async fn gdrive_download_with_retry(file_id: &str) -> Result<Vec<u8>, String> {
    let mut last_error = String::new();
    for attempt in 0..3u32 {
        match gdrive_download(file_id).await {
            Ok(bytes) => {
                if !bytes.is_empty() {
                    return Ok(bytes);
                }
                last_error = "Google Drive returned an empty response".to_string();
            }
            Err(err) => {
                // Permanent answers — no point in retrying each one three times.
                if err.contains("HTTP 404") || err.contains("HTTP 403") || err.contains("file id is empty") {
                    return Err(err);
                }
                last_error = err;
            }
        }
        if attempt < 2 {
            tokio::time::sleep(Duration::from_millis(700 * (attempt as u64 + 1))).await;
        }
    }
    Err(last_error)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pdf_with_trailer(len: usize) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(len);
        bytes.extend_from_slice(b"%PDF-1.4\n");
        while bytes.len() < len.saturating_sub(8) {
            bytes.push(b'x');
        }
        bytes.extend_from_slice(b"\n%%EOF\n");
        bytes
    }

    #[test]
    fn validates_a_complete_pdf() {
        assert!(validate_pdf_bytes(&pdf_with_trailer(2048)).is_ok());
    }

    #[test]
    fn rejects_a_truncated_pdf() {
        let mut bytes = pdf_with_trailer(2048);
        bytes.truncate(1200); // no %%EOF anymore
        let err = validate_pdf_bytes(&bytes).unwrap_err();
        assert!(err.contains("incomplete"), "unexpected error: {err}");
    }

    #[test]
    fn rejects_the_two_kilobyte_google_error_page() {
        let html = b"<html lang=\"en\" dir=ltr><meta charset=utf-8><title>Error 404 (Not Found)!!1</title><style>...</style></html>";
        assert!(looks_like_html(html));
        let err = validate_download(html, "board.pdf").unwrap_err();
        assert!(err.contains("HTML"), "unexpected error: {err}");
        assert!(validate_download(html, "board.fz").is_err());
    }

    #[test]
    fn rejects_doctype_html_even_without_lowercase_html_tag() {
        let html = b"<!DOCTYPE HTML><html><body>quota exceeded</body></html>";
        assert!(looks_like_html(html));
    }

    #[test]
    fn validates_json() {
        assert!(validate_json_bytes(br#"{"a": 1}"#).is_ok());
        assert!(validate_json_bytes(b"<html>").is_err());
    }

    #[test]
    fn accepts_public_urls_and_rejects_local_ones() {
        assert!(is_public_http_url("https://www.ti.com/lit/ds/symlink/lm7805.pdf"));
        assert!(is_public_http_url("http://example.com/file.pdf"));
        assert!(!is_public_http_url("file:///C:/Windows/win.ini"));
        assert!(!is_public_http_url("http://127.0.0.1:8080/secret"));
        assert!(!is_public_http_url("http://192.168.1.1/"));
        assert!(!is_public_http_url("http://localhost/"));
        assert!(!is_public_http_url("javascript:alert(1)"));
        assert!(!is_public_http_url(""));
    }

    #[test]
    fn detects_a_corrupt_cached_pdf_and_wants_a_redownload() {
        let dir = std::env::temp_dir().join(format!("canifol_cache_test_{}", std::process::id()));
        let _ = fs::create_dir_all(&dir);
        let path = dir.join("broken.pdf");
        // This is exactly what version 1.6.0 cached when Google Drive answered
        // with an error page: a ~1.6 KB HTML document named "*.pdf".
        let mut html = String::from("<html lang=\"en\"><title>Error 404 (Not Found)!!1</title><style>");
        while html.len() < 1600 {
            html.push('x');
        }
        html.push_str("</style></html>");
        fs::write(&path, html.as_bytes()).unwrap();
        let err = validate_file_on_disk(&path, "broken.pdf").unwrap_err();
        assert!(err.contains("HTML"), "unexpected error: {err}");
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn atomic_write_replaces_the_file() {
        let dir = std::env::temp_dir().join(format!("canifol_net_test_{}", std::process::id()));
        let _ = fs::create_dir_all(&dir);
        let path = dir.join("file.pdf");
        fs::write(&path, b"old").unwrap();
        let bytes = pdf_with_trailer(2048);
        write_atomic(&path, &bytes).unwrap();
        assert_eq!(fs::read(&path).unwrap(), bytes);
        assert!(validate_file_on_disk(&path, "file.pdf").is_ok());
        let leftovers: Vec<_> = fs::read_dir(&dir)
            .unwrap()
            .flatten()
            .filter(|e| e.file_name().to_string_lossy().ends_with(".part"))
            .collect();
        assert!(leftovers.is_empty(), "temporary .part files were left behind");
        let _ = fs::remove_dir_all(&dir);
    }
}
