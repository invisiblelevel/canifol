use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize, Clone, Debug)]
pub struct GenCadLine {
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
    pub layer: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct GenCadArc {
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
    pub x3: f64,
    pub y3: f64,
    pub layer: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct GenCadPin {
    pub name: String,
    pub x: f64,
    pub y: f64,
    pub net: String,
    pub side: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct GenCadPad {
    pub shape: String,
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
    pub layer: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct GenCadRoute {
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
    pub layer: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct GenCadComponent {
    pub ref_name: String,
    pub x: f64,
    pub y: f64,
    pub shape: String,
    pub layer: String,
    pub bounds_x1: f64,
    pub bounds_y1: f64,
    pub bounds_x2: f64,
    pub bounds_y2: f64,
    pub rotation: f64,
    pub mirror: bool,
    pub device: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct GenCadBoard {
    pub lines: Vec<GenCadLine>,
    pub arcs: Vec<GenCadArc>,
    pub pins: Vec<GenCadPin>,
    pub components: Vec<GenCadComponent>,
    pub pads: Vec<GenCadPad>,
    pub routes: Vec<GenCadRoute>,
    pub units: f64,
}

#[derive(Clone)]
struct ShapePin {
    name: String,
    x: f64,
    y: f64,
}

struct ComponentRaw {
    ref_name: String,
    x: f64,
    y: f64,
    shape: String,
    layer: String,
    rotation: f64,
    mirror: bool,
    device: String,
}

pub fn parse_gencad(text: &str) -> Result<GenCadBoard, String> {
    let mut lines = Vec::new();
    let mut arcs = Vec::new();
    let mut units = 1000.0;
    let mut current_layer = "TOP".to_string();

    let mut shapes: HashMap<String, Vec<ShapePin>> = HashMap::new();
    let mut components_raw: Vec<ComponentRaw> = Vec::new();
    let mut signals: HashMap<(String, String), String> = HashMap::new();
    let mut pads: Vec<GenCadPad> = Vec::new();
    let mut routes: Vec<GenCadRoute> = Vec::new();
    let mut devices: HashMap<String, String> = HashMap::new();

    let mut section = String::new();
    let mut current_shape: Option<String> = None;
    let mut current_component: Option<ComponentRaw> = None;
    let mut current_signal: Option<String> = None;
    let mut current_pad: Option<(String, String)> = None;
    let mut current_device: Option<String> = None;

    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with(';') { continue; }

        if line.starts_with('$') {
            if let Some(comp) = current_component.take() {
                components_raw.push(comp);
            }
            current_signal = None;
            current_pad = None;
            current_device = None;
            section = line.to_string();
            continue;
        }

        match section.as_str() {
            "$HEADER" => {
                if line.starts_with("UNITS USER") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if let Some(val) = parts.get(2) {
                        units = val.parse().unwrap_or(1000.0);
                    }
                }
            }
            "$BOARD" => {
                if line.starts_with("ARTWORK ") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if let Some(layer) = parts.get(2) {
                        current_layer = layer.to_string();
                    }
                } else if line.starts_with("LINE ") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 5 {
                        let x1 = parts[1].parse().unwrap_or(0.0);
                        let y1 = parts[2].parse().unwrap_or(0.0);
                        let x2 = parts[3].parse().unwrap_or(0.0);
                        let y2 = parts[4].parse().unwrap_or(0.0);
                        lines.push(GenCadLine { x1, y1, x2, y2, layer: current_layer.clone() });
                    }
                } else if line.starts_with("ARC ") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 7 {
                        let x1 = parts[1].parse().unwrap_or(0.0);
                        let y1 = parts[2].parse().unwrap_or(0.0);
                        let x2 = parts[3].parse().unwrap_or(0.0);
                        let y2 = parts[4].parse().unwrap_or(0.0);
                        let x3 = parts[5].parse().unwrap_or(0.0);
                        let y3 = parts[6].parse().unwrap_or(0.0);
                        arcs.push(GenCadArc { x1, y1, x2, y2, x3, y3, layer: current_layer.clone() });
                    }
                }
            }
            "$PADS" => {
                if line.starts_with("PAD ") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 3 {
                        current_pad = Some((parts[1].to_string(), parts[2].to_string()));
                    }
                } else if line.starts_with("RECTANGLE ") {
                    if let Some((shape, layer)) = &current_pad {
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        if parts.len() >= 5 {
                            let w = parts[3].parse().unwrap_or(0.0);
                            let h = parts[4].parse().unwrap_or(0.0);
                            let half_w = w / 2.0;
                            let half_h = h / 2.0;
                            pads.push(GenCadPad {
                                shape: shape.clone(),
                                x1: -half_w, y1: -half_h,
                                x2: half_w, y2: half_h,
                                layer: layer.clone(),
                            });
                        }
                    }
                } else if line.starts_with("CIRCLE ") {
                    if let Some((shape, layer)) = &current_pad {
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        if parts.len() >= 4 {
                            let r = parts[3].parse().unwrap_or(0.0);
                            pads.push(GenCadPad {
                                shape: shape.clone(),
                                x1: -r, y1: -r,
                                x2: r, y2: r,
                                layer: layer.clone(),
                            });
                        }
                    }
                }
            }
            // ИСПРАВЛЕНИЕ: парсим $ROUTES как LINE (в GenCAD строки там LINE)
            "$ROUTES" => {
                if line.starts_with("LINE ") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 5 {
                        let x1 = parts[1].parse().unwrap_or(0.0);
                        let y1 = parts[2].parse().unwrap_or(0.0);
                        let x2 = parts[3].parse().unwrap_or(0.0);
                        let y2 = parts[4].parse().unwrap_or(0.0);
                        routes.push(GenCadRoute { x1, y1, x2, y2, layer: current_layer.clone() });
                    }
                }
            }
            "$SHAPES" => {
                if line.starts_with("SHAPE ") {
                    let shape_name = line.strip_prefix("SHAPE ").unwrap_or("").trim().to_string();
                    shapes.insert(shape_name.clone(), Vec::new());
                    current_shape = Some(shape_name);
                } else if line.starts_with("PIN ") {
                    if let Some(ref shape) = current_shape {
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        if parts.len() >= 5 {
                            let name = parts[1].to_string();
                            let x = parts[3].parse().unwrap_or(0.0);
                            let y = parts[4].parse().unwrap_or(0.0);
                            if let Some(pins) = shapes.get_mut(shape) {
                                pins.push(ShapePin { name, x, y });
                            }
                        }
                    }
                }
            }
            "$COMPONENTS" => {
                if line.starts_with("COMPONENT ") {
                    if let Some(comp) = current_component.take() {
                        components_raw.push(comp);
                    }
                    let ref_name = line.strip_prefix("COMPONENT ").unwrap_or("").trim().to_string();
                    current_component = Some(ComponentRaw {
                        ref_name,
                        x: 0.0,
                        y: 0.0,
                        shape: String::new(),
                        layer: "TOP".to_string(),
                        rotation: 0.0,
                        mirror: false,
                        device: String::new(),
                    });
                } else if line.starts_with("PLACE ") {
                    if let Some(ref mut comp) = current_component {
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        if parts.len() >= 3 {
                            comp.x = parts[1].parse().unwrap_or(0.0);
                            comp.y = parts[2].parse().unwrap_or(0.0);
                        }
                    }
                } else if line.starts_with("LAYER ") {
                    if let Some(ref mut comp) = current_component {
                        let layer = line.strip_prefix("LAYER ").unwrap_or("").trim().to_string();
                        comp.layer = layer;
                    }
                } else if line.starts_with("ROTATION ") {
                    if let Some(ref mut comp) = current_component {
                        let rot = line.strip_prefix("ROTATION ").unwrap_or("").trim().parse().unwrap_or(0.0);
                        comp.rotation = rot;
                    }
                } else if line.starts_with("SHAPE ") {
                    if let Some(ref mut comp) = current_component {
                        let rest = line.strip_prefix("SHAPE ").unwrap_or("");
                        let mut parts = rest.split_whitespace();
                        if let Some(shape_name) = parts.next() {
                            comp.shape = shape_name.to_string();
                        }
                        let rest_str: Vec<&str> = parts.collect();
                        if rest_str.iter().any(|s| s.contains("MIRRORY") || s.contains("MIRROR")) {
                            comp.mirror = true;
                        }
                    }
                } else if line.starts_with("DEVICE ") {
                    if let Some(ref mut comp) = current_component {
                        let dev = line.strip_prefix("DEVICE ").unwrap_or("").trim().to_string();
                        comp.device = dev;
                    }
                }
            }
            "$SIGNALS" => {
                if line.starts_with("SIGNAL ") {
                    let net_name = line.strip_prefix("SIGNAL ").unwrap_or("").trim().to_string();
                    current_signal = Some(net_name);
                } else if line.starts_with("NODE ") {
                    if let Some(ref net) = current_signal {
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        if parts.len() >= 3 {
                            let ref_name = parts[1].to_string();
                            let pin_name = parts[2].to_string();
                            signals.insert((ref_name, pin_name), net.clone());
                        }
                    }
                }
            }
            "$DEVICES" => {
                if line.starts_with("DEVICE ") {
                    let dev_name = line.strip_prefix("DEVICE ").unwrap_or("").trim().to_string();
                    current_device = Some(dev_name);
                } else if line.starts_with("PART ") {
                    if let Some(ref dev_name) = current_device {
                        let part = line.strip_prefix("PART ").unwrap_or("").trim().to_string();
                        devices.insert(dev_name.clone(), part);
                    }
                }
            }
            _ => {}
        }
    }

    if let Some(comp) = current_component.take() {
        components_raw.push(comp);
    }

    let mut pins = Vec::new();
    let mut components = Vec::new();

    for comp in &components_raw {
        let shape_pins = shapes.get(&comp.shape).cloned().unwrap_or_default();
        let rot_rad = comp.rotation.to_radians();
        let cos_r = rot_rad.cos();
        let sin_r = rot_rad.sin();

        let mut min_x = f64::MAX;
        let mut min_y = f64::MAX;
        let mut max_x = f64::MIN;
        let mut max_y = f64::MIN;

        for pin in &shape_pins {
            let mut px = pin.x;
            let py = pin.y;

            if comp.mirror {
                px = -px;
            }

            let rotated_x = px * cos_r - py * sin_r;
            let rotated_y = px * sin_r + py * cos_r;
            let global_x = comp.x + rotated_x;
            let global_y = comp.y + rotated_y;

            if global_x < min_x { min_x = global_x; }
            if global_y < min_y { min_y = global_y; }
            if global_x > max_x { max_x = global_x; }
            if global_y > max_y { max_y = global_y; }

            let net_key = (comp.ref_name.clone(), pin.name.clone());
            let net = signals.get(&net_key).cloned().unwrap_or_default();

            pins.push(GenCadPin {
                name: format!("{}.{}", comp.ref_name, pin.name),
                x: global_x,
                y: global_y,
                net,
                side: comp.layer.to_lowercase(),
            });
        }

        if min_x < f64::MAX {
            let padding = 1.5;
            let min_size = 2.0;
            let mut x1 = min_x - padding;
            let mut y1 = min_y - padding;
            let mut x2 = max_x + padding;
            let mut y2 = max_y + padding;

            if (x2 - x1) < min_size {
                let cx = (x1 + x2) / 2.0;
                x1 = cx - min_size / 2.0;
                x2 = cx + min_size / 2.0;
            }
            if (y2 - y1) < min_size {
                let cy = (y1 + y2) / 2.0;
                y1 = cy - min_size / 2.0;
                y2 = cy + min_size / 2.0;
            }

            components.push(GenCadComponent {
                ref_name: comp.ref_name.clone(),
                x: comp.x,
                y: comp.y,
                shape: comp.shape.clone(),
                layer: comp.layer.clone(),
                bounds_x1: x1,
                bounds_y1: y1,
                bounds_x2: x2,
                bounds_y2: y2,
                rotation: comp.rotation,
                mirror: comp.mirror,
                device: devices.get(&comp.device).cloned().unwrap_or(comp.device.clone()),
            });
        }
    }

    Ok(GenCadBoard { lines, arcs, pins, components, pads, routes, units })
}