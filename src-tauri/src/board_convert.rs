//! Конвертер `crate::board::Board` (новый формат) -> `BoardData` (для фронтенда).
//!
//! `GenCadPad` и `GenCadRoute` в Canifol имеют поля `x1,y1,x2,y2` (не width/height),
//! поэтому пады разворачиваем в прямоугольник по центру пина.

use crate::board::Board;
use crate::boardview::{BoardData, BoardPin};
use crate::gencad_parser::{GenCadArc, GenCadComponent, GenCadLine, GenCadPad, GenCadRoute};
use crate::model::Side;

fn side_str(s: Side) -> String {
    match s {
        Side::Top => "top".into(),
        Side::Bottom => "bottom".into(),
        Side::Both => "both".into(),
    }
}

fn layer_str(s: Side) -> String {
    match s {
        Side::Top => "TOP".into(),
        Side::Bottom => "BOTTOM".into(),
        Side::Both => "BOTH".into(),
    }
}

pub fn board_to_boarddata(board: Board, format: &str) -> BoardData {
    // --- Пины ---
    let mut pins = Vec::with_capacity(board.pins.len());
    for pin in &board.pins {
        let part_name = board.parts.get(pin.part_id)
            .map(|p| p.name.as_str())
            .unwrap_or("?");
        pins.push(BoardPin {
            name: format!("{}.{}", part_name, pin.name),
            x: pin.pos.x,
            y: pin.pos.y,
            net: pin.net_name.clone(),
            side: side_str(pin.side),
        });
    }

    // --- Компоненты ---
    let components = board.parts.iter().map(|p| {
        let (x1, y1, x2, y2) = if let Some(b) = p.bounding_box {
            (b.min_x, b.min_y, b.max_x, b.max_y)
        } else {
            (p.p1.x, p.p1.y, p.p2.x, p.p2.y)
        };
        GenCadComponent {
            ref_name: p.name.clone(),
            x: (x1 + x2) / 2.0,
            y: (y1 + y2) / 2.0,
            shape: p.mfg_code.clone(),
            layer: layer_str(p.side),
            bounds_x1: x1,
            bounds_y1: y1,
            bounds_x2: x2,
            bounds_y2: y2,
            rotation: p.rotation_deg,
            mirror: false,
            device: p.mfg_code.clone(),
        }
    }).collect();

    // --- Линии контура платы ---
    let mut lines = Vec::with_capacity(board.outline_polygon.len());
    if board.outline_polygon.len() >= 2 {
        for w in board.outline_polygon.windows(2) {
            lines.push(GenCadLine {
                x1: w[0].x, y1: w[0].y,
                x2: w[1].x, y2: w[1].y,
                layer: "BOARD".into(),
            });
        }
        let first = board.outline_polygon[0];
        let last = *board.outline_polygon.last().unwrap();
        if (first.x - last.x).abs() > 1e-9 || (first.y - last.y).abs() > 1e-9 {
            lines.push(GenCadLine {
                x1: last.x, y1: last.y,
                x2: first.x, y2: first.y,
                layer: "BOARD".into(),
            });
        }
    }

    // --- Пады (как прямоугольники вокруг пина) ---
    let mut pads = Vec::with_capacity(board.pins.len());
    for pin in &board.pins {
        if let Some((w, h)) = pin.pad_size() {
            let hw = w / 2.0;
            let hh = h / 2.0;
            pads.push(GenCadPad {
                shape: format!("{:?}", pin.shape).to_uppercase(),
                x1: pin.pos.x - hw,
                y1: pin.pos.y - hh,
                x2: pin.pos.x + hw,
                y2: pin.pos.y + hh,
                layer: layer_str(pin.side),
            });
        }
    }

    // --- Виа как routes (короткие отрезки нулевой длины, чтобы были видны) ---
    let routes: Vec<GenCadRoute> = board.vias.iter().map(|v| GenCadRoute {
        x1: v.pos.x,
        y1: v.pos.y,
        x2: v.pos.x,
        y2: v.pos.y,
        layer: layer_str(v.side),
    }).collect();

    // arcs — пустой массив (форматы не всегда дают дуги в новом Board)
    let arcs: Vec<GenCadArc> = Vec::new();

    BoardData {
        format: format.to_string(),
        pins,
        lines,
        arcs,
        components,
        pads,
        routes,
    }
}