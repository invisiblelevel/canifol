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
    pub r#type: String, // "video"
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

const CACHE_TTL_SECS: u64 = 6 * 60 * 60;

fn get_guides_cache_dir(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let mut path = app.path().app_data_dir().map_err(|e| format!("No data dir: {}", e))?;
    path.push("guides");
    if !path.exists() {
        fs::create_dir_all(&path).map_err(|e| format!("Create dir error: {}", e))?;
    }
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
    if result.starts_with("<![CDATA[") {
        result = result.trim_start_matches("<![CDATA[").to_string();
    }
    if result.ends_with("]]>") {
        result = result.trim_end_matches("]]>").to_string();
    }
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
            if let Some(cap) = link_re.captures(entry_str) {
                if let Some(m) = cap.get(1) {
                    link = m.as_str().to_string();
                }
            }
            
            let image = thumb_re.captures(entry_str).and_then(|c| c.get(1)).map(|m| m.as_str().to_string());

            if !title.is_empty() && !link.is_empty() {
                let embed_link = if let Some(vid) = link.split("v=").nth(1) {
                    format!("https://www.youtube.com/embed/{}", vid.split('&').next().unwrap_or(vid))
                } else {
                    link.clone()
                };

                guides.push(Guide {
                    id: make_id(&link),
                    title: title.clone(),
                    link: embed_link,
                    image,
                    description: String::new(),
                    source: source_name.to_string(),
                    r#type: "video".to_string(),
                    published,
                });
            }
        }
    }
    guides
}

async fn fetch_youtube_feeds(channels: &[(&str, &str)]) -> Vec<Guide> {
    let client = reqwest::Client::new();
    let mut all = Vec::new();
    for (name, channel_id) in channels {
        let url = format!("https://www.youtube.com/feeds/videos.xml?channel_id={}", channel_id);
        match client.get(&url).send().await {
            Ok(resp) => {
                let status = resp.status();
                match resp.text().await {
                    Ok(text) => {
                        let parsed = parse_youtube_feed(&text, name);
                        eprintln!("[guides] YouTube {}: status={}, parsed={}", name, status, parsed.len());
                        all.extend(parsed);
                    }
                    Err(e) => eprintln!("[guides] YouTube {} text error: {}", name, e),
                }
            }
            Err(e) => eprintln!("[guides] YouTube {} request error: {}", name, e),
        }
    }
    all
}

#[tauri::command]
pub async fn fetch_guides(app: tauri::AppHandle, locale: String, force: Option<bool>) -> Result<Vec<Guide>, String> {
    let force = force.unwrap_or(false);
    let cache_path = cache_path_for_locale(&app, &locale)?;
    
    if !force && cache_path.exists() {
        if let Ok(content) = fs::read_to_string(&cache_path) {
            if let Ok(cached) = serde_json::from_str::<CachedGuides>(&content) {
                if now_secs() - cached.timestamp < CACHE_TTL_SECS && !cached.guides.is_empty() {
                    eprintln!("[guides] cache hit for locale={}, count={}", locale, cached.guides.len());
                    return Ok(cached.guides);
                }
            }
        }
    }

    eprintln!("[guides] fetching fresh data for locale={}, force={}", locale, force);
    let mut guides = Vec::new();
    
    if locale == "ru" {
        guides.extend(fetch_youtube_feeds(YT_CHANNELS_RU).await);
    } else {
        guides.extend(fetch_youtube_feeds(YT_CHANNELS_EN).await);
    }

    guides.sort_by(|a, b| {
        if a.published.is_empty() && b.published.is_empty() { std::cmp::Ordering::Equal }
        else if a.published.is_empty() { std::cmp::Ordering::Greater }
        else if b.published.is_empty() { std::cmp::Ordering::Less }
        else { b.published.cmp(&a.published) }
    });

    if !guides.is_empty() {
        let cached = CachedGuides { timestamp: now_secs(), guides: guides.clone() };
        if let Ok(json) = serde_json::to_string(&cached) { let _ = fs::write(&cache_path, json); }
    } else {
        let _ = fs::remove_file(&cache_path);
    }

    eprintln!("[guides] returning {} guides for locale={}", guides.len(), locale);
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