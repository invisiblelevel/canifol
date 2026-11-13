use crate::gencad_parser::{parse_gencad, GenCadArc, GenCadComponent, GenCadLine, GenCadPad, GenCadRoute};
use crate::tvw_canifol::TvwParser as TvwCanifolParser;
use des::cipher::{generic_array::GenericArray, BlockDecrypt, KeyInit};
use des::Des;
use pcbrepair::decoder::DecodedPcbRepairFile;
use serde::Serialize;
use std::collections::{BTreeMap, HashMap};
use std::io::Cursor;

// ==========================================================================
//  Публичные структуры (контракт с фронтендом — НЕ менялись)
// ==========================================================================
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

// ==========================================================================
//  0. ОБЩИЕ ХЕЛПЕРЫ
// ==========================================================================

/// 1 mil (1/1000 дюйма) в миллиметрах.
const MIL_TO_MM: f64 = 0.0254;
/// Запас вокруг бокса компонента, собранного по пинам (мм).
const COMP_PADDING_MM: f64 = 0.3;
/// Минимальный размер бокса компонента (мм), чтобы 1-2-пиновые детали были кликабельны.
const COMP_MIN_SIZE_MM: f64 = 1.2;
/// Отступ рамки платы, которую рисуем сами, если в файле нет контура (мм).
const FALLBACK_OUTLINE_MARGIN_MM: f64 = 2.0;

#[derive(Clone, Copy, Debug)]
struct Bounds {
    min_x: f64,
    min_y: f64,
    max_x: f64,
    max_y: f64,
}

impl Bounds {
    fn empty() -> Self {
        Bounds { min_x: f64::INFINITY, min_y: f64::INFINITY, max_x: f64::NEG_INFINITY, max_y: f64::NEG_INFINITY }
    }
    fn is_empty(&self) -> bool {
        self.min_x > self.max_x || self.min_y > self.max_y
    }
    fn add(&mut self, x: f64, y: f64) {
        self.min_x = self.min_x.min(x);
        self.min_y = self.min_y.min(y);
        self.max_x = self.max_x.max(x);
        self.max_y = self.max_y.max(y);
    }
    fn add_r(&mut self, x: f64, y: f64, r: f64) {
        self.add(x - r, y - r);
        self.add(x + r, y + r);
    }
    fn width(&self) -> f64 {
        self.max_x - self.min_x
    }
    fn height(&self) -> f64 {
        self.max_y - self.min_y
    }
    fn expanded(&self, dx: f64, dy: f64) -> Bounds {
        Bounds { min_x: self.min_x - dx, min_y: self.min_y - dy, max_x: self.max_x + dx, max_y: self.max_y + dy }
    }
    fn contains(&self, x: f64, y: f64) -> bool {
        x >= self.min_x && x <= self.max_x && y >= self.min_y && y <= self.max_y
    }
}

fn pins_bounds(pins: &[BoardPin]) -> Bounds {
    let mut b = Bounds::empty();
    for p in pins {
        b.add(p.x, p.y);
    }
    b
}

/// Собирает компонент из бокса пинов (с отступом и минимальным размером).
fn make_component(
    ref_name: String,
    b: &Bounds,
    layer: &str,
    shape: String,
    device: String,
    rotation: f64,
    mirror: bool,
) -> GenCadComponent {
    let mut x1 = b.min_x - COMP_PADDING_MM;
    let mut y1 = b.min_y - COMP_PADDING_MM;
    let mut x2 = b.max_x + COMP_PADDING_MM;
    let mut y2 = b.max_y + COMP_PADDING_MM;
    if (x2 - x1) < COMP_MIN_SIZE_MM {
        let cx = (x1 + x2) / 2.0;
        x1 = cx - COMP_MIN_SIZE_MM / 2.0;
        x2 = cx + COMP_MIN_SIZE_MM / 2.0;
    }
    if (y2 - y1) < COMP_MIN_SIZE_MM {
        let cy = (y1 + y2) / 2.0;
        y1 = cy - COMP_MIN_SIZE_MM / 2.0;
        y2 = cy + COMP_MIN_SIZE_MM / 2.0;
    }
    GenCadComponent {
        ref_name,
        x: (x1 + x2) / 2.0,
        y: (y1 + y2) / 2.0,
        shape,
        layer: layer.to_string(),
        bounds_x1: x1,
        bounds_y1: y1,
        bounds_x2: x2,
        bounds_y2: y2,
        rotation,
        mirror,
        device,
    }
}

fn layer_name(is_bottom: bool) -> &'static str {
    if is_bottom { "BOTTOM" } else { "TOP" }
}

fn side_name(is_bottom: bool) -> &'static str {
    if is_bottom { "bottom" } else { "top" }
}

/// Парсит число, допускает запятую как десятичный разделитель. None при ошибке/NaN/inf.
fn parse_num(s: &str) -> Option<f64> {
    let s = s.trim();
    if s.is_empty() {
        return None;
    }
    let v = if s.contains(',') { s.replace(',', ".").parse::<f64>().ok()? } else { s.parse::<f64>().ok()? };
    if v.is_finite() { Some(v) } else { None }
}

/// Замкнутый/открытый полилайн -> линии.
fn polyline_lines(points: &[(f64, f64)], close: bool, layer: &str) -> Vec<GenCadLine> {
    let mut out = Vec::new();
    if points.len() < 2 {
        return out;
    }
    for w in points.windows(2) {
        if w[0] != w[1] {
            out.push(GenCadLine { x1: w[0].0, y1: w[0].1, x2: w[1].0, y2: w[1].1, layer: layer.to_string() });
        }
    }
    if close {
        let (f, l) = (points[0], points[points.len() - 1]);
        if f != l {
            out.push(GenCadLine { x1: l.0, y1: l.1, x2: f.0, y2: f.1, layer: layer.to_string() });
        }
    }
    out
}

fn rect_lines(b: &Bounds, margin: f64, layer: &str) -> Vec<GenCadLine> {
    let (x1, y1, x2, y2) = (b.min_x - margin, b.min_y - margin, b.max_x + margin, b.max_y + margin);
    polyline_lines(&[(x1, y1), (x2, y1), (x2, y2), (x1, y2)], true, layer)
}

/// Если у формата нет контура платы — рисуем рамку по габаритам пинов,
/// чтобы плата не «висела в воздухе».
fn ensure_outline(board: &mut BoardData) {
    if !board.lines.is_empty() || board.pins.is_empty() {
        return;
    }
    let b = pins_bounds(&board.pins);
    if b.is_empty() {
        return;
    }
    board.lines = rect_lines(&b, FALLBACK_OUTLINE_MARGIN_MM, "BOARD");
}

/// Разбивает дугу (центр + начало + конец) на отрезки.
/// Направление в формате неизвестно -> берём меньшую дугу (<=180°), что верно для
/// скруглённых углов. Если начало == конец — полная окружность.
fn tessellate_arc(x1: f64, y1: f64, x2: f64, y2: f64, cx: f64, cy: f64) -> Vec<(f64, f64, f64, f64)> {
    use std::f64::consts::PI;
    let r1 = ((x1 - cx).powi(2) + (y1 - cy).powi(2)).sqrt();
    let r2 = ((x2 - cx).powi(2) + (y2 - cy).powi(2)).sqrt();
    if r1 < 1e-9 || r2 < 1e-9 {
        return vec![(x1, y1, x2, y2)];
    }
    let a1 = (y1 - cy).atan2(x1 - cx);
    let a2 = (y2 - cy).atan2(x2 - cx);
    let full = ((x1 - x2).abs() < 1e-9) && ((y1 - y2).abs() < 1e-9);
    let mut delta = a2 - a1;
    if full {
        delta = 2.0 * PI;
    } else {
        while delta > PI { delta -= 2.0 * PI; }
        while delta <= -PI { delta += 2.0 * PI; }
    }
    let steps = ((delta.abs() / (PI / 18.0)).ceil() as usize).clamp(2, 72);
    let mut out = Vec::with_capacity(steps);
    let mut prev = (x1, y1);
    for i in 1..=steps {
        let t = i as f64 / steps as f64;
        let a = a1 + delta * t;
        let r = r1 + (r2 - r1) * t;
        let p = if i == steps && !full { (x2, y2) } else { (cx + r * a.cos(), cy + r * a.sin()) };
        out.push((prev.0, prev.1, p.0, p.1));
        prev = p;
    }
    out
}

// --- безопасное чтение little-endian (никаких паник на битых файлах) ---
fn rd_u32(d: &[u8], o: usize) -> Option<u32> {
    let end = o.checked_add(4)?;
    d.get(o..end).map(|s| u32::from_le_bytes([s[0], s[1], s[2], s[3]]))
}
fn rd_i32(d: &[u8], o: usize) -> Option<i32> {
    rd_u32(d, o).map(|v| v as i32)
}

fn contains_bytes(hay: &[u8], needle: &[u8]) -> bool {
    !needle.is_empty() && hay.windows(needle.len()).any(|w| w == needle)
}

// ==========================================================================
//  1. ПАРСЕР FZ / CAE (ASUS PCBRepair / ASRock PCBRepair Pro)
// ==========================================================================
//
//  Формат (см. исходники крейта pcbrepair, parser.rs) — текст с разделителем '!':
//
//    UNIT:millimeters                      <- если нет, единицы = mils
//    A!REFDES!COMP_INSERTION_CODE!SYM_NAME!SYM_MIRROR!SYM_ROTATE!
//    S!U1!...!QFN32!YES!90!                <- символы (компоненты), YES = нижняя сторона
//    A!NET_NAME!REFDES!PIN_NUMBER!PIN_NAME!PIN_X!PIN_Y!TEST_POINT!RADIUS!
//    S!GND!U1!1!GND!1234.5!678.9!!12!      <- пины
//    A!CLASS!SUBCLASS!GRAPHIC_DATA_NAME!...  <- графика (контур, дорожки)
//
//  ВАЖНО: ключ секции — это ПЕРВАЯ колонка заголовка (REFDES / NET_NAME / ...),
//  и она же первое значение в строках `S!`. Поэтому колонки и значения нужно
//  индексировать одинаково (от parts[1..]). Старый код терял эту колонку у
//  заголовка, из-за чего ВСЕ индексы были сдвинуты на 1.

struct FzCols(Vec<String>);

impl FzCols {
    fn idx(&self, names: &[&str], default: usize) -> usize {
        self.0
            .iter()
            .position(|c| names.iter().any(|n| c.eq_ignore_ascii_case(n)))
            .unwrap_or(default)
    }
}

fn fld<'a>(vals: &[&'a str], i: usize) -> &'a str {
    vals.get(i).map(|s| s.trim()).unwrap_or("")
}

#[derive(Clone, Copy, PartialEq)]
enum FzSection {
    Symbol,
    Pin,
    Classed,
    Other,
}

struct FzSym {
    mirror: bool,
    rotate: f64,
    sym_name: String,
}

struct FzPinRaw {
    net: String,
    refdes: String,
    number: String,
    name: String,
    x: f64,
    y: f64,
    radius: f64,
}

struct FzGfx {
    class: String,
    subclass: String,
    name: String,
    g: [Option<f64>; 9],
}

/// Описание компонентов из `description` (таблица с табами): refdes -> описание.
fn parse_fz_description(desc: &[u8]) -> HashMap<String, String> {
    let text = String::from_utf8_lossy(desc);
    let mut map = HashMap::new();
    // 1-я строка — заголовок платы, 2-я — названия колонок
    for line in text.lines().skip(2) {
        let cols: Vec<&str> = line.split('\t').collect();
        if cols.len() < 5 {
            continue;
        }
        let part_no = cols[0].trim();
        let descr = cols[1].trim();
        let device = if !descr.is_empty() { descr } else { part_no };
        if device.is_empty() {
            continue;
        }
        for r in cols[3].split_whitespace() {
            map.insert(r.to_string(), device.to_string());
        }
    }
    map
}

/// Превращает графическую запись FZ в набор отрезков (в «сырых» единицах файла).
fn fz_gfx_segments(g: &FzGfx) -> Vec<(f64, f64, f64, f64)> {
    let n = |i: usize| g.g[i];
    match g.name.as_str() {
        "LINE" => match (n(0), n(1), n(2), n(3)) {
            (Some(x1), Some(y1), Some(x2), Some(y2)) => vec![(x1, y1, x2, y2)],
            _ => vec![],
        },
        "ARC" => match (n(0), n(1), n(2), n(3), n(4), n(5)) {
            (Some(x1), Some(y1), Some(x2), Some(y2), Some(cx), Some(cy)) => tessellate_arc(x1, y1, x2, y2, cx, cy),
            _ => vec![],
        },
        "RECTANGLE" | "RECT" => match (n(0), n(1), n(2), n(3)) {
            (Some(x1), Some(y1), Some(x2), Some(y2)) => {
                vec![(x1, y1, x2, y1), (x2, y1, x2, y2), (x2, y2, x1, y2), (x1, y2, x1, y1)]
            }
            _ => vec![],
        },
        _ => vec![],
    }
}

fn parse_fz_content(content: &[u8], description: &[u8]) -> Result<BoardData, String> {
    let text = String::from_utf8_lossy(content);

    // Единицы объявляются где-то в файле и действуют на всё -> предварительный проход
    let mut is_mm = false;
    for line in text.lines() {
        let l = line.trim();
        if l.len() > 5 && l[..5].eq_ignore_ascii_case("UNIT:") {
            let first = l.split('!').next().unwrap_or("");
            if first.eq_ignore_ascii_case("UNIT:millimeters") {
                is_mm = true;
            }
        }
    }
    let scale = if is_mm { 1.0 } else { MIL_TO_MM };

    let mut section = FzSection::Other;
    let mut cols = FzCols(Vec::new());
    let mut syms: HashMap<String, FzSym> = HashMap::new();
    let mut pins_raw: Vec<FzPinRaw> = Vec::new();
    let mut gfx: Vec<FzGfx> = Vec::new();

    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split('!').collect();
        match parts[0].trim() {
            "A" => {
                // Заголовок: колонки = parts[1..], ПЕРВАЯ колонка = ключ секции
                cols = FzCols(parts[1..].iter().map(|s| s.trim().to_uppercase()).collect());
                section = match cols.0.first().map(|s| s.as_str()) {
                    Some("REFDES") => FzSection::Symbol,
                    Some("NET_NAME") => FzSection::Pin,
                    Some("CLASS") => FzSection::Classed,
                    _ => FzSection::Other,
                };
            }
            "S" => {
                let vals: Vec<&str> = parts[1..].to_vec();
                match section {
                    FzSection::Symbol => {
                        let i_ref = cols.idx(&["REFDES"], 0);
                        let i_name = cols.idx(&["SYM_NAME"], 2);
                        let i_mir = cols.idx(&["SYM_MIRROR"], 3);
                        let i_rot = cols.idx(&["SYM_ROTATE"], 4);
                        let refdes = fld(&vals, i_ref);
                        if refdes.is_empty() {
                            continue;
                        }
                        syms.insert(
                            refdes.to_string(),
                            FzSym {
                                mirror: fld(&vals, i_mir).eq_ignore_ascii_case("YES"),
                                rotate: parse_num(fld(&vals, i_rot)).unwrap_or(0.0),
                                sym_name: fld(&vals, i_name).to_string(),
                            },
                        );
                    }
                    FzSection::Pin => {
                        let i_net = cols.idx(&["NET_NAME"], 0);
                        let i_ref = cols.idx(&["REFDES"], 1);
                        let i_num = cols.idx(&["PIN_NUMBER"], 2);
                        let i_pname = cols.idx(&["PIN_NAME"], 3);
                        let i_x = cols.idx(&["PIN_X"], 4);
                        let i_y = cols.idx(&["PIN_Y"], 5);
                        let i_rad = cols.idx(&["RADIUS"], 7);
                        let (x, y) = match (parse_num(fld(&vals, i_x)), parse_num(fld(&vals, i_y))) {
                            (Some(x), Some(y)) => (x, y),
                            _ => continue, // битая строка — пропускаем, а не рисуем в (0,0)
                        };
                        let refdes = fld(&vals, i_ref);
                        if refdes.is_empty() && x == 0.0 && y == 0.0 {
                            continue;
                        }
                        pins_raw.push(FzPinRaw {
                            net: fld(&vals, i_net).to_string(),
                            refdes: refdes.to_string(),
                            number: fld(&vals, i_num).to_string(),
                            name: fld(&vals, i_pname).to_string(),
                            x,
                            y,
                            radius: parse_num(fld(&vals, i_rad)).unwrap_or(0.0).abs(),
                        });
                    }
                    FzSection::Classed => {
                        let i_class = cols.idx(&["CLASS"], 0);
                        let i_sub = cols.idx(&["SUBCLASS"], 1);
                        let i_name = cols.idx(&["GRAPHIC_DATA_NAME"], 2);
                        let i_g1 = cols.idx(&["GRAPHIC_DATA_1"], 5);
                        let mut g = [None; 9];
                        for (k, slot) in g.iter_mut().enumerate() {
                            *slot = parse_num(fld(&vals, i_g1 + k));
                        }
                        gfx.push(FzGfx {
                            class: fld(&vals, i_class).to_uppercase(),
                            subclass: fld(&vals, i_sub).to_uppercase(),
                            name: fld(&vals, i_name).to_uppercase(),
                            g,
                        });
                    }
                    FzSection::Other => {}
                }
            }
            _ => {}
        }
    }

    if pins_raw.is_empty() {
        return Err("FZ: в файле не найдено ни одного пина (секция NET_NAME пуста)".to_string());
    }

    let descriptions = parse_fz_description(description);

    // ---- пины ----
    let mut pins = Vec::with_capacity(pins_raw.len());
    let mut by_ref: BTreeMap<String, Bounds> = BTreeMap::new();
    let mut pin_counter: HashMap<String, usize> = HashMap::new();
    for p in &pins_raw {
        let is_bottom = syms.get(&p.refdes).map(|s| s.mirror).unwrap_or(false);
        let (x, y, r) = (p.x * scale, p.y * scale, p.radius * scale);
        let cnt = pin_counter.entry(p.refdes.clone()).or_insert(0);
        *cnt += 1;
        // как в интерпретаторе крейта: номер пина, а если он пустой/"0" — имя пина
        let number = match p.number.as_str() {
            "" | "0" if !p.name.is_empty() => p.name.clone(),
            "" | "0" => cnt.to_string(),
            n => n.to_string(),
        };
        let refdes = if p.refdes.is_empty() { "NOREF" } else { p.refdes.as_str() };
        pins.push(BoardPin {
            name: format!("{}.{}", refdes, number),
            x,
            y,
            net: p.net.clone(),
            side: side_name(is_bottom).to_string(),
        });
        by_ref.entry(refdes.to_string()).or_insert_with(Bounds::empty).add_r(x, y, r);
    }

    // ---- компоненты ----
    let mut components = Vec::with_capacity(by_ref.len());
    for (refdes, b) in &by_ref {
        let sym = syms.get(refdes);
        let is_bottom = sym.map(|s| s.mirror).unwrap_or(false);
        components.push(make_component(
            refdes.clone(),
            b,
            layer_name(is_bottom),
            sym.map(|s| s.sym_name.clone()).unwrap_or_default(),
            descriptions.get(refdes).cloned().unwrap_or_default(),
            sym.map(|s| s.rotate).unwrap_or(0.0),
            is_bottom,
        ));
    }

    // ---- графика: контур платы и дорожки (best-effort, с проверкой адекватности) ----
    let pb = pins_bounds(&pins);
    let mut outline_pts: Vec<(f64, f64, f64, f64)> = Vec::new();
    let mut route_segs: Vec<(f64, f64, f64, f64, String)> = Vec::new();
    for g in &gfx {
        let is_outline = g.class.contains("BOARD") && g.subclass.contains("OUTLINE");
        let is_etch = g.class == "ETCH";
        if !is_outline && !is_etch {
            continue;
        }
        for (x1, y1, x2, y2) in fz_gfx_segments(g) {
            let seg = (x1 * scale, y1 * scale, x2 * scale, y2 * scale);
            if is_outline {
                outline_pts.push(seg);
            } else {
                route_segs.push((seg.0, seg.1, seg.2, seg.3, g.subclass.clone()));
            }
        }
    }

    let mut lines: Vec<GenCadLine> = Vec::new();
    if !outline_pts.is_empty() {
        let mut ob = Bounds::empty();
        for s in &outline_pts {
            ob.add(s.0, s.1);
            ob.add(s.2, s.3);
        }
        // контур обязан охватывать пины и быть соразмерным плате, иначе это мусор
        let inside = pins.iter().filter(|p| ob.expanded(ob.width() * 0.01, ob.height() * 0.01).contains(p.x, p.y)).count();
        let sane_size = ob.width() <= pb.width().max(1.0) * 6.0 && ob.height() <= pb.height().max(1.0) * 6.0;
        if inside * 100 >= pins.len() * 95 && sane_size {
            for s in &outline_pts {
                lines.push(GenCadLine { x1: s.0, y1: s.1, x2: s.2, y2: s.3, layer: "BOARD".to_string() });
            }
        }
    }

    let mut routes: Vec<GenCadRoute> = Vec::new();
    if !route_segs.is_empty() {
        let area = pb.expanded(pb.width() * 0.15 + 5.0, pb.height() * 0.15 + 5.0);
        let ok = route_segs.iter().filter(|s| area.contains(s.0, s.1) && area.contains(s.2, s.3)).count();
        if ok * 100 >= route_segs.len() * 90 {
            for s in route_segs {
                let layer = match s.4.as_str() {
                    "" => "TOP".to_string(),
                    other => other.to_string(),
                };
                routes.push(GenCadRoute { x1: s.0, y1: s.1, x2: s.2, y2: s.3, layer });
            }
        }
    }

    let mut board =
        BoardData { format: "fz".to_string(), pins, lines, arcs: vec![], components, pads: vec![], routes };
    ensure_outline(&mut board);
    Ok(board)
}

fn parse_fz_bytes(bytes: &[u8]) -> Result<BoardData, String> {
    let decoded = DecodedPcbRepairFile::new(Cursor::new(bytes)).map_err(|e| format!("FZ decode error: {}", e))?;
    parse_fz_content(&decoded.content, &decoded.description)
}

// ==========================================================================
//  2. ПАРСЕР CAD (GenCAD)
// ==========================================================================
fn parse_cad(text: &str) -> Result<BoardData, String> {
    let gencad = parse_gencad(text)?;
    let pins = gencad
        .pins
        .iter()
        .map(|p| BoardPin { name: p.name.clone(), x: p.x, y: p.y, net: p.net.clone(), side: p.side.clone() })
        .collect();
    Ok(BoardData {
        format: "cad".to_string(),
        pins,
        lines: gencad.lines,
        arcs: gencad.arcs,
        components: gencad.components,
        pads: gencad.pads,
        routes: gencad.routes,
    })
}

// ==========================================================================
//  3. ПАРСЕР BRD (шифрованный, 0x23E26328)
// ==========================================================================
//  Секции: Format (контур платы), Parts/Pins1 (name type end_of_pins),
//          Pins/Pins2 (x y probe part net), Nails. Координаты — mils.

const APPLE_BRD_MAGIC: &[u8] = b"\x23\xE2\x63\x28";

fn decode_apple_brd(raw: &[u8]) -> String {
    let mut data = raw.to_vec();
    for b in data.iter_mut() {
        if *b != b'\r' && *b != b'\n' && *b != 0 {
            let c = *b as i8 as i32;
            let shifted = ((c >> 6) & 3) | (c << 2);
            *b = (!shifted & 0xFF) as u8;
        }
    }
    String::from_utf8_lossy(&data).into_owned()
}

struct BrdPart {
    name: String,
    kind: u32,
    end_of_pins: usize,
}

struct BrdPinRaw {
    x: f64,
    y: f64,
    part: usize,
    net: String,
}

fn parse_apple_brd_text(text: &str) -> Result<BoardData, String> {
    #[derive(PartialEq, Clone, Copy)]
    enum Sec {
        None,
        Format,
        Parts,
        Pins,
        Other,
    }
    let mut sec = Sec::None;
    let mut outline: Vec<(f64, f64)> = Vec::new();
    let mut parts: Vec<BrdPart> = Vec::new();
    let mut pins_raw: Vec<BrdPinRaw> = Vec::new();

    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let l = line.to_ascii_lowercase();
        if l.starts_with("str_length:") || l.starts_with("var_data:") {
            sec = Sec::Other;
            continue;
        }
        if l.starts_with("format:") {
            sec = Sec::Format;
            continue;
        }
        if l.starts_with("parts:") || l.starts_with("pins1:") {
            sec = Sec::Parts;
            continue;
        }
        if l.starts_with("pins2:") || l.starts_with("pins:") {
            sec = Sec::Pins;
            continue;
        }
        if l.starts_with("nails:") {
            sec = Sec::Other;
            continue;
        }

        let t: Vec<&str> = line.split_whitespace().collect();
        match sec {
            Sec::Format => {
                if t.len() >= 2 {
                    if let (Some(x), Some(y)) = (parse_num(t[0]), parse_num(t[1])) {
                        outline.push((x * MIL_TO_MM, y * MIL_TO_MM));
                    }
                }
            }
            Sec::Parts => {
                if t.len() >= 3 {
                    parts.push(BrdPart {
                        name: t[0].to_string(),
                        kind: t[1].parse().unwrap_or(0),
                        end_of_pins: t[2].parse().unwrap_or(0),
                    });
                }
            }
            Sec::Pins => {
                if t.len() >= 4 {
                    if let (Some(x), Some(y)) = (parse_num(t[0]), parse_num(t[1])) {
                        pins_raw.push(BrdPinRaw {
                            x: x * MIL_TO_MM,
                            y: y * MIL_TO_MM,
                            part: t[3].parse().unwrap_or(0),
                            net: t.get(4).map(|s| s.to_string()).unwrap_or_default(),
                        });
                    }
                }
            }
            _ => {}
        }
    }

    if pins_raw.is_empty() {
        return Err("BRD: не найдено ни одного пина".to_string());
    }

    // Индексы деталей у пинов бывают 1-based (обычно) или 0-based
    let zero_based = pins_raw.iter().any(|p| p.part == 0) && !parts.is_empty();
    let part_of_pin = |i: usize, p: &BrdPinRaw| -> Option<usize> {
        if !parts.is_empty() {
            let idx = if zero_based { Some(p.part) } else { p.part.checked_sub(1) };
            if let Some(ix) = idx {
                if ix < parts.len() {
                    return Some(ix);
                }
            }
            // запасной вариант: по end_of_pins (1-based номер пина)
            return parts.iter().position(|pt| pt.end_of_pins >= i + 1);
        }
        None
    };

    let mut pins = Vec::with_capacity(pins_raw.len());
    let mut per_part: BTreeMap<String, (Bounds, bool)> = BTreeMap::new();
    let mut counters: HashMap<String, usize> = HashMap::new();
    for (i, p) in pins_raw.iter().enumerate() {
        let (ref_name, is_bottom) = match part_of_pin(i, p) {
            Some(ix) => (parts[ix].name.clone(), (parts[ix].kind & 8) != 0), // 8 = нижняя сторона (SMD bottom)
            None => (format!("P{}", p.part), false),
        };
        let c = counters.entry(ref_name.clone()).or_insert(0);
        *c += 1;
        let net = if p.net.eq_ignore_ascii_case("UNCONNECTED") { String::new() } else { p.net.clone() };
        pins.push(BoardPin {
            name: format!("{}.{}", ref_name, c),
            x: p.x,
            y: p.y,
            net,
            side: side_name(is_bottom).to_string(),
        });
        let e = per_part.entry(ref_name).or_insert((Bounds::empty(), is_bottom));
        e.0.add(p.x, p.y);
    }

    let components = per_part
        .into_iter()
        .map(|(name, (b, bottom))| make_component(name, &b, layer_name(bottom), String::new(), String::new(), 0.0, false))
        .collect();

    let lines = polyline_lines(&outline, true, "BOARD");
    let mut board =
        BoardData { format: "apple_brd".to_string(), pins, lines, arcs: vec![], components, pads: vec![], routes: vec![] };
    ensure_outline(&mut board);
    Ok(board)
}

fn parse_apple_brd_bytes(raw: &[u8]) -> Result<BoardData, String> {
    if !raw.starts_with(APPLE_BRD_MAGIC) {
        return Err("Not an Apple BRD file (wrong magic)".to_string());
    }
    parse_apple_brd_text(&decode_apple_brd(raw))
}

// ==========================================================================
//  4. ПАРСЕР BRD2 (текстовый: BRDOUT / NETS / PARTS / PINS)
// ==========================================================================
fn parse_brd2_text(content: &str) -> Result<BoardData, String> {
    let mut nets: HashMap<usize, String> = HashMap::new();
    // (name, x1, y1, x2, y2, first_pin, is_top)
    let mut parts_raw: Vec<(String, f64, f64, f64, f64, usize, bool)> = Vec::new();
    // (x, y, net_idx, is_top)
    let mut pins_raw: Vec<(f64, f64, usize, bool)> = Vec::new();
    let mut outline: Vec<(f64, f64)> = Vec::new();
    let mut sec = "";

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if line.starts_with("BRDOUT:") {
            sec = "BRDOUT";
            continue;
        } else if line.starts_with("NETS:") {
            sec = "NETS";
            continue;
        } else if line.starts_with("PARTS:") {
            sec = "PARTS";
            continue;
        } else if line.starts_with("PINS:") {
            sec = "PINS";
            continue;
        } else if line.starts_with("NAILS:") {
            sec = "NAILS";
            continue;
        }

        let t: Vec<&str> = line.split_whitespace().collect();
        match sec {
            "BRDOUT" => {
                // точки контура платы
                if t.len() >= 2 {
                    if let (Some(x), Some(y)) = (parse_num(t[0]), parse_num(t[1])) {
                        outline.push((x, y));
                    }
                }
            }
            "NETS" => {
                if t.len() >= 2 {
                    if let Ok(idx) = t[0].parse::<usize>() {
                        nets.insert(idx, t[1].to_string());
                    }
                }
            }
            "PARTS" => {
                if t.len() >= 7 {
                    if let (Some(x1), Some(y1), Some(x2), Some(y2)) =
                        (parse_num(t[1]), parse_num(t[2]), parse_num(t[3]), parse_num(t[4]))
                    {
                        let start = t[5].parse().unwrap_or(0);
                        parts_raw.push((t[0].to_string(), x1, y1, x2, y2, start, t[6] == "1"));
                    }
                }
            }
            "PINS" => {
                if t.len() >= 4 {
                    if let (Some(x), Some(y)) = (parse_num(t[0]), parse_num(t[1])) {
                        pins_raw.push((x, y, t[2].parse().unwrap_or(0), t[3] == "1"));
                    }
                }
            }
            _ => {}
        }
    }

    if parts_raw.is_empty() || pins_raw.is_empty() {
        return Err("BRD2: нет секций PARTS/PINS".to_string());
    }

    let mut pins = Vec::with_capacity(pins_raw.len());
    let mut components = Vec::with_capacity(parts_raw.len());
    let mut g = 0usize;
    for (i, (name, x1, y1, x2, y2, _start, top)) in parts_raw.iter().enumerate() {
        // количество пинов детали = разница стартовых индексов соседних деталей
        let count = if i + 1 < parts_raw.len() {
            parts_raw[i + 1].5.saturating_sub(parts_raw[i].5)
        } else {
            pins_raw.len().saturating_sub(g)
        };
        let (lx, hx) = (x1.min(*x2), x1.max(*x2));
        let (ly, hy) = (y1.min(*y2), y1.max(*y2));
        components.push(GenCadComponent {
            ref_name: name.clone(),
            x: (lx + hx) / 2.0,
            y: (ly + hy) / 2.0,
            shape: String::new(),
            layer: layer_name(!*top).to_string(),
            bounds_x1: lx,
            bounds_y1: ly,
            bounds_x2: hx,
            bounds_y2: hy,
            rotation: 0.0,
            mirror: false,
            device: String::new(),
        });
        for p in 0..count {
            let Some((px, py, net_idx, pin_top)) = pins_raw.get(g) else { break };
            pins.push(BoardPin {
                name: format!("{}.{}", name, p + 1),
                x: *px,
                y: *py,
                net: nets.get(net_idx).cloned().unwrap_or_default(),
                side: side_name(!*pin_top).to_string(),
            });
            g += 1;
        }
    }

    let lines = polyline_lines(&outline, true, "BOARD");
    let mut board =
        BoardData { format: "brd2".to_string(), pins, lines, arcs: vec![], components, pads: vec![], routes: vec![] };
    ensure_outline(&mut board);
    Ok(board)
}

// ==========================================================================
//  5. ПАРСЕР BDV
// ==========================================================================
const BDV_SIGNATURE: &[u8] = b"dd:1.3?,r?-=bb";

fn decode_bdv(bytes: &[u8]) -> String {
    let mut data = bytes.to_vec();
    if data.starts_with(BDV_SIGNATURE) {
        let mut count: u16 = 0xA0;
        let mut i = 0;
        while i < data.len() {
            let byte = data[i];
            if byte == b'\r' || byte == b'\n' || byte == 0 {
                if byte == b'\r' && i + 1 < data.len() && data[i + 1] == b'\n' {
                    count += 1;
                    if count > 285 {
                        count = 159;
                    }
                }
                i += 1;
                continue;
            }
            data[i] = (count as u8).wrapping_sub(byte);
            i += 1;
        }
    }
    String::from_utf8_lossy(&data).into_owned()
}

fn parse_bdv_from_content(content: &str) -> Result<BoardData, String> {
    let mut pins = Vec::new();
    let mut by_ref: BTreeMap<String, (Bounds, bool)> = BTreeMap::new();
    let mut current_part: Option<(String, bool)> = None; // (name, is_bottom)
    let mut in_pins_section = false;

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if line.starts_with("<<pins.asc>>") {
            in_pins_section = true;
            continue;
        }
        if line.starts_with("<<") {
            in_pins_section = false;
            continue;
        }
        if !in_pins_section {
            continue;
        }
        if line.starts_with("Part ") {
            let t: Vec<&str> = line.split_whitespace().collect();
            if t.len() >= 3 {
                let side = t[2].trim_matches(|c| c == '(' || c == ')');
                let is_bottom = !side.eq_ignore_ascii_case("T");
                current_part = Some((t[1].to_string(), is_bottom));
                by_ref.entry(t[1].to_string()).or_insert((Bounds::empty(), is_bottom));
            }
        } else if line.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false) {
            let t: Vec<&str> = line.split_whitespace().collect();
            if t.len() >= 6 {
                if let Some((ref_name, is_bottom)) = &current_part {
                    let (Some(x), Some(y)) = (parse_num(t[2]), parse_num(t[3])) else { continue };
                    let (x_mm, y_mm) = (x * 25.4, y * 25.4);
                    pins.push(BoardPin {
                        name: format!("{}.{}", ref_name, t[1]),
                        x: x_mm,
                        y: y_mm,
                        net: t[5].to_string(),
                        side: side_name(*is_bottom).to_string(),
                    });
                    if let Some(e) = by_ref.get_mut(ref_name) {
                        e.0.add(x_mm, y_mm);
                    }
                }
            }
        }
    }

    if pins.is_empty() {
        return Err("BDV: не найдено ни одного пина".to_string());
    }

    let components = by_ref
        .into_iter()
        .filter(|(_, (b, _))| !b.is_empty())
        .map(|(name, (b, bottom))| make_component(name, &b, layer_name(bottom), String::new(), String::new(), 0.0, false))
        .collect();

    let mut board =
        BoardData { format: "bdv".to_string(), pins, lines: vec![], arcs: vec![], components, pads: vec![], routes: vec![] };
    ensure_outline(&mut board);
    Ok(board)
}

// ==========================================================================
//  6. ПАРСЕР XZZ (.pcb)
// ==========================================================================
const XZZ_SIGNATURE: &[u8] = b"XZZPCB";
/// Единицы XZZ -> мм (как было в исходном коде: raw / 10000 * 250).
const XZZ_UNIT_TO_MM: f64 = 250.0 / 10000.0;

fn get_xzz_des_key() -> GenericArray<u8, des::cipher::generic_array::typenum::U8> {
    let byte_list: [u16; 8] = [0xE0, 0xCF, 0x2E, 0x9F, 0x3C, 0x33, 0x3C, 0x33];
    let mut hex_str = String::with_capacity(16);
    for chunk in byte_list.chunks(2) {
        let val = (chunk[0] << 8) | chunk[1];
        hex_str.push_str(&format!("{:04X}", val ^ 0x3C33));
    }
    let key_u64 = u64::from_str_radix(&hex_str, 16).unwrap_or(0);
    GenericArray::from(key_u64.to_be_bytes())
}

/// Сигнатура XZZPCB, в том числе когда файл целиком XOR-нут ключом из байта 0x10.
fn xzz_signature_ok(raw: &[u8]) -> bool {
    if raw.len() < 6 {
        return false;
    }
    if &raw[..6] == XZZ_SIGNATURE {
        return true;
    }
    if raw.len() > 0x10 {
        let k = raw[0x10];
        if k != 0 {
            let hdr: Vec<u8> = raw[..6].iter().map(|b| b ^ k).collect();
            return hdr.as_slice() == XZZ_SIGNATURE;
        }
    }
    false
}

/// Таблица цепей XZZ: [u32 размер блока] { [u32 размер записи][u32 id][имя] ... }.
/// Возвращает None, если по этому смещению нет правдоподобной таблицы.
fn parse_xzz_net_block(data: &[u8], off: usize) -> Option<HashMap<u32, String>> {
    let size = rd_u32(data, off)? as usize;
    let start = off.checked_add(4)?;
    let end = start.checked_add(size)?;
    if size == 0 || end > data.len() {
        return None;
    }
    let mut map = HashMap::new();
    let mut cur = start;
    while cur + 8 <= end {
        let rec = rd_u32(data, cur)? as usize;
        let id = rd_u32(data, cur + 4)?;
        if rec < 8 || cur + rec > end {
            break;
        }
        let raw_name = &data[cur + 8..cur + rec];
        let trimmed: Vec<u8> = raw_name.iter().copied().take_while(|&b| b != 0).collect();
        if trimmed.iter().any(|&b| !(0x20..0x7F).contains(&b)) {
            break; // не текст — значит это не таблица цепей
        }
        map.insert(id, String::from_utf8_lossy(&trimmed).trim().to_string());
        cur += rec;
    }
    if map.is_empty() { None } else { Some(map) }
}

fn parse_xzz_bytes(file_data: &[u8]) -> Result<BoardData, String> {
    if file_data.len() < 6 {
        return Err("File too small".to_string());
    }
    let mut data = file_data.to_vec();
    if &data[0..6] != XZZ_SIGNATURE {
        if data.len() > 0x10 {
            let xor_key = data[0x10];
            if xor_key != 0 {
                for b in data.iter_mut() {
                    *b ^= xor_key;
                }
            }
        }
        if &data[0..6] != XZZ_SIGNATURE {
            return Err("Not a valid XZZPCB file".to_string());
        }
    }
    if data.len() < 0x44 {
        return Err("Header too small".to_string());
    }

    // Основной блок: по умолчанию 0x40; если размер там неправдоподобен — берём смещение из заголовка (0x20)
    let plausible = |off: usize| -> Option<usize> {
        let size = rd_u32(&data, off)? as usize;
        if size > 0 && off + 4 + size <= data.len() { Some(size) } else { None }
    };
    let (main_data_offset, main_data_size) = if let Some(s) = plausible(0x40) {
        (0x40usize, s)
    } else {
        let hdr = rd_u32(&data, 0x20).ok_or("Header too small")? as usize;
        let off = hdr.checked_add(0x20).ok_or("Bad header")?;
        let s = plausible(off).ok_or("Main data block not found")?;
        (off, s)
    };

    // Таблица цепей: пробуем смещения из заголовка и позицию сразу за основным блоком
    let main_end = main_data_offset + 4 + main_data_size;
    let mut net_candidates: Vec<usize> = Vec::new();
    for h in [0x24usize, 0x28, 0x2C] {
        if let Some(v) = rd_u32(&data, h) {
            if let Some(o) = (v as usize).checked_add(0x20) {
                net_candidates.push(o);
            }
        }
    }
    net_candidates.push(main_end);
    let net_names: HashMap<u32, String> =
        net_candidates.into_iter().find_map(|o| parse_xzz_net_block(&data, o)).unwrap_or_default();

    let des_key = get_xzz_des_key();
    let cipher = Des::new(&des_key);
    let decrypt_block_data = |input: &[u8]| -> Vec<u8> {
        let mut decrypted = input.to_vec();
        for chunk in decrypted.chunks_exact_mut(8) {
            let mut block = GenericArray::clone_from_slice(chunk);
            cipher.decrypt_block(&mut block);
            chunk.copy_from_slice(&block);
        }
        decrypted
    };
    let mm = |v: i32| v as f64 * XZZ_UNIT_TO_MM;

    let mut pins: Vec<BoardPin> = Vec::new();
    let mut components: Vec<GenCadComponent> = Vec::new();
    let mut lines: Vec<GenCadLine> = Vec::new();
    let mut used_refs: HashMap<String, usize> = HashMap::new();

    let mut current_offset = main_data_offset + 4;
    let end_offset = (current_offset + main_data_size).min(data.len());

    while current_offset + 5 <= end_offset {
        let block_type = data[current_offset];
        let Some(block_size) = rd_u32(&data, current_offset + 1).map(|v| v as usize) else { break };
        current_offset += 5;
        let Some(block_end) = current_offset.checked_add(block_size) else { break };
        if block_end > data.len() {
            break;
        }
        let block_data = &data[current_offset..block_end];

        match block_type {
            0x05 => {
                if block_data.len() >= 20 {
                    if let (Some(layer), Some(x1), Some(y1), Some(x2), Some(y2)) = (
                        rd_u32(block_data, 0),
                        rd_i32(block_data, 4),
                        rd_i32(block_data, 8),
                        rd_i32(block_data, 12),
                        rd_i32(block_data, 16),
                    ) {
                        lines.push(GenCadLine { layer: layer.to_string(), x1: mm(x1), y1: mm(y1), x2: mm(x2), y2: mm(y2) });
                    }
                }
            }
            0x07 => {
                let dec = decrypt_block_data(block_data);
                if dec.len() >= 26 {
                    let part_x = mm(rd_i32(&dec, 8).unwrap_or(0));
                    let part_y = mm(rd_i32(&dec, 12).unwrap_or(0));
                    let part_rotation = rd_u32(&dec, 16).unwrap_or(0) as f64 / 10000.0;
                    let pad_size_len = rd_u32(&dec, 22).unwrap_or(0) as usize;
                    let mut local = 26usize;

                    let mut footprint_name = String::new();
                    if pad_size_len > 0 && local + pad_size_len <= dec.len() {
                        footprint_name = String::from_utf8_lossy(&dec[local..local + pad_size_len]).trim().to_string();
                    }
                    local += pad_size_len;

                    let mut ref_name =
                        if !footprint_name.is_empty() { footprint_name.clone() } else { format!("COMP_{}", components.len()) };

                    let mut comp_is_bottom = false;
                    let mut comp_pins: Vec<(String, f64, f64, u32)> = Vec::new();
                    let mut comp_lines: Vec<(f64, f64, f64, f64)> = Vec::new();

                    while local + 5 <= dec.len() {
                        let sub_type = dec[local];
                        local += 1;
                        if sub_type == 0x00 {
                            continue;
                        }
                        let Some(sub_size) = rd_u32(&dec, local).map(|v| v as usize) else { break };
                        local += 4;
                        let Some(sub_end) = local.checked_add(sub_size) else { break };
                        if sub_end > dec.len() {
                            break;
                        }
                        let sub = &dec[local..sub_end];

                        match sub_type {
                            0x05 => {
                                if sub.len() >= 20 {
                                    if let (Some(x1), Some(y1), Some(x2), Some(y2)) =
                                        (rd_i32(sub, 4), rd_i32(sub, 8), rd_i32(sub, 12), rd_i32(sub, 16))
                                    {
                                        let seg = (mm(x1), mm(y1), mm(x2), mm(y2));
                                        lines.push(GenCadLine { layer: "COMPONENT".to_string(), x1: seg.0, y1: seg.1, x2: seg.2, y2: seg.3 });
                                        comp_lines.push(seg);
                                    }
                                }
                            }
                            0x06 => {
                                if sub.len() >= 30 {
                                    let name_size = rd_u32(sub, 26).unwrap_or(0) as usize;
                                    if 30 + name_size <= sub.len() {
                                        let lbl = String::from_utf8_lossy(&sub[30..30 + name_size]).trim().to_string();
                                        if !lbl.is_empty() {
                                            ref_name = lbl;
                                        }
                                    }
                                }
                            }
                            0x09 => {
                                if sub.len() >= 24 {
                                    let pin_x = mm(rd_i32(sub, 4).unwrap_or(0));
                                    let pin_y = mm(rd_i32(sub, 8).unwrap_or(0));
                                    let pin_flags = rd_i32(sub, 12).unwrap_or(0);
                                    if (pin_flags & 0x1) != 0 || (pin_flags & 0x10000) != 0 {
                                        comp_is_bottom = true;
                                    }
                                    let pin_name_size = rd_u32(sub, 20).unwrap_or(0) as usize;

                                    // имя пина — по порядку внутри детали, а не глобальный счётчик
                                    let mut pin_name = format!("{}", comp_pins.len() + 1);
                                    if pin_name_size > 0 && 24 + pin_name_size <= sub.len() {
                                        let n = String::from_utf8_lossy(&sub[24..24 + pin_name_size]).trim().to_string();
                                        if !n.is_empty() {
                                            pin_name = n;
                                        }
                                    }
                                    let net_id = if sub.len() >= 12 { rd_u32(sub, sub.len() - 12).unwrap_or(0) } else { 0 };
                                    comp_pins.push((pin_name, pin_x, pin_y, net_id));
                                }
                            }
                            _ => {}
                        }
                        local = sub_end;
                    }

                    // уникальное имя детали (иначе пины разных деталей слипаются по имени)
                    let n = used_refs.entry(ref_name.clone()).or_insert(0);
                    *n += 1;
                    if *n > 1 {
                        ref_name = format!("{}_{}", ref_name, *n);
                    }

                    let mut b = Bounds::empty();
                    for (_, px, py, _) in &comp_pins {
                        b.add(*px, *py);
                    }
                    // контур детали учитываем, только если он лежит рядом с пинами (защита от мусора)
                    if !b.is_empty() {
                        let near = b.expanded(b.width().max(2.0) + 3.0, b.height().max(2.0) + 3.0);
                        for s in &comp_lines {
                            if near.contains(s.0, s.1) && near.contains(s.2, s.3) {
                                b.add(s.0, s.1);
                                b.add(s.2, s.3);
                            }
                        }
                    } else {
                        b.add(part_x - 1.5, part_y - 1.5);
                        b.add(part_x + 1.5, part_y + 1.5);
                    }

                    for (pin_name, px, py, net_id) in comp_pins {
                        let net = match net_names.get(&net_id) {
                            Some(n) => n.clone(),
                            None if net_id == 0 => String::new(),
                            None => format!("NET_{}", net_id),
                        };
                        pins.push(BoardPin {
                            name: format!("{}.{}", ref_name, pin_name),
                            x: px,
                            y: py,
                            net,
                            side: side_name(comp_is_bottom).to_string(),
                        });
                    }

                    // бокс уже содержит нужный запас; make_component добавит минимальный размер
                    let mut c = make_component(
                        ref_name,
                        &b,
                        layer_name(comp_is_bottom),
                        footprint_name,
                        String::new(),
                        part_rotation,
                        false,
                    );
                    c.rotation = part_rotation;
                    components.push(c);
                }
            }
            _ => {}
        }
        current_offset = block_end;
    }

    if pins.is_empty() && lines.is_empty() {
        return Err("XZZ: в файле не найдено данных платы".to_string());
    }

    let mut board =
        BoardData { format: "pcb".to_string(), pins, lines, arcs: vec![], components, pads: vec![], routes: vec![] };
    ensure_outline(&mut board);
    Ok(board)
}

// ==========================================================================
//  7. ОПРЕДЕЛЕНИЕ ФОРМАТА И ЕДИНАЯ ТОЧКА ВХОДА
// ==========================================================================
#[derive(Debug, Clone, Copy, PartialEq)]
enum Kind {
    Bdv,
    Cad,
    Xzz,
    AppleBrd,
    Brd2,
    Fz,
}

/// Сначала смотрим на содержимое (сигнатуры), и только потом на расширение.
/// Так переименованные файлы и .brd двух разных форматов открываются правильно,
/// а на Android (где расширения может не быть) работает тот же код.
fn detect_kind(raw: &[u8], ext: &str) -> Option<Kind> {
    let head = &raw[..raw.len().min(4096)];
    if raw.starts_with(BDV_SIGNATURE) || contains_bytes(head, b"<<format.asc>>") || contains_bytes(head, b"<<pins.asc>>") {
        return Some(Kind::Bdv);
    }
    if raw.starts_with(b"$HEADER") || contains_bytes(head, b"GENCAD") {
        return Some(Kind::Cad);
    }
    if xzz_signature_ok(raw) {
        return Some(Kind::Xzz);
    }
    if raw.starts_with(APPLE_BRD_MAGIC) {
        return Some(Kind::AppleBrd);
    }
    if contains_bytes(&raw[..raw.len().min(1 << 20)], b"BRDOUT:") && contains_bytes(raw, b"NETS:") {
        return Some(Kind::Brd2);
    }
    match ext {
        "fz" | "cae" => Some(Kind::Fz),
        "cad" => Some(Kind::Cad),
        "bdv" | "asc" => Some(Kind::Bdv),
        "pcb" => Some(Kind::Xzz),
        _ => None,
    }
}

/// Единая функция разбора: используется и десктопной, и Android-командой
/// (временные файлы больше не нужны).
fn parse_boardview_bytes(raw: &[u8], ext: &str) -> Result<BoardData, String> {
    if raw.is_empty() {
        return Err("Файл пустой".to_string());
    }
    let ext = ext.to_lowercase();

    if ext == "tvw" || TvwCanifolParser::is_binary(raw) {
        match TvwCanifolParser::parse(raw) {
            Ok(board_data) => return Ok(board_data),
            Err(e) => eprintln!("[TVW] failed: {}", e),
        }
    }

    let kind = match detect_kind(raw, &ext) {
        Some(k) => k,
        None if ext.is_empty() => Kind::Fz, // у FZ нет открытой сигнатуры (зашифрован+сжат) — пробуем в последнюю очередь
        None => return Err(format!("Unsupported format: {}", ext)),
    };

    let result = match kind {
        Kind::Fz => parse_fz_bytes(raw),
        Kind::Cad => parse_cad(&String::from_utf8_lossy(raw)),
        Kind::Bdv => parse_bdv_from_content(&decode_bdv(raw)),
        Kind::Xzz => parse_xzz_bytes(raw),
        Kind::AppleBrd => parse_apple_brd_bytes(raw),
        Kind::Brd2 => parse_brd2_text(&String::from_utf8_lossy(raw)),
    };
    result.map_err(|e| format!("[{:?}] {}", kind, e))
}

/// Обёртка: любая паника в парсере превращается в ошибку, а не роняет приложение.
fn parse_boardview_safe(raw: &[u8], ext: &str) -> Result<BoardData, String> {
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| parse_boardview_bytes(raw, ext)))
        .unwrap_or_else(|_| Err("Внутренняя ошибка парсера (файл повреждён или формат не поддерживается)".to_string()))
}

// ==========================================================================
//  8. TAURI КОМАНДЫ
// ==========================================================================
#[tauri::command]
pub async fn read_boardview(path: String) -> Result<BoardData, String> {
    let raw = std::fs::read(&path).map_err(|e| format!("Read error: {}", e))?;
    let ext = std::path::Path::new(&path).extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
    parse_boardview_safe(&raw, &ext)
}

#[cfg(target_os = "android")]
#[tauri::command]
pub async fn read_boardview_uri(app: tauri::AppHandle, uri: String) -> Result<BoardData, String> {
    use std::io::Read;
    use tauri_plugin_android_fs::{AndroidFsExt, FileUri};

    let file_uri = FileUri::from_uri(&uri);
    let api = app.android_fs_async();
    let mut file = api.open_file_readable(&file_uri).await.map_err(|e| e.to_string())?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).map_err(|e| e.to_string())?;

    // у content:// URI расширения обычно нет -> определяем по содержимому
    parse_boardview_safe(&buffer, "")
}

// ==========================================================================
//  ТЕСТЫ
// ==========================================================================
#[cfg(test)]
mod tests {
    use super::*;

    const FZ_SAMPLE: &str = "\
UNIT:mils
A!REFDES!COMP_INSERTION_CODE!SYM_NAME!SYM_MIRROR!SYM_ROTATE!
S!U1!0!QFN4!NO!90!
S!C5!0!C0402!YES!!
A!NET_NAME!REFDES!PIN_NUMBER!PIN_NAME!PIN_X!PIN_Y!TEST_POINT!RADIUS!
S!GND!U1!1!GND!1000!2000!!10!
S!VCC!U1!2!VCC!1100!2000!!10!
S!GND!C5!1!1!3000!4000!!8!
S!VCC!C5!2!2!3040!4000!!8!
S!BAD!C5!3!3!oops!4000!!8!
";

    #[test]
    fn fz_columns_not_shifted() {
        let b = parse_fz_content(FZ_SAMPLE.as_bytes(), b"").unwrap();
        assert_eq!(b.pins.len(), 4); // битая строка пропущена
        let p = &b.pins[0];
        assert_eq!(p.name, "U1.1");
        assert_eq!(p.net, "GND");
        assert!((p.x - 25.4).abs() < 1e-9 && (p.y - 50.8).abs() < 1e-9); // mils -> mm
        assert_eq!(p.side, "top");
    }

    #[test]
    fn fz_mirror_means_bottom() {
        let b = parse_fz_content(FZ_SAMPLE.as_bytes(), b"").unwrap();
        let c5 = b.components.iter().find(|c| c.ref_name == "C5").unwrap();
        assert_eq!(c5.layer, "BOTTOM");
        assert!(b.pins.iter().filter(|p| p.name.starts_with("C5.")).all(|p| p.side == "bottom"));
        let u1 = b.components.iter().find(|c| c.ref_name == "U1").unwrap();
        assert_eq!(u1.rotation, 90.0);
        assert_eq!(u1.shape, "QFN4");
    }

    #[test]
    fn fz_millimeters_not_scaled() {
        let s = format!("UNIT:millimeters\n{}", FZ_SAMPLE.replace("UNIT:mils\n", ""));
        let b = parse_fz_content(s.as_bytes(), b"").unwrap();
        assert!((b.pins[0].x - 1000.0).abs() < 1e-9);
    }

    #[test]
    fn fz_description_and_fallback_outline() {
        let desc = "BOARD|REV|BOARD|REV|PN\r\nPART\tDESC\tQTY\tLOC\tPN2\r\nR1\t10k 0402\t1\tC5 U9\t\r\n";
        let b = parse_fz_content(FZ_SAMPLE.as_bytes(), desc.as_bytes()).unwrap();
        assert_eq!(b.components.iter().find(|c| c.ref_name == "C5").unwrap().device, "10k 0402");
        assert_eq!(b.lines.len(), 4); // рамка по габаритам пинов
    }

    #[test]
    fn fz_outline_and_etch_from_graphics() {
        let mut s = String::from(FZ_SAMPLE);
        s.push_str("A!CLASS!SUBCLASS!GRAPHIC_DATA_NAME!GRAPHIC_DATA_NUMBER!RECORD_TAG!GRAPHIC_DATA_1!GRAPHIC_DATA_2!GRAPHIC_DATA_3!GRAPHIC_DATA_4!GRAPHIC_DATA_5!GRAPHIC_DATA_6!GRAPHIC_DATA_7!GRAPHIC_DATA_8!GRAPHIC_DATA_9!NET_NAME!\n");
        s.push_str("S!BOARD GEOMETRY!OUTLINE!LINE!1!0!500!1500!3500!1500!0!0!0!0!0!!\n");
        s.push_str("S!BOARD GEOMETRY!OUTLINE!LINE!1!0!3500!1500!3500!4500!0!0!0!0!0!!\n");
        s.push_str("S!BOARD GEOMETRY!OUTLINE!LINE!1!0!3500!4500!500!4500!0!0!0!0!0!!\n");
        s.push_str("S!BOARD GEOMETRY!OUTLINE!LINE!1!0!500!4500!500!1500!0!0!0!0!0!!\n");
        s.push_str("S!ETCH!TOP!LINE!1!0!1000!2000!1100!2000!8!0!0!0!0!GND!\n");
        let b = parse_fz_content(s.as_bytes(), b"").unwrap();
        assert_eq!(b.lines.len(), 4);
        assert_eq!(b.routes.len(), 1);
        assert_eq!(b.routes[0].layer, "TOP");
    }

    #[test]
    fn fz_garbage_outline_is_rejected() {
        let mut s = String::from(FZ_SAMPLE);
        s.push_str("A!CLASS!SUBCLASS!GRAPHIC_DATA_NAME!GRAPHIC_DATA_NUMBER!RECORD_TAG!G1!G2!G3!G4!G5!G6!G7!G8!G9!NET_NAME!\n");
        s.push_str("S!BOARD GEOMETRY!OUTLINE!LINE!1!0!9000000!9000000!9000100!9000000!0!0!0!0!0!!\n");
        let b = parse_fz_content(s.as_bytes(), b"").unwrap();
        assert_eq!(b.lines.len(), 4); // осталась запасная рамка, мусор отброшен
        assert!(b.lines.iter().all(|l| l.x1 < 1000.0));
    }

    #[test]
    fn arc_quarter_circle() {
        let segs = tessellate_arc(1.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        assert!(segs.len() >= 4);
        let last = segs.last().unwrap();
        assert!((last.2 - 0.0).abs() < 1e-9 && (last.3 - 1.0).abs() < 1e-9);
        for s in &segs {
            assert!(((s.2 * s.2 + s.3 * s.3).sqrt() - 1.0).abs() < 1e-6);
        }
    }

    #[test]
    fn brd2_outline_and_pins() {
        let t = "BRDOUT: 4 100 100\n0 0\n100 0\n100 100\n0 100\nNETS: 2\n1 GND\n2 VCC\nPARTS: 2\nC1 0 0 10 10 1 1\nU1 20 20 40 40 3 2\nPINS: 5\n0 0 1 1\n10 10 2 1\n20 20 1 2\n30 30 2 2\n40 40 1 2\nNAILS: 0\n";
        let b = parse_brd2_text(t).unwrap();
        assert_eq!(b.lines.len(), 4);
        assert_eq!(b.components.len(), 2);
        assert_eq!(b.pins.len(), 5);
        assert_eq!(b.pins[0].name, "C1.1");
        assert_eq!(b.pins[2].name, "U1.1");
        assert_eq!(b.pins[2].net, "GND");
        assert_eq!(b.components[1].layer, "BOTTOM");
    }

    #[test]
    fn apple_brd_text_uses_real_refdes() {
        let t = "str_length: 0 0\nvar_data: 4 2 3 0\nFormat:\n0 0\n1000 0\n1000 1000\n0 1000\nParts:\nU1 4 2\nC2 8 3\nPins:\n100 100 1 1 GND\n200 100 1 1 VCC\n500 500 1 2 GND\nNails:\n";
        let b = parse_apple_brd_text(t).unwrap();
        assert_eq!(b.lines.len(), 4);
        assert_eq!(b.pins[0].name, "U1.1");
        assert_eq!(b.pins[1].name, "U1.2");
        assert_eq!(b.pins[2].name, "C2.1");
        assert_eq!(b.pins[2].side, "bottom");
        assert_eq!(b.components.len(), 2);
    }

    #[test]
    fn bdv_roundtrip_plain_text() {
        let t = "<<format.asc>>\n<<pins.asc>>\nPart U1 (T)\n1 A1 1.0 2.0 x GND\n2 A2 1.1 2.0 x VCC\nPart C1 (B)\n1 1 3.0 4.0 x GND\n<<nails.asc>>\n";
        let b = parse_bdv_from_content(t).unwrap();
        assert_eq!(b.pins.len(), 3);
        assert_eq!(b.pins[2].side, "bottom");
        assert_eq!(b.components.len(), 2);
    }

    #[test]
    fn xzz_net_block_parses() {
        // [size][ {rec_size=8+len, id, name} ... ]
        let mut blk = Vec::new();
        for (id, name) in [(1u32, "GND"), (2u32, "VCC_3V3")] {
            blk.extend_from_slice(&((8 + name.len()) as u32).to_le_bytes());
            blk.extend_from_slice(&id.to_le_bytes());
            blk.extend_from_slice(name.as_bytes());
        }
        let mut data = vec![0u8; 16];
        data.extend_from_slice(&(blk.len() as u32).to_le_bytes());
        data.extend_from_slice(&blk);
        let m = parse_xzz_net_block(&data, 16).unwrap();
        assert_eq!(m[&1], "GND");
        assert_eq!(m[&2], "VCC_3V3");
        assert!(parse_xzz_net_block(&data, 0).is_none());
    }

    #[test]
    fn detect_by_content_beats_extension() {
        assert_eq!(detect_kind(b"BRDOUT: 0 0 0\nNETS: 0\n", "brd"), Some(Kind::Brd2));
        assert_eq!(detect_kind(b"$HEADER\nGENCAD 1.4\n", "txt"), Some(Kind::Cad));
        assert_eq!(detect_kind(b"\x23\xE2\x63\x28abcd", "brd"), Some(Kind::AppleBrd));
        assert_eq!(detect_kind(b"whatever", "fz"), Some(Kind::Fz));
        assert_eq!(detect_kind(b"whatever", "zzz"), None);
    }

    #[test]
    fn garbage_does_not_panic() {
        for ext in ["fz", "pcb", "brd", "bdv", "cad", ""] {
            let _ = parse_boardview_safe(&[0xFF; 64], ext);
            let _ = parse_boardview_safe(b"XZZPCB", ext);
        }
    }
}