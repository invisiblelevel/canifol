use reqwest::Client;
use serde::Deserialize;
use std::time::{SystemTime, UNIX_EPOCH};

const NAV_URL: &str = "https://api.bilibili.com/x/web-interface/nav";
const SEARCH_URL: &str = "https://api.bilibili.com/x/space/wbi/arc/search";
const BUVID_URL: &str = "https://api.bilibili.com/x/frontend/finger/spi";
const TICKET_URL: &str = "https://api.bilibili.com/bapis/bilibili.api.ticket.v1.Ticket/GenWebTicket";

const MIXIN_KEY_ENC_TAB: [usize; 64] = [
    46, 47, 18, 2, 53, 8, 23, 32, 15, 50, 10, 31, 58, 3, 45, 35, 27, 43, 5, 49,
    33, 9, 42, 19, 29, 28, 14, 39, 12, 38, 41, 13, 37, 48, 7, 16, 24, 55, 40,
    61, 26, 17, 0, 1, 60, 51, 30, 4, 22, 25, 54, 21, 56, 59, 6, 63, 57, 62, 11,
    36, 20, 34, 44, 52,
];

#[derive(Deserialize)]
struct BuvidResponse { data: BuvidData }
#[derive(Deserialize)]
struct BuvidData { b_3: String, b_4: String }

#[derive(Deserialize)]
struct NavResponse { data: NavData }
#[derive(Deserialize)]
struct NavData { wbi_img: WbiImg }
#[derive(Deserialize)]
struct WbiImg { img_url: String, sub_url: String }

#[derive(Deserialize)]
struct SearchResponse { data: SearchData }
#[derive(Deserialize)]
struct SearchData { list: SearchList }
#[derive(Deserialize)]
struct SearchList { vlist: Vec<VideoItem> }
#[derive(Deserialize)]
struct VideoItem { bvid: String, title: String, pic: String, created: u64, author: String }

fn get_mixin_key(img_key: &str, sub_key: &str) -> String {
    let raw = format!("{}{}", img_key, sub_key);
    let chars: Vec<char> = raw.chars().collect();
    let mut result = String::with_capacity(32);
    for &idx in MIXIN_KEY_ENC_TAB.iter().take(32) {
        if let Some(&c) = chars.get(idx) { result.push(c); }
    }
    result
}

fn md5_hex(input: &str) -> String {
    format!("{:x}", md5::compute(input.as_bytes()))
}

fn hmac_sha256_hex(key: &str, msg: &str) -> String {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;
    type HmacSha256 = Hmac<Sha256>;
    let mut mac = HmacSha256::new_from_slice(key.as_bytes()).unwrap();
    mac.update(msg.as_bytes());
    format!("{:x}", mac.finalize().into_bytes())
}

async fn get_buvid(client: &Client) -> Result<(String, String), String> {
    let res = client.get(BUVID_URL)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .send().await.map_err(|e| format!("Buvid request error: {}", e))?;
    let b: BuvidResponse = res.json().await.map_err(|e| format!("Buvid parse error: {}", e))?;
    Ok((b.data.b_3, b.data.b_4))
}

async fn get_bili_ticket(client: &Client) -> Result<String, String> {
    let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
    let hexsign = hmac_sha256_hex("XgwSnGZ1p", &format!("ts{}", ts));
    let res = client.post(TICKET_URL)
        .query(&[("key_id", "ec02"), ("hexsign", &hexsign), ("context[ts]", &ts.to_string())])
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .header("Referer", "https://www.bilibili.com/")
        .send().await.map_err(|e| format!("Ticket request error: {}", e))?;
    let json: serde_json::Value = res.json().await.map_err(|e| format!("Ticket parse error: {}", e))?;
    json.get("data").and_then(|d| d.get("ticket")).and_then(|t| t.as_str())
        .map(|s| s.to_string()).ok_or("No ticket in response".to_string())
}

async fn get_wbi_keys(client: &Client, buvid3: &str, buvid4: &str, ticket: &str) -> Result<(String, String), String> {
    let res = client.get(NAV_URL)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .header("Referer", "https://www.bilibili.com/")
        .header("Cookie", format!("buvid3={}; buvid4={}; bili_ticket={}", buvid3, buvid4, ticket))
        .send().await.map_err(|e| format!("Nav request error: {}", e))?;
    let nav: NavResponse = res.json().await.map_err(|e| format!("Nav parse error: {}", e))?;
    let img_key = nav.data.wbi_img.img_url.split('/').last().and_then(|s| s.split('.').next()).unwrap_or("").to_string();
    let sub_key = nav.data.wbi_img.sub_url.split('/').last().and_then(|s| s.split('.').next()).unwrap_or("").to_string();
    Ok((img_key, sub_key))
}

fn sign_params(params: &[(String, String)], mixin_key: &str) -> String {
    let mut sorted: Vec<_> = params.to_vec();
    sorted.sort_by(|a, b| a.0.cmp(&b.0));
    let mut query = String::new();
    for (i, (k, v)) in sorted.iter().enumerate() {
        if i > 0 { query.push('&'); }
        let filtered = v.replace(['!', '\'', '(', ')', '*'], "");
        query.push_str(&urlencoding::encode(k));
        query.push('=');
        query.push_str(&urlencoding::encode(&filtered));
    }
    let wts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
    query.push_str(&format!("&wts={}", wts));
    let w_rid = md5_hex(&format!("{}{}", query, mixin_key));
    query.push_str(&format!("&w_rid={}", w_rid));
    query
}

#[tauri::command]
pub async fn fetch_bilibili_videos(uid: u64, page: u32) -> Result<Vec<crate::guides::Guide>, String> {
    let client = Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .timeout(std::time::Duration::from_secs(20))
        .build().map_err(|e| format!("Client error: {}", e))?;

    let (buvid3, buvid4) = get_buvid(&client).await?;
    let ticket = get_bili_ticket(&client).await?;
    let (img_key, sub_key) = get_wbi_keys(&client, &buvid3, &buvid4, &ticket).await?;
    let mixin_key = get_mixin_key(&img_key, &sub_key);

    let params = vec![
        ("mid".to_string(), uid.to_string()),
        ("pn".to_string(), page.to_string()),
        ("ps".to_string(), "40".to_string()),
        ("order".to_string(), "pubdate".to_string()),
        ("platform".to_string(), "web".to_string()),
        ("web_location".to_string(), "333.1387".to_string()),
        ("order_avoided".to_string(), "true".to_string()),
        ("dm_img_str".to_string(), "V2ViR0wgMS4wIChPcGVuR0wgRVMgMi4wIENocm9taXVtKQ".to_string()),
        ("dm_cover_img_str".to_string(), "QU5HTEUgKEludGVsLCBJbnRlbChSKSBVSEQgR3JhcGhpY3MgNjMwICgweDAwMDAzRTlCKSBEaXJlY3QzRDExIHZzXzVfMCBwc181XzApLCBvciBzaW1pbGFy".to_string()),
    ];
    let query = sign_params(&params, &mixin_key);
    let url = format!("{}?{}", SEARCH_URL, query);

    let res = client.get(&url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .header("Referer", format!("https://space.bilibili.com/{}/upload/video", uid))
        .header("Origin", "https://space.bilibili.com")
        .header("Cookie", format!("buvid3={}; buvid4={}; bili_ticket={}", buvid3, buvid4, ticket))
        .send().await.map_err(|e| format!("Search request error: {}", e))?;

    if res.status() == 412 {
        return Err("RISK_CONTROL".to_string());
    }
    if !res.status().is_success() {
        return Err(format!("Bilibili returned: {}", res.status()));
    }

    let search: SearchResponse = res.json().await.map_err(|e| format!("Search parse error: {}", e))?;

    let mut guides = Vec::new();
    for v in search.data.list.vlist {
        let embed_url = format!("https://player.bilibili.com/player.html?bvid={}&danmaku=0&autoplay=0&high_quality=1", v.bvid);
        guides.push(crate::guides::Guide {
            id: format!("bili_{}", v.bvid),
            title: v.title,
            link: embed_url,
            image: Some(v.pic),
            description: String::new(),
            source: v.author,
            r#type: "video".to_string(),
            published: v.created.to_string(),
        });
    }
    Ok(guides)
}