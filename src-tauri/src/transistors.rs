use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::OnceLock;

#[derive(Serialize, Clone, Debug)]
pub struct SovietAnalog {
    pub name: String,
    pub analogs: Vec<String>,
}

/// The database is compiled into the binary.  It used to be read from
/// `env!("CARGO_MANIFEST_DIR")`, i.e. the build machine's source folder, so on
/// every user's PC (and on Android) the whole Soviet-analog tab was dead.
const ANALOGS_JSON: &str = include_str!("../../src/lib/data/analogs.json");

static ANALOGS: OnceLock<Result<HashMap<String, Vec<String>>, String>> = OnceLock::new();

fn build_analogs() -> Result<HashMap<String, Vec<String>>, String> {
    let json: Value = serde_json::from_str(ANALOGS_JSON)
        .map_err(|e| format!("Failed to parse the built-in analogs database: {}", e))?;

    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    if let Some(obj) = json.as_object() {
        for (key, val) in obj {
            let clean_key = normalize(key);
            let mut analogs = Vec::new();
            if let Some(arr) = val.as_array() {
                for v in arr {
                    if let Some(s) = v.as_str() {
                        let clean = normalize(s);
                        if !clean.is_empty() {
                            analogs.push(clean);
                        }
                    }
                }
            }
            if !clean_key.is_empty() && !analogs.is_empty() {
                // Добавляем и в прямом, и в обратном направлении
                map.entry(clean_key.clone()).or_default().extend(analogs.clone());
                for a in &analogs {
                    map.entry(a.clone()).or_default().push(clean_key.clone());
                }
            }
        }
    }

    // Убираем дубликаты
    for value in map.values_mut() {
        value.sort();
        value.dedup();
    }

    Ok(map)
}

/// Parsed once per process instead of on every keystroke search.
fn analogs_map() -> Result<&'static HashMap<String, Vec<String>>, String> {
    match ANALOGS.get_or_init(build_analogs) {
        Ok(map) => Ok(map),
        Err(err) => Err(err.clone()),
    }
}

/// Нормализация кракозябр от PyMuPDF
fn normalize(s: &str) -> String {
    let mut result = s.replace('–', "");
    let map = [
        ("У", "Г"), ("Ґ", "Т"), ("Р", "А"), ("С", "Б"), ("Т", "В"),
        ("Ф", "Д"), ("Х", "Е"), ("Ц", "Ж"), ("Ч", "З"), ("Ш", "И"),
        ("Щ", "Й"), ("Ъ", "К"), ("Ы", "Л"), ("Ь", "М"), ("Э", "Н"),
        ("Ю", "О"), ("Я", "П"),
    ];
    for (from, to) in map {
        result = result.replace(from, to);
    }
    result.trim().to_string()
}

#[tauri::command]
pub async fn search_soviet_analog(query: String) -> Result<Vec<SovietAnalog>, String> {
    let clean_query = normalize(&query.to_uppercase());
    // An empty query used to match every key (`key.contains("")` is always true)
    // and return thousands of rows.
    if clean_query.is_empty() {
        return Ok(Vec::new());
    }

    let map = analogs_map()?;
    let mut results = Vec::new();

    if let Some(analogs) = map.get(&clean_query) {
        results.push(SovietAnalog {
            name: clean_query.clone(),
            analogs: analogs.clone(),
        });
    } else {
        let mut keys: Vec<&String> = map.keys().filter(|key| key.contains(&clean_query)).collect();
        keys.sort();
        for key in keys.into_iter().take(50) {
            results.push(SovietAnalog {
                name: key.clone(),
                analogs: map[key].clone(),
            });
        }
    }

    Ok(results)
}
