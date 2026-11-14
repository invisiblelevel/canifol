use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::Manager;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Guide {
    pub id: String,
    pub title: String,
    pub link: String,
    pub image: Option<String>,
    pub description: String,
    pub source: String,
    pub r#type: String,
    pub published: String,
}

#[derive(Serialize, Deserialize)]
struct CachedGuides {
    timestamp: u64,
    guides: Vec<Guide>,
}

const YT_CHANNELS_EN: &[(&str, &str)] = &[
    ("NorthridgeFix", "UCLaXgfNlVxY149shiA1pykQ"),
    ("REWA Technology", "UC3ByF8DcZ3yxUs7VP1NOuyA"),
    ("Electronics Repair School", "UCooKQlg-HZ0PFAPc4Ymg3RA"),
];

const YT_CHANNELS_RU: &[(&str, &str)] = &[
    ("VIK-off", "UCjscfmLmyeajUdxDxrlQc9g"),
    ("notebook-31", "UCR1HSvPGgweMlKoHRkSkwjw"),
    ("Remonter", "UC2D9iDnY_ZD7sozyhN-Bq4A"),
    ("GadgetHelp", "UCyhYiQsHN7Od2UEVgi4mKNA"),
];

const BILI_UIDS: &[u64] = &[382666849, 611438407, 279787076, 1515460764];

const CACHE_TTL_SECS: u64 = 6 * 60 * 60;

fn get_guides_cache_dir(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let mut path = app.path().app_data_dir().map_err(|e| format!("No data dir: {}", e))?;
    path.push("guides");
    if !path.exists() { fs::create_dir_all(&path).map_err(|e| format!("Create dir error: {}", e))?; }
    Ok(path)
}

fn cache_path_for_locale(app: &tauri::AppHandle, locale: &str) -> Result<PathBuf, String> {
    let dir = get_guides_cache_dir(app)?;
    Ok(dir.join(format!("guides_cache_{}.json", locale)))
}

fn now_secs() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs()
}

fn clean_cdata(input: &str) -> String {
    let mut result = input.trim().to_string();
    if result.starts_with("<![CDATA[") { result = result.trim_start_matches("<![CDATA[").to_string(); }
    if result.ends_with("]]>") { result = result.trim_end_matches("]]>").to_string(); }
    result.trim().to_string()
}

fn make_id(link: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    link.hash(&mut hasher);
    format!("{:x}", hasher.finish())
}

fn parse_youtube_feed(xml: &str, source_name: &str) -> Vec<Guide> {
    let mut guides = Vec::new();
    let entry_re = regex::Regex::new(r"(?s)<(?:[^>]*:)?entry[^>]*>(.*?)</(?:[^>]*:)?entry>").unwrap();
    let title_re = regex::Regex::new(r"(?s)<(?:[^>]*:)?title[^>]*>(.*?)</(?:[^>]*:)?title>").unwrap();
    let published_re = regex::Regex::new(r"(?s)<(?:[^>]*:)?published[^>]*>(.*?)</(?:[^>]*:)?published>").unwrap();
    let link_re = regex::Regex::new(r#"<link[^>]*href="([^"]+)""#).unwrap();
    let thumb_re = regex::Regex::new(r#"<media:thumbnail[^>]*url="([^"]+)""#).unwrap();

    for cap in entry_re.captures_iter(xml) {
        if let Some(entry) = cap.get(1) {
            let entry_str = entry.as_str();
            let title = title_re.captures(entry_str).and_then(|c| c.get(1)).map(|m| clean_cdata(m.as_str())).unwrap_or_default();
            let published = published_re.captures(entry_str).and_then(|c| c.get(1)).map(|m| m.as_str().trim().to_string()).unwrap_or_default();
            let mut link = String::new();
            if let Some(cap) = link_re.captures(entry_str) { if let Some(m) = cap.get(1) { link = m.as_str().to_string(); } }
            let image = thumb_re.captures(entry_str).and_then(|c| c.get(1)).map(|m| m.as_str().to_string());

            if !title.is_empty() && !link.is_empty() {
                let embed_link = if let Some(vid) = link.split("v=").nth(1) {
                    format!("https://www.youtube.com/embed/{}", vid.split('&').next().unwrap_or(vid))
                } else { link.clone() };

                guides.push(Guide {
                    id: make_id(&link), title: title.clone(), link: embed_link, image,
                    description: String::new(), source: source_name.to_string(),
                    r#type: "video".to_string(), published,
                });
            }
        }
    }
    guides
}

/// Парсит JSON-ответ от rss2json.com (фоллбэк, когда прямой YouTube RSS отдаёт 404/пустоту).
fn parse_rss2json_feed(json: &str, source_name: &str) -> Vec<Guide> {
    let mut guides = Vec::new();
    let Ok(v) = serde_json::from_str::<serde_json::Value>(json) else { return guides; };
    let Some(items) = v.get("items").and_then(|i| i.as_array()) else { return guides; };

    for item in items {
        let title = item.get("title").and_then(|t| t.as_str()).unwrap_or("").to_string();
        let link = item.get("link").and_then(|l| l.as_str()).unwrap_or("").to_string();
        let pub_date = item.get("pubDate").and_then(|p| p.as_str()).unwrap_or("").to_string();
        let thumbnail = item.get("thumbnail").and_then(|t| t.as_str()).map(|s| s.to_string());

        if title.is_empty() || link.is_empty() { continue; }

        let embed_link = if let Some(vid) = link.split("v=").nth(1) {
            format!("https://www.youtube.com/embed/{}", vid.split('&').next().unwrap_or(vid))
        } else { link.clone() };

        guides.push(Guide {
            id: make_id(&link), title, link: embed_link, image: thumbnail,
            description: String::new(), source: source_name.to_string(),
            r#type: "video".to_string(), published: pub_date,
        });
    }
    guides
}

async fn fetch_youtube_feeds(channels: &[(&str, &str)]) -> Vec<Guide> {
    let client = reqwest::Client::new();
    let mut all = Vec::new();
    for (name, channel_id) in channels {
        let yt_url = format!("https://www.youtube.com/feeds/videos.xml?channel_id={}", channel_id);
        let mut got = false;

        // 1. Пробуем прямой запрос
        if let Ok(resp) = client.get(&yt_url).send().await {
            if resp.status().is_success() {
                if let Ok(text) = resp.text().await {
                    let parsed = parse_youtube_feed(&text, name);
                    if !parsed.is_empty() {
                        all.extend(parsed);
                        got = true;
                    }
                }
            }
        }

        // 2. Фоллбэк через rss2json (если прямой не дал результатов)
        if !got {
            let proxy_url = format!(
                "https://api.rss2json.com/v1/api.json?rss_url={}",
                urlencoding::encode(&yt_url)
            );
            if let Ok(resp) = client.get(&proxy_url).send().await {
                if resp.status().is_success() {
                    if let Ok(text) = resp.text().await {
                        let parsed = parse_rss2json_feed(&text, name);
                        if !parsed.is_empty() {
                            all.extend(parsed);
                        }
                    }
                }
            }
        }
    }
    all
}

async fn fetch_bilibili_feeds(page: u32) -> Result<Vec<Guide>, String> {
    let mut all = Vec::new();
    let mut risk_control = false;
    for uid in BILI_UIDS {
        match crate::bilibili::fetch_bilibili_videos(*uid, page).await {
            Ok(videos) => all.extend(videos),
            Err(e) => {
                if e == "RISK_CONTROL" { risk_control = true; }
            }
        }
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    }
    if risk_control && all.is_empty() {
        return Err("Bilibili временно недоступен (риск-контроль). Попробуйте позже.".to_string());
    }
    Ok(all)
}

#[tauri::command]
pub async fn fetch_guides(app: tauri::AppHandle, locale: String, force: Option<bool>, page: Option<u32>) -> Result<Vec<Guide>, String> {
    let force = force.unwrap_or(false);
    let page = page.unwrap_or(1);
    let cache_path = cache_path_for_locale(&app, &locale)?;

    if !force && page == 1 && cache_path.exists() {
        if let Ok(content) = fs::read_to_string(&cache_path) {
            if let Ok(cached) = serde_json::from_str::<CachedGuides>(&content) {
                if now_secs() - cached.timestamp < CACHE_TTL_SECS && !cached.guides.is_empty() {
                    return Ok(cached.guides);
                }
            }
        }
    }

    let mut guides = Vec::new();
    if locale == "ru" {
        guides.extend(fetch_youtube_feeds(YT_CHANNELS_RU).await);
    } else if locale == "zh" {
        guides = fetch_bilibili_feeds(page).await?;
    } else {
        guides.extend(fetch_youtube_feeds(YT_CHANNELS_EN).await);
    }

    guides.sort_by(|a, b| b.published.cmp(&a.published));

    if !guides.is_empty() && page == 1 {
        let cached = CachedGuides { timestamp: now_secs(), guides: guides.clone() };
        if let Ok(json) = serde_json::to_string(&cached) { let _ = fs::write(&cache_path, json); }
    }

    Ok(guides)
}

#[tauri::command]
pub fn get_guides_cache_size(app: tauri::AppHandle) -> Result<u64, String> {
    let dir = get_guides_cache_dir(&app)?;
    if !dir.exists() { return Ok(0); }
    let mut total = 0u64;
    if let Ok(entries) = fs::read_dir(&dir) {
        for entry in entries.flatten() {
            if let Ok(meta) = entry.metadata() {
                if meta.is_file() { total += meta.len(); }
            }
        }
    }
    Ok(total)
}

#[tauri::command]
pub fn clear_guides_cache(app: tauri::AppHandle) -> Result<(), String> {
    let dir = get_guides_cache_dir(&app)?;
    if dir.exists() { fs::remove_dir_all(&dir).map_err(|e| e.to_string())?; }
    Ok(())
}