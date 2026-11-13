use std::collections::BTreeMap;
use crate::model::{Net, Part, Pin, Point, Rect, Side, Via};

#[derive(Debug, Clone, Default)]
pub struct Board {
    pub parts: Vec<Part>,
    pub pins: Vec<Pin>,
    pub vias: Vec<Via>,
    pub outline_polygon: Vec<Point>,
    pub bounding_box: Option<Rect>,
    pub nets: BTreeMap<String, Net>,
    pub part_by_name: BTreeMap<String, usize>,
}

impl Board {
    pub fn new() -> Self { Self::default() }

    pub fn rebuild_indices(&mut self) {
        self.part_by_name.clear();
        for (idx, part) in self.parts.iter().enumerate() {
            self.part_by_name.insert(part.name.to_ascii_uppercase(), idx);
        }
        self.nets.clear();
        for (idx, pin) in self.pins.iter().enumerate() {
            if !pin.net_name.is_empty() {
                let entry = self.nets.entry(pin.net_name.clone()).or_insert_with(|| Net {
                    name: pin.net_name.clone(), pin_indices: Vec::new(), via_indices: Vec::new(),
                });
                entry.pin_indices.push(idx);
            }
        }
        for (idx, via) in self.vias.iter().enumerate() {
            if !via.net_name.is_empty() {
                let entry = self.nets.entry(via.net_name.clone()).or_insert_with(|| Net {
                    name: via.net_name.clone(), pin_indices: Vec::new(), via_indices: Vec::new(),
                });
                entry.via_indices.push(idx);
            }
        }
        for part in &mut self.parts { part.pin_indices.clear(); }
        for (pin_idx, pin) in self.pins.iter().enumerate() {
            if pin.part_id < self.parts.len() {
                self.parts[pin.part_id].pin_indices.push(pin_idx);
            }
        }
        self.recompute_bounds();
    }

    pub fn recompute_bounds(&mut self) {
        let mut pts = Vec::new();
        for p in &self.outline_polygon { pts.push(p); }
        for pin in &self.pins { pts.push(&pin.pos); }
        for via in &self.vias { pts.push(&via.pos); }
        if let Some(mut bbox) = Rect::from_points(pts) {
            if bbox.width() < 10.0 { bbox.max_x += 10.0; }
            if bbox.height() < 10.0 { bbox.max_y += 10.0; }
            self.bounding_box = Some(bbox);
        } else {
            self.bounding_box = None;
        }
    }
}