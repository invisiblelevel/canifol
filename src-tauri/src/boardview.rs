use serde::Serialize;
use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;

use pcbrepair::decoder::DecodedPcbRepairFile;
use crate::gencad_parser::{parse_gencad, GenCadArc, GenCadComponent, GenCadLine, GenCadPad, GenCadRoute};

#[derive(Serialize, Clone, Debug)]
pub struct BoardPin {
    pub name: String,
    pub x: f64,
    pub y: f64,
    pub net: String,
    pub side: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct BoardData {
    pub format: String,
    pub pins: Vec<BoardPin>,
    pub lines: Vec<GenCadLine>,
    pub arcs: Vec<GenCadArc>,
    pub components: Vec<GenCadComponent>,
    pub pads: Vec<GenCadPad>,
    pub routes: Vec<GenCadRoute>,
}

fn build_components_from_pins_fz(pins: &[BoardPin]) -> Vec<GenCadComponent> {
    let mut pins_by_comp: HashMap<String, Vec<&BoardPin>> = HashMap::new();
    for pin in pins {
        let comp_name = pin.name.split('.').next().unwrap_or("").to_string();
        pins_by_comp.entry(comp_name).or_default().push(pin);
    }
    let mut components = Vec::new();
    for (ref_name, comp_pins) in pins_by_comp {
        if comp_pins.is_empty() { continue; }
        let mut min_x = f64::MAX; let mut min_y = f64::MAX;
        let mut max_x = f64::MIN; let mut max_y = f64::MIN;
        for p in &comp_pins {
            if p.x < min_x { min_x = p.x; } if p.y < min_y { min_y = p.y; }
            if p.x > max_x { max_x = p.x; } if p.y > max_y { max_y = p.y; }
        }
        let padding = 0.5; let min_size = 1.5;
        let mut x1 = min_x - padding; let mut y1 = min_y - padding;
        let mut x2 = max_x + padding; let mut y2 = max_y + padding;
        if (x2 - x1) < min_size { let cx = (x1+x2)/2.0; x1 = cx - min_size/2.0; x2 = cx + min_size/2.0; }
        if (y2 - y1) < min_size { let cy = (y1+y2)/2.0; y1 = cy - min_size/2.0; y2 = cy + min_size/2.0; }
        components.push(GenCadComponent {
            ref_name, x: (x1+x2)/2.0, y: (y1+y2)/2.0,
            shape: "".to_string(), layer: "TOP".to_string(),
            bounds_x1: x1, bounds_y1: y1, bounds_x2: x2, bounds_y2: y2,
            rotation: 0.0, mirror: false, device: "".to_string(),
        });
    }
    components
}

fn parse_fz_from_file(path: &str) -> Result<BoardData, String> {
    let file = File::open(path).map_err(|e| format!("File open error: {}", e))?;
    let reader = BufReader::new(file);
    let decoded = DecodedPcbRepairFile::new(reader).map_err(|e| format!("FZ decode error: {}", e))?;
    let content_str = String::from_utf8_lossy(&decoded.content);
    let mut pins = Vec::new();
    let mut current_block = "";
    for line in content_str.lines() {
        let line = line.trim();
        if line.is_empty() { continue; }
        if line.starts_with("A!") {
            if line.contains("NET_NAME") { current_block = "NET_NAME"; } else { current_block = ""; }
            continue;
        }
        if current_block == "NET_NAME" && line.starts_with("S!") {
            let parts: Vec<&str> = line.split('!').collect();
            if parts.len() >= 7 {
                pins.push(BoardPin {
                    name: format!("{}.{}", parts[2], parts[3]),
                    x: parts[5].parse().unwrap_or(0.0),
                    y: parts[6].parse().unwrap_or(0.0),
                    net: parts[1].to_string(),
                    side: "top".to_string(),
                });
            }
        }
    }
    let components = build_components_from_pins_fz(&pins);
    Ok(BoardData { format: "fz".to_string(), pins, lines: vec![], arcs: vec![], components, pads: vec![], routes: vec![] })
}

fn parse_cad(text: &str) -> Result<BoardData, String> {
    let gencad = parse_gencad(text)?;
    let mut pins = Vec::new();
    for p in &gencad.pins {
        pins.push(BoardPin { name: p.name.clone(), x: p.x, y: p.y, net: p.net.clone(), side: p.side.clone() });
    }
    Ok(BoardData {
        format: "cad".to_string(), pins,
        lines: gencad.lines, arcs: gencad.arcs, components: gencad.components,
        pads: gencad.pads, routes: gencad.routes,
    })
}

fn decode_bdv(bytes: &[u8]) -> Result<String, String> {
    let sig = b"dd:1.3?,r?-=bb";
    let mut data = bytes.to_vec();
    if data.len() >= sig.len() && &data[..sig.len()] == sig {
        let mut count: u16 = 0xA0;
        let mut i = 0;
        while i < data.len() {
            let byte = data[i];
            if byte == b'\r' || byte == b'\n' || byte == 0 {
                if byte == b'\r' && i + 1 < data.len() && data[i + 1] == b'\n' {
                    count += 1; if count > 285 { count = 159; }
                }
                i += 1; continue;
            }
            data[i] = ((count as u8).wrapping_sub(byte)) & 0xFF;
            i += 1;
        }
    }
    String::from_utf8(data).map_err(|e| format!("Invalid UTF-8 after decode: {}", e))
}

fn parse_bdv_from_content(content: &str) -> Result<BoardData, String> {
    let mut pins = Vec::new();
    let mut components_raw: HashMap<String, (Vec<f64>, Vec<f64>, String)> = HashMap::new();
    let mut current_part: Option<(String, String)> = None;
    let mut in_pins_section = false;

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() { continue; }
        if line.starts_with("<<pins.asc>>") { in_pins_section = true; continue; }
        if line.starts_with("<<format.asc>>") || line.starts_with("<<nails.asc>>") || line.starts_with("<<parts.asc>>") { in_pins_section = false; continue; }
        if in_pins_section {
            if line.starts_with("Part ") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 {
                    let name = parts[1].to_string();
                    let side = parts[2].trim_matches(|c| c == '(' || c == ')').to_string();
                    current_part = Some((name.clone(), side.clone()));
                    components_raw.entry(name).or_insert((vec![], vec![], side));
                }
            } else if line.chars().next().unwrap_or(' ').is_numeric() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 6 {
                    if let Some((ref_name, side)) = &current_part {
                        let pin_name = parts[1].to_string();
                        let x = parts[2].parse().unwrap_or(0.0);
                        let y = parts[3].parse().unwrap_or(0.0);
                        let net = parts[5].to_string();
                        let x_mm = x * 25.4; let y_mm = y * 25.4;
                        pins.push(BoardPin {
                            name: format!("{}.{}", ref_name, pin_name),
                            x: x_mm, y: y_mm, net,
                            side: if side == "T" { "top".to_string() } else { "bottom".to_string() },
                        });
                        if let Some((xs, ys, _)) = components_raw.get_mut(ref_name) { xs.push(x_mm); ys.push(y_mm); }
                    }
                }
            }
        }
    }

    let mut components = Vec::new();
    for (name, (xs, ys, side)) in &components_raw {
        if xs.is_empty() || ys.is_empty() { continue; }
        let min_x = xs.iter().cloned().fold(f64::MAX, f64::min);
        let max_x = xs.iter().cloned().fold(f64::MIN, f64::max);
        let min_y = ys.iter().cloned().fold(f64::MAX, f64::min);
        let max_y = ys.iter().cloned().fold(f64::MIN, f64::max);
        components.push(GenCadComponent {
            ref_name: name.clone(),
            x: (min_x + max_x) / 2.0, y: (min_y + max_y) / 2.0,
            shape: "".to_string(),
            layer: if side == "T" { "TOP".to_string() } else { "BOTTOM".to_string() },
            bounds_x1: min_x - 0.5, bounds_y1: min_y - 0.5,
            bounds_x2: max_x + 0.5, bounds_y2: max_y + 0.5,
            rotation: 0.0, mirror: false, device: "".to_string(),
        });
    }
    Ok(BoardData { format: "bdv".to_string(), pins, lines: vec![], arcs: vec![], components, pads: vec![], routes: vec![] })
}

fn parse_bdv(path: &str) -> Result<BoardData, String> {
    let bytes = std::fs::read(path).map_err(|e| format!("Read error: {}", e))?;
    let content = decode_bdv(&bytes)?;
    parse_bdv_from_content(&content)
}

#[tauri::command]
pub async fn read_boardview(path: String) -> Result<BoardData, String> {
    let ext = std::path::Path::new(&path).extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
    match ext.as_str() {
        "fz" | "cae" => parse_fz_from_file(&path),
        "cad" => { let text = std::fs::read_to_string(&path).map_err(|e| format!("Read error: {}", e))?; parse_cad(&text) }
        "bdv" | "asc" => parse_bdv(&path),
        _ => Err(format!("Unsupported format: {}", ext)),
    }
}

#[cfg(target_os = "android")]
#[tauri::command]
pub async fn read_boardview_uri(app: tauri::AppHandle, uri: String) -> Result<BoardData, String> {
    use tauri_plugin_android_fs::{AndroidFsExt, FileUri};
    use std::io::Read;
    use tauri::Manager;

    let file_uri = FileUri::from_uri(&uri);
    let api = app.android_fs_async();
    let mut file = api.open_file_readable(&file_uri).await.map_err(|e| e.to_string())?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).map_err(|e| e.to_string())?;

    // 1. BDV (зашифрованный)
    if buffer.starts_with(b"dd:1.3?,r?-=bb") {
        let content = decode_bdv(&buffer)?;
        return parse_bdv_from_content(&content);
    }

    // 2. ASC (текстовый BDV)
    if buffer.starts_with(b"<<format.asc>>") {
        let content = String::from_utf8(buffer).map_err(|e| format!("Invalid UTF-8: {}", e))?;
        return parse_bdv_from_content(&content);
    }

    // 3. CAD (GenCAD)
    if buffer.starts_with(b"$HEADER") || buffer.windows(7).any(|w| w == b"GENCAD") {
        let text = String::from_utf8(buffer).map_err(|e| format!("Invalid UTF-8: {}", e))?;
        return parse_cad(&text);
    }

    // 4. FZ (сохраняем в кэш приложения)
    if let Ok(cache_dir) = app.path().app_cache_dir() {
        let temp_path = cache_dir.join(format!("canifol_temp_{}.fz", std::process::id()));
        if let Ok(_) = std::fs::write(&temp_path, &buffer) {
            let result = parse_fz_from_file(temp_path.to_str().unwrap_or(""));
            let _ = std::fs::remove_file(&temp_path);
            if let Ok(data) = result {
                return Ok(data);
            }
        }
    }

    Err("Unsupported format on Android".to_string())
}