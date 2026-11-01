use serde::Serialize;
use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;
use pcbrepair::decoder::DecodedPcbRepairFile;
use crate::gencad_parser::{parse_gencad, GenCadArc, GenCadComponent, GenCadLine, GenCadPad, GenCadRoute};
use des::cipher::{BlockDecrypt, KeyInit, generic_array::GenericArray};
use des::Des;

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

// ==========================================
// 1. ПАРСЕР FZ
// ==========================================
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

// ==========================================
// 2. ПАРСЕР CAD
// ==========================================
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
// ==========================================
// ПАРСЕР BRD Apple (Ультра-толерантный рабочий вариант)
// ==========================================
fn parse_apple_brd(path: &str) -> Result<BoardData, String> {
    let raw = std::fs::read(path).map_err(|e| format!("Read error: {}", e))?;
    if raw.len() < 4 || &raw[0..4] != b"\x23\xE2\x63\x28" {
        return Err("Not an Apple BRD file (wrong magic)".to_string());
    }

    let mut data = raw.clone();
    for b in data.iter_mut() {
        if *b != b'\r' && *b != b'\n' && *b != 0 {
            let c = *b as i8 as i32;
            let shifted = ((c >> 6) & 3) | (c << 2);
            *b = (!shifted & 0xFF) as u8;
        }
    }

    let text = String::from_utf8_lossy(&data);
    let mut pins_raw: Vec<(f64, f64, String)> = Vec::new();
    let mut current_section = "";

    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() { continue; }

        if line.starts_with("str_length:") { current_section = "str_length"; continue; }
        if line.starts_with("var_data:") { current_section = "var_data"; continue; }
        if line.starts_with("Format:") { current_section = "format"; continue; }
        if line.starts_with("Pins1:") { current_section = "pins1"; continue; }
        if line.starts_with("Pins2:") { current_section = "pins2"; continue; }
        if line.starts_with("Pins:") { current_section = "pins"; continue; }
        if line.starts_with("Nails:") { current_section = "nails"; continue; }

        let parts: Vec<&str> = line.split_whitespace().collect();

        match current_section {
            "pins2" | "pins" => {
                if parts.len() >= 5 {
                    if let (Ok(x), Ok(y)) = (parts[0].parse::<f64>(), parts[1].parse::<f64>()) {
                        let net = parts[4].to_string();
                        pins_raw.push((x, y, net));
                    }
                }
            }
            _ => {}
        }
    }

    eprintln!("[Apple BRD] pins_raw: {}", pins_raw.len());

    let mut final_pins = Vec::new();
    for (i, (x, y, net)) in pins_raw.iter().enumerate() {
        let x_mm = x / 1000.0 * 25.4;
        let y_mm = y / 1000.0 * 25.4;
        final_pins.push(BoardPin {
            name: format!("P{}.{}", i + 1, if net.is_empty() { "NC" } else { net }),
            x: x_mm,
            y: y_mm,
            net: net.clone(),
            side: "top".to_string(),
        });
    }

    eprintln!("[Apple BRD] Final: 0 components, {} pins", final_pins.len());

    Ok(BoardData {
        format: "apple_brd".to_string(),
        pins: final_pins,
        lines: vec![],
        arcs: vec![],
        components: vec![], // <-- НЕ создаем компоненты
        pads: vec![],
        routes: vec![],
    })
}

// ==========================================
// 3. ПАРСЕР BRD
// ==========================================
fn parse_brd2(path: &str) -> Result<BoardData, String> {
    let content = std::fs::read_to_string(path).map_err(|e| format!("Read error: {}", e))?;
    
    let mut pins = Vec::new();
    let mut components = Vec::new();
    let mut nets: HashMap<usize, String> = HashMap::new();
    
    // Временное хранилище для партов (имя, x1, y1, x2, y2, start_pin_idx, side)
    let mut parts_raw: Vec<(String, f64, f64, f64, f64, usize, String)> = Vec::new();
    // Временное хранилище для пинов (x, y, net_idx, side)
    let mut pins_raw: Vec<(f64, f64, usize, String)> = Vec::new();
    
    let mut current_section = "";
    
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() { continue; }
        
        if line.starts_with("BRDOUT:") {
            current_section = "BRDOUT";
            continue;
        } else if line.starts_with("NETS:") {
            current_section = "NETS";
            continue;
        } else if line.starts_with("PARTS:") {
            current_section = "PARTS";
            continue;
        } else if line.starts_with("PINS:") {
            current_section = "PINS";
            continue;
        } else if line.starts_with("NAILS:") {
            current_section = "NAILS";
            continue;
        }
        
        let parts: Vec<&str> = line.split_whitespace().collect();
        
        match current_section {
            "NETS" => {
                if parts.len() >= 2 {
                    if let Ok(idx) = parts[0].parse::<usize>() {
                        nets.insert(idx, parts[1].to_string());
                    }
                }
            }
            "PARTS" => {
                if parts.len() >= 7 {
                    let name = parts[0].to_string();
                    let x1 = parts[1].parse().unwrap_or(0.0);
                    let y1 = parts[2].parse().unwrap_or(0.0);
                    let x2 = parts[3].parse().unwrap_or(0.0);
                    let y2 = parts[4].parse().unwrap_or(0.0);
                    let start_idx = parts[5].parse().unwrap_or(0);
                    let side = if parts[6] == "1" { "top".to_string() } else { "bottom".to_string() };
                    parts_raw.push((name, x1, y1, x2, y2, start_idx, side));
                }
            }
            "PINS" => {
                if parts.len() >= 4 {
                    let x = parts[0].parse().unwrap_or(0.0);
                    let y = parts[1].parse().unwrap_or(0.0);
                    let net_idx = parts[2].parse().unwrap_or(0);
                    let side = if parts[3] == "1" { "top".to_string() } else { "bottom".to_string() };
                    pins_raw.push((x, y, net_idx, side));
                }
            }
            _ => {}
        }
    }
    
    // Теперь связываем пины с компонентами по индексам
    // В BRD2: part[i] владеет пинами от start_idx до start_idx следующего парта
    let mut part_pin_counts: Vec<usize> = Vec::new();
    for i in 0..parts_raw.len() {
        let end = if i + 1 < parts_raw.len() {
            parts_raw[i + 1].5
        } else {
            pins_raw.len()
        };
        let count = end.saturating_sub(parts_raw[i].5);
        part_pin_counts.push(count);
    }
    
    // Создаем компоненты и пины
    let mut global_pin_idx = 0;
    for (i, (name, x1, y1, x2, y2, _start_idx, side)) in parts_raw.iter().enumerate() {
        components.push(GenCadComponent {
            ref_name: name.clone(),
            x: (x1 + x2) / 2.0,
            y: (y1 + y2) / 2.0,
            shape: "".to_string(),
            layer: if side == "top" { "TOP".to_string() } else { "BOTTOM".to_string() },
            bounds_x1: *x1,
            bounds_y1: *y1,
            bounds_x2: *x2,
            bounds_y2: *y2,
            rotation: 0.0,
            mirror: false,
            device: "".to_string(),
        });
        
        // Добавляем пины для этого компонента
        let pin_count = part_pin_counts[i];
        for p in 0..pin_count {
            if global_pin_idx < pins_raw.len() {
                let (px, py, net_idx, pin_side) = &pins_raw[global_pin_idx];
                let net_name = nets.get(net_idx).cloned().unwrap_or_else(|| "".to_string());
                pins.push(BoardPin {
                    name: format!("{}.{}", name, p + 1),
                    x: *px,
                    y: *py,
                    net: net_name,
                    side: pin_side.clone(),
                });
                global_pin_idx += 1;
            }
        }
    }
    
    Ok(BoardData {
        format: "brd2".to_string(),
        pins,
        lines: vec![],
        arcs: vec![],
        components,
        pads: vec![],
        routes: vec![],
    })
}


// ==========================================
// 3. ПАРСЕР BDV
// ==========================================
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

// ==========================================
// 4. ПАРСЕР XZZ (.pcb) — МАСШТАБ 250.0 И ОТДЕЛЬНЫЙ СЛОЙ КОМПОНЕНТОВ
// ==========================================
fn get_xzz_des_key() -> GenericArray<u8, des::cipher::generic_array::typenum::U8> {
    let byte_list: [u16; 8] = [0xE0, 0xCF, 0x2E, 0x9F, 0x3C, 0x33, 0x3C, 0x33];
    let mut hex_str = String::with_capacity(16);
    for chunk in byte_list.chunks(2) {
        let val = ((chunk[0] as u16) << 8) | (chunk[1] as u16);
        let xored = val ^ 0x3C33;
        hex_str.push_str(&format!("{:04X}", xored));
    }
    let key_u64 = u64::from_str_radix(&hex_str, 16).unwrap_or(0);
    GenericArray::from(key_u64.to_be_bytes())
}

fn parse_xzz(path: &str) -> Result<BoardData, String> {
    let file_data = std::fs::read(path).map_err(|e| format!("Read error: {}", e))?;
    if file_data.len() < 6 { return Err("File too small".to_string()); }

    let sig = b"XZZPCB";
    let mut data = file_data.clone();

    if &data[0..6] != sig {
        if data.len() > 0x10 {
            let xor_key = data[0x10];
            if xor_key != 0 {
                for b in data.iter_mut() { *b ^= xor_key; }
            }
        }
        if &data[0..6] != sig { return Err("Not a valid XZZPCB file".to_string()); }
    }

    if data.len() < 0x44 { return Err("Header too small".to_string()); }
    let main_data_offset = 0x40;
    let main_data_size = u32::from_le_bytes(data[0x40..0x44].try_into().unwrap()) as usize;

    let des_key = get_xzz_des_key();
    let cipher = Des::new(&des_key);

    let mut pins = Vec::new();
    let mut components = Vec::new();
    let mut lines = Vec::new();
    let mut current_offset = main_data_offset + 4;
    let end_offset = current_offset + main_data_size;

    let decrypt_block_data = |input: &[u8]| -> Vec<u8> {
        let mut decrypted = input.to_vec();
        for chunk in decrypted.chunks_exact_mut(8) {
            let mut block = GenericArray::clone_from_slice(chunk);
            cipher.decrypt_block(&mut block);
            chunk.copy_from_slice(&block);
        }
        decrypted
    };

    while current_offset < end_offset && current_offset + 5 <= data.len() {
        let block_type = data[current_offset];
        let block_size = u32::from_le_bytes(data[current_offset + 1..current_offset + 5].try_into().unwrap()) as usize;
        current_offset += 5;

        if current_offset + block_size > data.len() { break; }
        let block_data = &data[current_offset..current_offset + block_size];

        match block_type {
            0x05 => { // Основные дорожки платы
                if block_data.len() >= 28 {
                    let layer = u32::from_le_bytes(block_data[0..4].try_into().unwrap());
                    let x1 = (i32::from_le_bytes(block_data[4..8].try_into().unwrap()) as f64 / 10000.0) * 250.0;
                    let y1 = (i32::from_le_bytes(block_data[8..12].try_into().unwrap()) as f64 / 10000.0) * 250.0;
                    let x2 = (i32::from_le_bytes(block_data[12..16].try_into().unwrap()) as f64 / 10000.0) * 250.0;
                    let y2 = (i32::from_le_bytes(block_data[16..20].try_into().unwrap()) as f64 / 10000.0) * 250.0;
                    lines.push(GenCadLine { layer: layer.to_string(), x1, y1, x2, y2 });
                }
            }
            0x07 => { // Компоненты, контуры (0x05), надписи (0x06) и пины (0x09)
                let decrypted = decrypt_block_data(block_data);
                if decrypted.len() >= 26 {
                    let mut local_offset = 0;
                    local_offset += 4; // skip overall size
                    local_offset += 4; // padding

                    let part_x = (i32::from_le_bytes(decrypted[local_offset..local_offset+4].try_into().unwrap()) as f64 / 10000.0) * 250.0;
                    local_offset += 4;
                    let part_y = (i32::from_le_bytes(decrypted[local_offset..local_offset+4].try_into().unwrap()) as f64 / 10000.0) * 250.0;
                    local_offset += 4;
                    let part_rotation = u32::from_le_bytes(decrypted[local_offset..local_offset+4].try_into().unwrap()) as f64 / 10000.0;
                    local_offset += 4;

                    local_offset += 2; // flags
                    let pad_size_len = u32::from_le_bytes(decrypted[local_offset..local_offset+4].try_into().unwrap()) as usize;
                    local_offset += 4;

                    let mut footprint_name = String::new();
                    if pad_size_len > 0 && local_offset + pad_size_len <= decrypted.len() {
                        footprint_name = String::from_utf8_lossy(&decrypted[local_offset..local_offset+pad_size_len]).to_string();
                    }
                    local_offset += pad_size_len;

                    let mut ref_name = if !footprint_name.is_empty() {
                        footprint_name.clone()
                    } else {
                        format!("COMP_{}", components.len())
                    };

                    while local_offset + 5 <= decrypted.len() {
                        let sub_type = decrypted[local_offset];
                        local_offset += 1;

                        if sub_type == 0x00 { continue; }

                        if local_offset + 4 > decrypted.len() { break; }
                        let sub_block_size = u32::from_le_bytes(decrypted[local_offset..local_offset+4].try_into().unwrap()) as usize;
                        local_offset += 4;

                        if local_offset + sub_block_size > decrypted.len() { break; }
                        let sub_data = &decrypted[local_offset..local_offset+sub_block_size];

                        match sub_type {
                            0x05 => { // Внутренний контур компонента — выносим на отдельный слой "COMPONENT"
                                if sub_data.len() >= 20 {
                                    let x1 = (i32::from_le_bytes(sub_data[4..8].try_into().unwrap()) as f64 / 10000.0) * 250.0;
                                    let y1 = (i32::from_le_bytes(sub_data[8..12].try_into().unwrap()) as f64 / 10000.0) * 250.0;
                                    let x2 = (i32::from_le_bytes(sub_data[12..16].try_into().unwrap()) as f64 / 10000.0) * 250.0;
                                    let y2 = (i32::from_le_bytes(sub_data[16..20].try_into().unwrap()) as f64 / 10000.0) * 250.0;
                                    lines.push(GenCadLine { layer: "COMPONENT".to_string(), x1, y1, x2, y2 });
                                }
                            }
                            0x06 => { // Текст / Имя (RefDes)
                                if sub_data.len() >= 30 {
                                    let name_size = u32::from_le_bytes(sub_data[26..30].try_into().unwrap()) as usize;
                                    if 30 + name_size <= sub_data.len() {
                                        let lbl_text = String::from_utf8_lossy(&sub_data[30..30+name_size]).trim().to_string();
                                        if !lbl_text.is_empty() {
                                            ref_name = lbl_text;
                                        }
                                    }
                                }
                            }
                            0x09 => { // Пины
                                if sub_data.len() >= 24 {
                                    let mut pin_iter = 0;
                                    pin_iter += 4; // padding

                                    let pin_x = (i32::from_le_bytes(sub_data[pin_iter..pin_iter+4].try_into().unwrap()) as f64 / 10000.0) * 250.0;
                                    pin_iter += 4;
                                    let pin_y = (i32::from_le_bytes(sub_data[pin_iter..pin_iter+4].try_into().unwrap()) as f64 / 10000.0) * 250.0;
                                    pin_iter += 4;

                                    pin_iter += 4; // padding
                                    pin_iter += 4; // rotation

                                    let pin_name_size = u32::from_le_bytes(sub_data[pin_iter..pin_iter+4].try_into().unwrap()) as usize;
                                    pin_iter += 4;

                                    let mut pin_name = format!("P{}", pins.len() + 1);
                                    if pin_name_size > 0 && pin_iter + pin_name_size <= sub_data.len() {
                                        let n_str = String::from_utf8_lossy(&sub_data[pin_iter..pin_iter+pin_name_size]).trim().to_string();
                                        if !n_str.is_empty() {
                                            pin_name = n_str;
                                        }
                                    }

                                    let net_id = if sub_data.len() >= 12 {
                                        u32::from_le_bytes(sub_data[sub_data.len()-12..sub_data.len()-8].try_into().unwrap())
                                    } else {
                                        0
                                    };

                                    pins.push(BoardPin {
                                        name: format!("{}.{}", ref_name, pin_name),
                                        x: pin_x,
                                        y: pin_y,
                                        net: format!("NET_{}", net_id),
                                        side: "top".to_string(),
                                    });
                                }
                            }
                            _ => {}
                        }

                        local_offset += sub_block_size;
                    }

                    components.push(GenCadComponent {
                        ref_name,
                        x: part_x,
                        y: part_y,
                        shape: "".to_string(),
                        layer: "TOP".to_string(),
                        bounds_x1: part_x - 3.0,
                        bounds_y1: part_y - 3.0,
                        bounds_x2: part_x + 3.0,
                        bounds_y2: part_y + 3.0,
                        rotation: part_rotation,
                        mirror: false,
                        device: "".to_string(),
                    });
                }
            }
            _ => {}
        }
        current_offset += block_size;
    }

    Ok(BoardData {
        format: "pcb".to_string(),
        pins,
        lines,
        arcs: vec![],
        components,
        pads: vec![],
        routes: vec![],
    })
}

// ==========================================
// 5. TAURI КОМАНДЫ
// ==========================================
#[tauri::command]
pub async fn read_boardview(path: String) -> Result<BoardData, String> {
    let ext = std::path::Path::new(&path).extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
    match ext.as_str() {
        "fz" | "cae" => parse_fz_from_file(&path),
        "cad" => { let text = std::fs::read_to_string(&path).map_err(|e| format!("Read error: {}", e))?; parse_cad(&text) }
        "bdv" | "asc" => parse_bdv(&path),
        "pcb" => parse_xzz(&path),
        "brd" =>  {
            let raw = std::fs::read(&path).map_err(|e| format!("Read error: {}", e))?;
            if raw.len() >= 4 && &raw[0..4] == b"\x23\xE2\x63\x28" {
                return parse_apple_brd(&path);
            }
            if let Ok(text) = std::str::from_utf8(&raw) {
                if text.contains("BRDOUT:") && text.contains("NETS:") {
                    return parse_brd2(&path);
                }
            }
            Err("Unsupported .brd format (not Apple, not BRD2)".to_string())
        }
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

    // 4. XZZ (PCB)
    if buffer.len() >= 6 {
        let mut header_check = buffer[0..6].to_vec();
        if buffer.len() > 0x10 {
            let xor_key = buffer[0x10];
            if xor_key != 0 {
                for b in header_check.iter_mut() { *b ^= xor_key; }
            }
        }
        if header_check == b"XZZPCB" {
            if let Ok(cache_dir) = app.path().app_cache_dir() {
                let temp_path = cache_dir.join(format!("canifol_temp_{}.pcb", std::process::id()));
                if let Ok(_) = std::fs::write(&temp_path, &buffer) {
                    let result = parse_xzz(temp_path.to_str().unwrap_or(""));
                    let _ = std::fs::remove_file(&temp_path);
                    if let Ok(data) = result { return Ok(data); }
                }
            }
        }
    }

    // 5. BRD (Apple BoardView или TOPTEST BRD2)
    if buffer.len() >= 4 {
        // 5.1. Apple BoardView (магия 23 E2 63 28)
        if &buffer[0..4] == b"\x23\xE2\x63\x28" {
            if let Ok(cache_dir) = app.path().app_cache_dir() {
                let temp_path = cache_dir.join(format!("canifol_temp_{}.brd", std::process::id()));
                if let Ok(_) = std::fs::write(&temp_path, &buffer) {
                    let result = parse_apple_brd(temp_path.to_str().unwrap_or(""));
                    let _ = std::fs::remove_file(&temp_path);
                    if let Ok(data) = result { return Ok(data); }
                }
            }
        }
        // 5.2. TOPTEST BRD2 (текст с BRDOUT: и NETS:)
        if let Ok(text) = String::from_utf8(buffer.clone()) {
            if text.contains("BRDOUT:") && text.contains("NETS:") {
                if let Ok(cache_dir) = app.path().app_cache_dir() {
                    let temp_path = cache_dir.join(format!("canifol_temp_{}.brd", std::process::id()));
                    if let Ok(_) = std::fs::write(&temp_path, &buffer) {
                        let result = parse_brd2(temp_path.to_str().unwrap_or(""));
                        let _ = std::fs::remove_file(&temp_path);
                        if let Ok(data) = result { return Ok(data); }
                    }
                }
            }
        }
    }

    // 6. FZ (сохраняем во временный файл)
    if let Ok(cache_dir) = app.path().app_cache_dir() {
        let temp_path = cache_dir.join(format!("canifol_temp_{}.fz", std::process::id()));
        if let Ok(_) = std::fs::write(&temp_path, &buffer) {
            let result = parse_fz_from_file(temp_path.to_str().unwrap_or(""));
            let _ = std::fs::remove_file(&temp_path);
            if let Ok(data) = result { return Ok(data); }
        }
    }

    Err("Unsupported format on Android".to_string())
}