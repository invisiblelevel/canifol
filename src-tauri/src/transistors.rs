use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Clone, Debug)]
pub struct SovietAnalog {
    pub name: String,
    pub analogs: Vec<String>,
}

/// Загружает JSON и возвращает карту { нормализованное_имя: [нормализованные_аналоги] }
fn load_analogs() -> Result<HashMap<String, Vec<String>>, String> {
    // Путь к JSON: в dev — из src/lib/data, в prod — нужно будет встроить в bundle.
    // Пока для простоты используем относительный путь от корня проекта.
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop(); // из src-tauri в корень
    path.push("src");
    path.push("lib");
    path.push("data");
    path.push("analogs.json");

    println!("[DEBUG] Загрузка аналогов из: {:?}", path);

    let content = fs::read_to_string(&path)
        .map_err(|e| format!("Не удалось прочитать analogs.json: {}", e))?;
    let json: Value = serde_json::from_str(&content)
        .map_err(|e| format!("Ошибка парсинга analogs.json: {}", e))?;

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
    for (_, v) in map.iter_mut() {
        v.sort();
        v.dedup();
    }

    println!("[DEBUG] Загружено уникальных ключей: {}", map.len());
    Ok(map)
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
    let map = load_analogs()?;
    let clean_query = normalize(&query.to_uppercase());

    // Ищем точное совпадение
    let mut results = Vec::new();
    if let Some(analogs) = map.get(&clean_query) {
        results.push(SovietAnalog {
            name: clean_query.clone(),
            analogs: analogs.clone(),
        });
    } else {
        // Если точного нет — ищем частичное совпадение
        for (key, analogs) in &map {
            if key.contains(&clean_query) {
                results.push(SovietAnalog {
                    name: key.clone(),
                    analogs: analogs.clone(),
                });
            }
        }
    }

    Ok(results)
}