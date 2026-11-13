//! Парсер Teleplan / TestView `.tvw` для Canifol.
//!
//! Адаптирован под старые типы `BoardPin`, `GenCadComponent`, `GenCadLine`.
//! Отдаёт `BoardData` напрямую (без промежуточного `Board`), чтобы не гонять
//! данные через конвертер.
//!
//! Схема разбора:
//!   1. таблица имён цепей (в конце файла)              -> net_names
//!   2. записи компонентов (refdes, bbox, side, пины)   -> ComponentRecord
//!   3. по каждому слою (маркеры TOP / BOTTOM) ищем ЛУЧШУЮ секцию пинов
//!      (самую длинную, а не все подряд)                -> top_pins / bot_pins
//!   4. пины компонентов в три прохода:
//!      а) канонически: top_pins/bot_pins[pad_index / 8] (сначала своя сторона, занятые не берём);
//!      б) запасной вариант для промахнувшихся: свободные пины в bbox по порядку индекса,
//!         сопоставляются с пинами по возрастанию pad_index (маленькие компоненты первыми);
//!      в) «бесхозные» пины внутри bbox той же стороны получают синтетические имена x1, x2…
//!      Мусорные компоненты отсекаются по экстенту пинов, имена пинов уникальны в компоненте.
//!   5. контур платы = самая «чистая» большая поверхность (surface)
//!
//! Координаты в файле — сотые доли мила (cmil), наружу отдаём в милах (/100).

use std::collections::{HashMap, HashSet};
use crate::gencad_parser::{GenCadComponent, GenCadLine};
use crate::boardview::{BoardData, BoardPin};

const TVW_MAGIC_SIG: (u8, &[u8]) = (0x13, b"O95w-28ps49m 02v9o.");
const TVW_VERSION_LE: [u8; 4] = [0x01, 0x00, 0x00, 0x00];
const MAX_COORD_CMILS: i64 = 5_000_000;
/// Запас bbox для поиска пина (в cmil, 100 мил = 10000 cmil).
const BBOX_MARGIN_CMIL: i64 = 10_000;
/// Запас для «запасного» маппинга (20 мил): он менее надёжен, поэтому строже.
const FALLBACK_MARGIN_CMIL: i64 = 2_000;
/// Запас для привязки «бесхозных» пинов к компоненту (5 мил).
const STRAY_MARGIN_CMIL: i64 = 500;
/// Запас вокруг экстента пинов при фильтре мусорных компонентов (200 мил).
const EXTENT_MARGIN_CMIL: i64 = 20_000;

#[derive(Clone, Debug)]
struct TvwPin {
    net_index: u32,
    pin_local: u32,
    x: i32,
    y: i32,
}

#[derive(Clone, Debug)]
struct SurfaceRecord {
    vertices: Vec<(i32, i32)>,
    void_count: u32,
}

#[derive(Clone, Debug)]
struct ComponentPin {
    pad_index: u32,
    pin_index: u32,
    name: String,
}

#[derive(Clone, Debug)]
struct ComponentRecord {
    refdes: String,
    value: String,
    cx: i32,
    cy: i32,
    bx1: i32,
    by1: i32,
    bx2: i32,
    by2: i32,
    footprint_index: u32,
    side: u32,
    pins: Vec<ComponentPin>,
}

pub struct TvwParser;

impl TvwParser {
    // ------------------------------------------------------------------
    // Публичный вход
    // ------------------------------------------------------------------

    pub fn parse(data: &[u8]) -> Result<BoardData, String> {
        if Self::is_binary(data) {
            return Self::parse_binary(data);
        }
        let decoded = Self::deobfuscate_ascii(data);
        let text = String::from_utf8_lossy(&decoded);
        let (pins, comps) = Self::parse_ascii(&text);
        if pins.is_empty() && comps.is_empty() {
            return Err("Unsupported TVW variant (not production binary, ASCII parse found nothing)".to_string());
        }
        Ok(BoardData {
            format: "tvw".to_string(),
            pins,
            lines: Vec::new(),
            arcs: Vec::new(),
            components: comps,
            pads: Vec::new(),
            routes: Vec::new(),
        })
    }

    pub fn is_binary(data: &[u8]) -> bool {
        if data.len() < 64 { return false; }
        let sig_len = TVW_MAGIC_SIG.0 as usize;
        let off = 1 + sig_len;
        if data.len() < off + 4 { return false; }
        if data[0] != TVW_MAGIC_SIG.0 || &data[1..1 + sig_len] != TVW_MAGIC_SIG.1 {
            return false;
        }
        data[off..off + 4] == TVW_VERSION_LE
    }

    pub fn deobfuscate_ascii(data: &[u8]) -> Vec<u8> {
        data.iter().map(|&b| {
            if (b'0'..=b'9').contains(&b) {
                ((b - b'0' + 10 - 3) % 10) + b'0'
            } else if (b'a'..=b'z').contains(&b) {
                ((b - b'a' + 26 - 10) % 26) + b'a'
            } else if (b'A'..=b'Z').contains(&b) {
                ((b - b'A' + 26 - 10) % 26) + b'A'
            } else { b }
        }).collect()
    }

    // ------------------------------------------------------------------
    // Низкоуровневое чтение
    // ------------------------------------------------------------------

    #[inline(always)]
    fn rd_u32(buf: &[u8], off: usize) -> Option<u32> {
        let end = off.checked_add(4)?;
        if end > buf.len() { return None; }
        Some(u32::from_le_bytes([buf[off], buf[off + 1], buf[off + 2], buf[off + 3]]))
    }

    #[inline(always)]
    fn rd_i32(buf: &[u8], off: usize) -> Option<i32> {
        let end = off.checked_add(4)?;
        if end > buf.len() { return None; }
        Some(i32::from_le_bytes([buf[off], buf[off + 1], buf[off + 2], buf[off + 3]]))
    }

    // ------------------------------------------------------------------
    // Секции пинов
    // ------------------------------------------------------------------

    fn read_pin_record(buf: &[u8], off0: usize, region_end: usize) -> Option<(TvwPin, usize)> {
        let mut off = off0;
        if off + 18 > region_end { return None; }

        let net_index = Self::rd_u32(buf, off)?; off += 4;
        let pin_local = Self::rd_u32(buf, off)?; off += 4;
        let x = Self::rd_i32(buf, off)?; off += 4;
        let y = Self::rd_i32(buf, off)?; off += 4;
        let _flag1 = *buf.get(off)?; off += 1;
        let has_ext = *buf.get(off)?; off += 1;

        if has_ext != 0 {
            if off + 3 > region_end { return None; }
            let sub_a = *buf.get(off)?; off += 1;
            if sub_a == 1 {
                if off + 12 > region_end { return None; }
                off += 12;
            }
            let sub_b = *buf.get(off)?; off += 1;
            if sub_b != 0 {
                if off + 16 > region_end { return None; }
                off += 16;
            }
            let sub_c = *buf.get(off)?; off += 1;
            if sub_c != 0 {
                if off + 16 > region_end { return None; }
                off += 16;
            }
        }

        if off + 1 > region_end { return None; }
        let _flag3 = *buf.get(off)?; off += 1;

        Some((TvwPin { net_index, pin_local, x, y }, off))
    }

    fn is_plausible_pin(p: &TvwPin) -> bool {
        if (p.x as i64).abs() > MAX_COORD_CMILS { return false; }
        if (p.y as i64).abs() > MAX_COORD_CMILS { return false; }
        if p.net_index >= (1u32 << 31) { return false; }
        if p.pin_local > 0x100000 { return false; }
        true
    }

    fn try_walk_pins_at(
        buf: &[u8],
        off: usize,
        region_end: usize,
        max_pin_count: u32,
        min_partial_ratio: f64,
    ) -> Option<(Vec<TvwPin>, usize, u32)> {
        if off + 12 > region_end { return None; }
        let first_count = Self::rd_u32(buf, off)?;
        let pin_count = Self::rd_u32(buf, off + 4)?;
        let mut off2 = off + 8;
        if pin_count == 0 {
            return Some((Vec::new(), off2, 0));
        }
        if pin_count > max_pin_count || first_count > max_pin_count { return None; }
        let _gap = Self::rd_u32(buf, off2)?;
        off2 += 4;

        let mut pins = Vec::new();
        let mut cur = off2;
        for _ in 0..pin_count {
            match Self::read_pin_record(buf, cur, region_end) {
                Some((r, new_off)) if Self::is_plausible_pin(&r) => {
                    pins.push(r);
                    cur = new_off;
                }
                _ => break,
            }
        }
        if pins.is_empty() { return None; }
        if pin_count >= 10 && (pins.len() as f64) < f64::max(2.0, pin_count as f64 * min_partial_ratio) {
            return None;
        }
        Some((pins, cur, pin_count))
    }

    fn looks_like_pin_record(buf: &[u8], off: usize, region_end: usize) -> bool {
        if off + 18 > region_end { return false; }
        let x = match Self::rd_i32(buf, off + 8) { Some(v) => v, None => return false };
        let y = match Self::rd_i32(buf, off + 12) { Some(v) => v, None => return false };
        (x as i64).abs() <= MAX_COORD_CMILS && (y as i64).abs() <= MAX_COORD_CMILS
    }

    fn looks_like_pin_section_header(buf: &[u8], off: usize, region_end: usize) -> bool {
        if off + 12 + 18 > region_end || off + 16 > buf.len() { return false; }
        if buf[off + 3] != 0 || buf[off + 7] != 0 || buf[off + 11] != 0 || buf[off + 15] != 0 {
            return false;
        }
        let first_count = match Self::rd_u32(buf, off) { Some(v) => v, None => return false };
        let pin_count = match Self::rd_u32(buf, off + 4) { Some(v) => v, None => return false };
        if pin_count < 1 || pin_count > 100_000 || first_count > 100_000 { return false; }
        Self::looks_like_pin_record(buf, off + 12, region_end)
    }

    fn scan_best_pin_section(
        buf: &[u8],
        scan_start: usize,
        scan_end: usize,
        region_end: usize,
        step: usize,
    ) -> Option<(usize, Vec<TvwPin>, usize, u32)> {
        let mut best = None;
        let mut best_len = 0usize;
        let step = step.max(1);
        let mut cand = scan_start;
        while cand < scan_end {
            if Self::looks_like_pin_section_header(buf, cand, region_end) {
                if let Some((pins, end, declared)) = Self::try_walk_pins_at(buf, cand, region_end, 200_000, 0.5) {
                    if pins.len() > best_len {
                        best_len = pins.len();
                        best = Some((cand, pins, end, declared));
                    }
                }
            }
            cand += step;
        }
        best
    }

    // ------------------------------------------------------------------
    // Поверхности
    // ------------------------------------------------------------------

    fn read_surfaces(buf: &[u8], off: usize, region_end: usize) -> Vec<SurfaceRecord> {
        let mut surfaces = Vec::new();
        if off + 4 > region_end { return surfaces; }
        let count = match Self::rd_u32(buf, off) { Some(v) => v, None => return surfaces };
        if count == 0 || count > 1_000_000 { return surfaces; }
        let mut cur = off + 8;

        for _ in 0..count {
            if cur + 8 > region_end { break; }
            let _kind = match Self::rd_i32(buf, cur) { Some(v) => v, None => break };
            let vc = match Self::rd_i32(buf, cur + 4) { Some(v) => v, None => break };
            cur += 8;

            if vc < 0 || vc > 50_000 { return surfaces; }
            let vc = vc as usize;
            if cur + 8 * vc > region_end { break; }

            let mut vertices = Vec::with_capacity(vc);
            for _ in 0..vc {
                let x = match Self::rd_i32(buf, cur) { Some(v) => v, None => return surfaces };
                let y = match Self::rd_i32(buf, cur + 4) { Some(v) => v, None => return surfaces };
                cur += 8;
                vertices.push((x, y));
            }

            if vc > 0 {
                if cur + 4 > region_end { break; }
                cur += 4;
            }

            if cur + 4 > region_end { break; }
            let void_count = match Self::rd_u32(buf, cur) { Some(v) => v, None => break };
            cur += 4;

            if void_count > 100_000 { return surfaces; }
            if void_count > 0 {
                if cur + 4 > region_end { return surfaces; }
                cur += 4;
            }

            let mut valid_voids = true;
            for _ in 0..void_count {
                if cur + 4 > region_end { valid_voids = false; break; }
                let vv = match Self::rd_u32(buf, cur) { Some(v) => v as usize, None => { valid_voids = false; break; } };
                cur += 4;
                if vv > 50_000 || cur + 8 * vv > region_end { valid_voids = false; break; }
                cur += 8 * vv;
                if vv > 0 {
                    if cur + 4 > region_end { valid_voids = false; break; }
                    cur += 4;
                }
            }
            if !valid_voids { return surfaces; }

            surfaces.push(SurfaceRecord { vertices, void_count });
        }
        surfaces
    }

    // ------------------------------------------------------------------
    // Таблица имён цепей
    // ------------------------------------------------------------------

    fn scan_pascal_string_run(buf: &[u8], start: usize, end: usize) -> (Vec<String>, usize) {
        let mut names = Vec::new();
        let mut cur = start;
        while cur < end {
            let n = buf[cur] as usize;
            if n == 0 || cur + 1 + n > end || n > 64 { break; }
            let s_bytes = &buf[cur + 1..cur + 1 + n];
            if s_bytes.iter().all(|&b| (32..=126).contains(&b)) {
                names.push(String::from_utf8_lossy(s_bytes).into_owned());
                cur += 1 + n;
            } else {
                break;
            }
        }
        (names, cur)
    }

    fn scan_network_names(buf: &[u8]) -> Vec<String> {
        let end = buf.len();
        let safe_window = end.saturating_sub(end / 4 + 4096);
        let mut cur = safe_window;

        while cur + 8 < end {
            if let (Some(c1), Some(c2)) = (Self::rd_u32(buf, cur), Self::rd_u32(buf, cur + 4)) {
                if c1 > 0 && c1 == c2 && c1 <= 100_000 {
                    let (names, _) = Self::scan_pascal_string_run(buf, cur + 8, end);
                    if names.len() == c1 as usize {
                        return names;
                    }
                }
            }
            cur += 1;
        }

        cur = safe_window;
        let mut best_names: Vec<String> = Vec::new();
        while cur + 4 < end {
            let (names, _) = Self::scan_pascal_string_run(buf, cur, end);
            if !names.is_empty() && names.len() >= best_names.len() {
                best_names = names;
            }
            cur += 1;
        }
        best_names
    }

    // ------------------------------------------------------------------
    // Компоненты
    // ------------------------------------------------------------------

    fn is_refdes_pattern(body: &[u8]) -> bool {
        if body.is_empty() || body.len() > 10 { return false; }
        let mut letters = 0;
        let mut digits = 0;
        let mut in_digits = false;
        for &b in body {
            if !in_digits {
                if b.is_ascii_alphabetic() {
                    letters += 1;
                    if letters > 5 { return false; }
                } else if b.is_ascii_digit() {
                    if letters == 0 { return false; }
                    in_digits = true;
                    digits += 1;
                } else { return false; }
            } else if b.is_ascii_digit() {
                digits += 1;
                if digits > 5 { return false; }
            } else { return false; }
        }
        letters > 0 && digits > 0
    }

    fn scan_component_records(buf: &[u8]) -> Vec<ComponentRecord> {
        let n = buf.len();
        let mut candidates = Vec::new();

        for i in 0..n.saturating_sub(40) {
            let plen = buf[i] as usize;
            if !(2..=8).contains(&plen) || i + 1 + plen + 32 > n { continue; }
            let body = &buf[i + 1..i + 1 + plen];
            if !Self::is_refdes_pattern(body) { continue; }

            if (plen == 2 || plen == 3) && body[0] == b'L' && body[1..].iter().all(|b| b.is_ascii_digit()) {
                let num: u32 = std::str::from_utf8(&body[1..]).ok().and_then(|s| s.parse().ok()).unwrap_or(0);
                if num <= 15 { continue; }
            }

            let o = i + 1 + plen;
            let bx1 = match Self::rd_i32(buf, o) { Some(v) => v, None => continue };
            let by1 = match Self::rd_i32(buf, o + 4) { Some(v) => v, None => continue };
            let bx2 = match Self::rd_i32(buf, o + 8) { Some(v) => v, None => continue };
            let by2 = match Self::rd_i32(buf, o + 12) { Some(v) => v, None => continue };
            let cx = match Self::rd_i32(buf, o + 16) { Some(v) => v, None => continue };
            let cy = match Self::rd_i32(buf, o + 20) { Some(v) => v, None => continue };

            if [bx1, by1, bx2, by2, cx, cy].iter().any(|&v| (v as i64).abs() > 10_000_000) { continue; }
            if bx1 >= bx2 || by1 >= by2 { continue; }
            let w = bx2 - bx1;
            let h = by2 - by1;
            if w < 100 || h < 100 || w > 5_000_000 || h > 5_000_000 { continue; }
            let mx = w / 2;
            let my = h / 2;
            if !(bx1 - mx <= cx && cx <= bx2 + mx && by1 - my <= cy && cy <= by2 + my) { continue; }
            candidates.push(i);
        }

        let mut components = Vec::new();
        for idx in 0..candidates.len() {
            let start = candidates[idx];
            let next_start = if idx + 1 < candidates.len() { candidates[idx + 1] } else { n };
            if let Some(comp) = Self::parse_component_at(buf, start, next_start) {
                components.push(comp);
            }
        }
        components
    }

    fn parse_component_at(buf: &[u8], start: usize, region_end: usize) -> Option<ComponentRecord> {
        let mut o = start;
        let plen = buf.get(o).copied()? as usize;
        o += 1;
        if !(1..=12).contains(&plen) || o + plen + 32 > region_end { return None; }
        let refdes_bytes = &buf[o..o + plen];
        if !refdes_bytes.iter().all(|&b| (32..=126).contains(&b)) { return None; }
        let refdes = String::from_utf8_lossy(refdes_bytes).into_owned();
        o += plen;

        let bx1 = Self::rd_i32(buf, o)?; o += 4;
        let by1 = Self::rd_i32(buf, o)?; o += 4;
        let bx2 = Self::rd_i32(buf, o)?; o += 4;
        let by2 = Self::rd_i32(buf, o)?; o += 4;
        let cx = Self::rd_i32(buf, o)?; o += 4;
        let cy = Self::rd_i32(buf, o)?; o += 4;
        let _rotation = Self::rd_u32(buf, o)?; o += 4;
        let footprint_index = Self::rd_u32(buf, o)?; o += 4;
        let _part_class = Self::rd_u32(buf, o)?; o += 4;
        if o + 8 > region_end { return None; }
        o += 8;

        let mut value = String::new();
        let mut string_region_ok = true;

        if o + 2 <= region_end {
            let sep_v = buf[o];
            let plen_v = buf[o + 1] as usize;
            if sep_v == 0x01 && plen_v <= 64 && o + 2 + plen_v <= region_end {
                o += 2;
                value = String::from_utf8_lossy(&buf[o..o + plen_v]).into_owned();
                o += plen_v;
            } else {
                string_region_ok = false;
            }
        }
        if string_region_ok && o + 1 <= region_end {
            let l = buf[o] as usize;
            if l <= 64 && o + 1 + l <= region_end { o += 1 + l; } else { string_region_ok = false; }
        }
        if string_region_ok && o + 1 <= region_end {
            let l = buf[o] as usize;
            if l <= 64 && o + 1 + l <= region_end { o += 1 + l; } else { string_region_ok = false; }
        }
        if string_region_ok && o + 1 <= region_end {
            let l = buf[o] as usize;
            if l <= 64 && o + 1 + l <= region_end { o += 1 + l; }
        }
        if string_region_ok && o + 1 <= region_end {
            let l = buf[o] as usize;
            if l <= 64 && o + 1 + l <= region_end { o += 1 + l; }
        }
        if o + 4 <= region_end { o += 4; }

        let mut pins = Vec::new();
        let mut side = 2u32;

        if o + 4 <= region_end {
            let cnt_u32 = Self::rd_u32(buf, o).unwrap_or(0);
            o += 4;
            let pin_count = if cnt_u32 <= 10_000 { cnt_u32 } else { 0 };

            if o + 8 <= region_end {
                side = Self::rd_u32(buf, o).unwrap_or(2);
                o += 4;
                let _zero = Self::rd_u32(buf, o).unwrap_or(0);
                o += 4;

                for _ in 0..pin_count {
                    if o + 12 > region_end { break; }
                    let pad_idx = Self::rd_u32(buf, o)?; o += 4;
                    let _u = Self::rd_u32(buf, o)?; o += 4;
                    let pin_idx = Self::rd_u32(buf, o)?; o += 4;
                    if o + 1 > region_end { break; }
                    let name_len = buf[o] as usize;
                    o += 1;
                    if name_len > 16 || o + name_len + 4 > region_end { break; }
                    let pin_name = String::from_utf8_lossy(&buf[o..o + name_len]).into_owned();
                    o += name_len;
                    let _trailer = Self::rd_u32(buf, o)?; o += 4;

                    pins.push(ComponentPin { pad_index: pad_idx, pin_index: pin_idx, name: pin_name });
                }
            }
        }

        Some(ComponentRecord { refdes, value, cx, cy, bx1, by1, bx2, by2, footprint_index, side, pins })
    }

    fn comp_is_top(comp: &ComponentRecord) -> bool {
        if comp.side == 2 { true }
        else if comp.side == 11 { false }
        else { (comp.footprint_index & 1) == 1 }
    }

    /// Пин попадает в bbox компонента с запасом `margin` (cmil).
    #[inline]
    fn pin_in_bbox(p: &TvwPin, comp: &ComponentRecord, margin: i64) -> bool {
        let px = p.x as i64;
        let py = p.y as i64;
        px >= comp.bx1 as i64 - margin && px <= comp.bx2 as i64 + margin
            && py >= comp.by1 as i64 - margin && py <= comp.by2 as i64 + margin
    }

    // ------------------------------------------------------------------
    // Основной разбор
    // ------------------------------------------------------------------

    pub fn parse_binary(buf: &[u8]) -> Result<BoardData, String> {
        // 1. Цепи и компоненты
        let net_names = Self::scan_network_names(buf);
        let raw_components = Self::scan_component_records(buf);

        // Убираем дубликаты
        let mut seen: HashSet<(String, i32, i32)> = HashSet::new();
        let mut components: Vec<ComponentRecord> = Vec::new();
        for c in raw_components {
            if seen.insert((c.refdes.clone(), c.cx, c.cy)) {
                components.push(c);
            }
        }
        if components.is_empty() {
            return Err("TVW: не найдено ни одного компонента".to_string());
        }


        #[cfg(debug_assertions)]
        {
            eprintln!("[TVW diag] net_names: {}", net_names.len());
            if !net_names.is_empty() {
                eprintln!("[TVW diag]   first 5 nets: {:?}", &net_names[..net_names.len().min(5)]);
            }
            eprintln!("[TVW diag] components: {}", components.len());
            if !components.is_empty() {
                let c = &components[0];
                eprintln!("[TVW diag]   sample comp: refdes={}, cx={}, cy={}, bbox=({},{})-({},{}), side={}, pins={}",
                    c.refdes, c.cx, c.cy, c.bx1, c.by1, c.bx2, c.by2, c.side, c.pins.len());
            }
        }

        // 2. Маркеры слоёв
        let mut layer_markers: Vec<(usize, bool)> = Vec::new();
        let target_names: [(&[u8], bool); 4] = [
            (&b"TOP"[..], true),
            (&b"top"[..], true),
            (&b"BOTTOM"[..], false),
            (&b"bottom"[..], false),
        ];
        for &(layer_name, is_top) in target_names.iter() {
            let len_byte = layer_name.len() as u8;
            let mut search_from = 0usize;
            while search_from < buf.len() {
                let found = buf[search_from..]
                    .windows(layer_name.len() + 1)
                    .position(|w| w[0] == len_byte && &w[1..] == layer_name);
                match found {
                    Some(pos) => {
                        let actual = search_from + pos;
                        layer_markers.push((actual, is_top));
                        search_from = actual + 1;
                    }
                    None => break,
                }
            }
        }
        layer_markers.sort_by_key(|&(pos, _)| pos);

        #[cfg(debug_assertions)]
        {
            eprintln!("[TVW diag] layer_markers: {}", layer_markers.len());
            for (i, (pos, is_top)) in layer_markers.iter().enumerate() {
                eprintln!("[TVW diag]   marker #{}: pos=0x{:x}, side={}", i, pos, if *is_top { "TOP" } else { "BOTTOM" });
            }
        }

        // 3. Секции пинов и поверхности по слоям
        let mut top_pins: Vec<TvwPin> = Vec::new();
        let mut bot_pins: Vec<TvwPin> = Vec::new();
        let mut all_surfaces: Vec<SurfaceRecord> = Vec::new();

        for i in 0..layer_markers.len() {
            let (marker_pos, is_top) = layer_markers[i];
            let layer_end = if i + 1 < layer_markers.len() {
                layer_markers[i + 1].0
            } else {
                buf.len().saturating_sub(buf.len() / 4)
            };
            if marker_pos >= layer_end { continue; }

            let scan_end = layer_end.saturating_sub(4); // расширенный отступ
            match Self::scan_best_pin_section(buf, marker_pos, scan_end, layer_end, 4) {
                Some((hdr_pos, pins, pins_end, declared)) => {
                    #[cfg(debug_assertions)]
                    eprintln!("[TVW diag]   layer #{} ({}): declared={}, read={}, header_pos=0x{:x}",
                        i, if is_top { "TOP" } else { "BOTTOM" }, declared, pins.len(), hdr_pos);

                    if declared > 0 && pins.len() == declared as usize {
                        let mut cur = pins_end;
                        if let Some(lc) = Self::rd_u32(buf, cur) {
                            if lc <= 10_000_000 && cur + 8 + (lc as usize * 24) <= layer_end {
                                cur += 8 + (lc as usize * 24);
                                if let Some(ac) = Self::rd_i32(buf, cur) {
                                    if ac <= 0 {
                                        cur += 4;
                                    } else if ac <= 10_000_000 && cur + 8 + (ac as usize * 28) <= layer_end {
                                        cur += 8 + (ac as usize * 28);
                                    }
                                    all_surfaces.extend(Self::read_surfaces(buf, cur, layer_end));
                                }
                            }
                        }
                    }

                    if is_top {
                        if pins.len() > top_pins.len() { top_pins = pins; }
                    } else if pins.len() > bot_pins.len() {
                        bot_pins = pins;
                    }
                }
                None => {
                    #[cfg(debug_assertions)]
                    eprintln!("[TVW diag]   layer #{} ({}): no pin section found", i, if is_top { "TOP" } else { "BOTTOM" });
                }
            }
        }

        if top_pins.is_empty() && bot_pins.is_empty() {
            let end = buf.len().saturating_sub(buf.len() / 4);
            if let Some((_, pins, _, _)) = Self::scan_best_pin_section(buf, 64, end.saturating_sub(30), end, 4) {
                top_pins = pins;
            }
        }

        #[cfg(debug_assertions)]
        eprintln!("[TVW diag] top_pins: {}, bot_pins: {}, surfaces: {}",
            top_pins.len(), bot_pins.len(), all_surfaces.len());

        // 3.5. Фильтр мусорных записей по РЕАЛЬНОМУ размеру платы (экстент пинов),
        //      а не по зашитым константам: плата любого размера останется целой.
        {
            let (mut min_x, mut min_y, mut max_x, mut max_y) = (i64::MAX, i64::MAX, i64::MIN, i64::MIN);
            for p in top_pins.iter().chain(bot_pins.iter()) {
                min_x = min_x.min(p.x as i64);
                max_x = max_x.max(p.x as i64);
                min_y = min_y.min(p.y as i64);
                max_y = max_y.max(p.y as i64);
            }
            if min_x <= max_x && min_y <= max_y {
                let mx = (max_x - min_x) / 8 + EXTENT_MARGIN_CMIL;
                let my = (max_y - min_y) / 8 + EXTENT_MARGIN_CMIL;
                let before = components.len();
                components.retain(|c| {
                    let ccx = (c.bx1 as i64 + c.bx2 as i64) / 2;
                    let ccy = (c.by1 as i64 + c.by2 as i64) / 2;
                    ccx >= min_x - mx && ccx <= max_x + mx && ccy >= min_y - my && ccy <= max_y + my
                });
                if cfg!(debug_assertions) {
                    eprintln!("[TVW diag] components outside pin extent dropped: {} -> {}", before, components.len());
                }
            }
        }
        if components.is_empty() {
            return Err("TVW: после фильтрации не осталось компонентов".to_string());
        }

        if cfg!(debug_assertions) {
            let mut names: HashSet<&str> = HashSet::new();
            let dup = components.iter().filter(|c| !names.insert(c.refdes.as_str())).count();
            if dup > 0 {
                eprintln!("[TVW diag] WARNING: {} components share a refdes with another record (different position)", dup);
            }
        }

        // 4. Компоненты, затем пины в три прохода
        let comp_top: Vec<bool> = components.iter().map(|c| Self::comp_is_top(c)).collect();
        let mut board_components: Vec<GenCadComponent> = Vec::new();
        // (refdes, is_top, x1, y1, x2, y2) в милах
        let mut part_boxes: Vec<(String, bool, f64, f64, f64, f64)> = Vec::new();

        for (ci, comp) in components.iter().enumerate() {
            let is_top = comp_top[ci];
            let (x1, y1) = (comp.bx1 as f64 / 100.0, comp.by1 as f64 / 100.0);
            let (x2, y2) = (comp.bx2 as f64 / 100.0, comp.by2 as f64 / 100.0);

            board_components.push(GenCadComponent {
                ref_name: comp.refdes.clone(),
                x: comp.cx as f64 / 100.0,
                y: comp.cy as f64 / 100.0,
                shape: "".to_string(),
                layer: if is_top { "TOP".to_string() } else { "BOTTOM".to_string() },
                bounds_x1: x1,
                bounds_y1: y1,
                bounds_x2: x2,
                bounds_y2: y2,
                rotation: 0.0,
                mirror: false,
                device: comp.value.clone(),
            });
            part_boxes.push((comp.refdes.clone(), is_top, x1, y1, x2, y2));
        }

        // assign[ci][pi] = Some((сторона_top, индекс_в_списке_слоя, вид)), вид: 0 = канонический, 1 = запасной
        let mut assign: Vec<Vec<Option<(bool, usize, u8)>>> =
            components.iter().map(|c| vec![None; c.pins.len()]).collect();
        let mut claimed_top: HashSet<usize> = HashSet::new();
        let mut claimed_bot: HashSet<usize> = HashSet::new();

        // --- Проход 1: канонический маппинг pad_index / 8.
        // Сначала сторона самого компонента, и только потом чужая (сквозные выводы лежат в обоих слоях).
        // Уже занятый пин повторно не берём: раньше два компонента могли получить один и тот же пин.
        for (ci, comp) in components.iter().enumerate() {
            let sides: [bool; 2] = if comp_top[ci] { [true, false] } else { [false, true] };
            for (pi, cp) in comp.pins.iter().enumerate() {
                let idx = (cp.pad_index / 8) as usize;
                for &side_top in sides.iter() {
                    let list = if side_top { &top_pins } else { &bot_pins };
                    let claimed = if side_top { &claimed_top } else { &claimed_bot };
                    if let Some(p) = list.get(idx) {
                        if !claimed.contains(&idx) && Self::pin_in_bbox(p, comp, BBOX_MARGIN_CMIL) {
                            assign[ci][pi] = Some((side_top, idx, 0));
                            break;
                        }
                    }
                }
                if let Some((st, idx, _)) = assign[ci][pi] {
                    if st { claimed_top.insert(idx); } else { claimed_bot.insert(idx); }
                }
            }
        }

        // --- Проход 2: запасной маппинг для тех, кто не нашёл канонический пин.
        // Маленькие компоненты первыми (у больших bbox шире и они воруют чужие пины под собой).
        // Кандидаты берутся в ПОРЯДКЕ индекса списка и сопоставляются с пинами по возрастанию pad_index:
        // если канонический маппинг промахнулся на постоянное смещение, порядок сохранится верным.
        // (Раньше выбирался «ближайший к центру», из-за чего номера пинов у микросхем перемешивались.)
        let mut order: Vec<usize> = (0..components.len()).collect();
        order.sort_by_key(|&ci| {
            let c = &components[ci];
            (c.bx2 as i64 - c.bx1 as i64) * (c.by2 as i64 - c.by1 as i64)
        });
        let mut offset_hist: HashMap<i64, usize> = HashMap::new();
        for &ci in &order {
            let comp = &components[ci];
            let mut missing: Vec<usize> = (0..comp.pins.len()).filter(|&pi| assign[ci][pi].is_none()).collect();
            if missing.is_empty() { continue; }
            missing.sort_by_key(|&pi| comp.pins[pi].pad_index);

            let sides: [bool; 2] = if comp_top[ci] { [true, false] } else { [false, true] };
            for &side_top in sides.iter() {
                if missing.is_empty() { break; }
                let cands: Vec<usize> = {
                    let list = if side_top { &top_pins } else { &bot_pins };
                    let claimed = if side_top { &claimed_top } else { &claimed_bot };
                    list.iter().enumerate()
                        .filter(|(i, p)| !claimed.contains(i) && Self::pin_in_bbox(p, comp, FALLBACK_MARGIN_CMIL))
                        .map(|(i, _)| i)
                        .collect()
                };
                let take = cands.len().min(missing.len());
                for k in 0..take {
                    let pi = missing[k];
                    assign[ci][pi] = Some((side_top, cands[k], 1));
                    if side_top { claimed_top.insert(cands[k]); } else { claimed_bot.insert(cands[k]); }
                    let canon = (comp.pins[pi].pad_index / 8) as i64;
                    *offset_hist.entry(cands[k] as i64 - canon).or_insert(0) += 1;
                }
                missing.drain(0..take);
            }
        }

        // --- Проход 3: пины слоя, которые не взял ни один компонент.
        // Владелец = самый маленький bbox ЭТОЙ ЖЕ стороны. Дубли (то же место у того же владельца,
        // например копия сквозного вывода в другом слое) пропускаем. Имена синтетические (x1, x2…).
        let mut assigned_xy: HashSet<(usize, i32, i32)> = HashSet::new();
        for ci in 0..components.len() {
            for pi in 0..components[ci].pins.len() {
                if let Some((st, idx, _)) = assign[ci][pi] {
                    let p = if st { &top_pins[idx] } else { &bot_pins[idx] };
                    assigned_xy.insert((ci, p.x, p.y));
                }
            }
        }
        // (владелец, сторона_top, индекс)
        let mut strays: Vec<(usize, bool, usize)> = Vec::new();
        for &(list, claimed, side_top) in [(&top_pins, &claimed_top, true), (&bot_pins, &claimed_bot, false)].iter() {
            for (idx, pr) in list.iter().enumerate() {
                if claimed.contains(&idx) { continue; }
                let owner = components.iter().enumerate()
                    .filter(|(ci, c)| comp_top[*ci] == side_top && Self::pin_in_bbox(pr, c, STRAY_MARGIN_CMIL))
                    .min_by_key(|(_, c)| (c.bx2 as i64 - c.bx1 as i64) * (c.by2 as i64 - c.by1 as i64))
                    .map(|(ci, _)| ci);
                if let Some(ci) = owner {
                    if assigned_xy.insert((ci, pr.x, pr.y)) {
                        strays.push((ci, side_top, idx));
                    }
                }
            }
        }

        // --- Сборка BoardPin. Имя пина должно быть уникальным внутри компонента:
        // фронтенд выделяет пины по name, одинаковые имена подсвечиваются разом.
        let resolve_net = |idx: u32| -> String {
            net_names.get(idx as usize).cloned().unwrap_or_default()
        };
        let mut name_seen: HashSet<(usize, String)> = HashSet::new();
        let mut unique_name = |ci: usize, base: String| -> (String, bool) {
            let mut name = base.clone();
            let mut n = 2;
            let mut changed = false;
            while !name_seen.insert((ci, name.clone())) {
                name = format!("{}_{}", base, n);
                n += 1;
                changed = true;
            }
            (name, changed)
        };

        let mut board_pins: Vec<BoardPin> = Vec::new();
        let (mut n_canon, mut n_fallback, mut n_failed, mut n_renamed) = (0usize, 0usize, 0usize, 0usize);
        let mut far_pins = 0usize;
        let mut failure_samples: Vec<String> = Vec::new();

        for (ci, comp) in components.iter().enumerate() {
            for (pi, cp) in comp.pins.iter().enumerate() {
                match assign[ci][pi] {
                    Some((st, idx, kind)) => {
                        let pr = if st { &top_pins[idx] } else { &bot_pins[idx] };
                        if kind == 0 { n_canon += 1; } else { n_fallback += 1; }
                        let base = if cp.name.is_empty() { cp.pin_index.to_string() } else { cp.name.clone() };
                        let (name, changed) = unique_name(ci, base);
                        if changed { n_renamed += 1; }

                        let (_, _, x1, y1, x2, y2) = &part_boxes[ci];
                        let (px, py) = (pr.x as f64 / 100.0, pr.y as f64 / 100.0);
                        if px < x1 - 50.0 || px > x2 + 50.0 || py < y1 - 50.0 || py > y2 + 50.0 {
                            far_pins += 1;
                        }

                        board_pins.push(BoardPin {
                            name: format!("{}.{}", comp.refdes, name),
                            x: px,
                            y: py,
                            net: resolve_net(pr.net_index),
                            side: if st { "top".to_string() } else { "bottom".to_string() },
                        });
                    }
                    None => {
                        n_failed += 1;
                        if failure_samples.len() < 10 {
                            let idx = (cp.pad_index / 8) as usize;
                            failure_samples.push(format!(
                                "{} pin '{}' pad_index={} -> idx={} (top_len={}, bot_len={}) bbox cmil=({},{})-({},{})",
                                comp.refdes, cp.name, cp.pad_index, idx, top_pins.len(), bot_pins.len(),
                                comp.bx1, comp.by1, comp.bx2, comp.by2
                            ));
                        }
                    }
                }
            }
        }

        let mut stray_counter: HashMap<usize, usize> = HashMap::new();
        for &(ci, side_top, idx) in &strays {
            let pr = if side_top { &top_pins[idx] } else { &bot_pins[idx] };
            let n = stray_counter.entry(ci).or_insert(0);
            *n += 1;
            let (name, _) = unique_name(ci, format!("x{}", *n));
            board_pins.push(BoardPin {
                name: format!("{}.{}", components[ci].refdes, name),
                x: pr.x as f64 / 100.0,
                y: pr.y as f64 / 100.0,
                net: resolve_net(pr.net_index),
                side: if side_top { "top".to_string() } else { "bottom".to_string() },
            });
        }

        if cfg!(debug_assertions) {
            eprintln!(
                "[TVW diag] pins: canonical={}, fallback={}, unmapped={}, stray(in bbox, synthetic names)={}, renamed duplicates={}",
                n_canon, n_fallback, n_failed, strays.len(), n_renamed
            );
            eprintln!("[TVW diag] pins farther than 50 mil from own bbox: {}", far_pins);
            let zero_pin_comps = (0..components.len())
                .filter(|&ci| !assign[ci].iter().any(|a| a.is_some()))
                .count();
            eprintln!("[TVW diag] components with 0 mapped pins: {}", zero_pin_comps);
            if !offset_hist.is_empty() {
                let mut h: Vec<(i64, usize)> = offset_hist.iter().map(|(k, v)| (*k, *v)).collect();
                h.sort_by(|a, b| b.1.cmp(&a.1));
                h.truncate(5);
                eprintln!("[TVW diag] fallback index offsets (found_idx - pad_index/8), top5: {:?}", h);
            }
            for s in &failure_samples {
                eprintln!("[TVW diag] unmapped: {}", s);
            }
        }

        // 5. Контур платы
        let mut best: Option<(i64, u32, usize, usize)> = None;
        for (si, s) in all_surfaces.iter().enumerate() {
            if s.vertices.len() < 4 { continue; }
            let (mut min_x, mut max_x, mut min_y, mut max_y) = (i32::MAX, i32::MIN, i32::MAX, i32::MIN);
            for &(vx, vy) in &s.vertices {
                if vx < min_x { min_x = vx; }
                if vx > max_x { max_x = vx; }
                if vy < min_y { min_y = vy; }
                if vy > max_y { max_y = vy; }
            }
            let w = max_x as i64 - min_x as i64;
            let h = max_y as i64 - min_y as i64;
            if w < 50_000 || h < 50_000 { continue; }
            let area = w * h;
            let clean = if s.void_count == 0 { 1u32 } else { 0u32 };
            let cand = (area, clean, s.vertices.len(), si);
            let better = match &best {
                Some(prev) => (cand.0, cand.1, cand.2) > (prev.0, prev.1, prev.2),
                None => true,
            };
            if better { best = Some(cand); }
        }

        let mut board_lines: Vec<GenCadLine> = Vec::new();
        if let Some((_, _, _, si)) = best {
            let v = &all_surfaces[si].vertices;
            for k in 0..v.len() {
                let (ax, ay) = v[k];
                let (bx, by) = v[(k + 1) % v.len()];
                board_lines.push(GenCadLine {
                    x1: ax as f64 / 100.0,
                    y1: ay as f64 / 100.0,
                    x2: bx as f64 / 100.0,
                    y2: by as f64 / 100.0,
                    layer: "BOARD".to_string(),
                });
            }
        }

        if cfg!(debug_assertions) {
            eprintln!(
                "[TVW] nets: {}, components: {}, top_pins: {}, bot_pins: {}, total_pins: {}, outline: {}",
                net_names.len(), board_components.len(), top_pins.len(), bot_pins.len(),
                board_pins.len(), board_lines.len()
            );
        }

        Ok(BoardData {
            format: "tvw".to_string(),
            pins: board_pins,
            lines: board_lines,
            arcs: Vec::new(),
            components: board_components,
            pads: Vec::new(),
            routes: Vec::new(),
        })
    }

    // ------------------------------------------------------------------
    // ASCII
    // ------------------------------------------------------------------

    pub fn parse_ascii(text: &str) -> (Vec<BoardPin>, Vec<GenCadComponent>) {
        let mut pins = Vec::new();
        let mut components = Vec::new();
        let mut current_part: Option<String> = None;
        let mut current_side = "top".to_string();

        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() { continue; }
            if line.starts_with("Part ") {
                let toks: Vec<&str> = line.split_whitespace().collect();
                if toks.len() >= 2 {
                    let refdes = toks[1].to_string();
                    let side = if toks.len() >= 3 && toks[2].contains('B') { "bottom" } else { "top" };
                    current_part = Some(refdes.clone());
                    current_side = side.to_string();
                    components.push(GenCadComponent {
                        ref_name: refdes,
                        x: 0.0, y: 0.0,
                        shape: "".to_string(),
                        layer: if side == "top" { "TOP".to_string() } else { "BOTTOM".to_string() },
                        bounds_x1: 0.0, bounds_y1: 0.0, bounds_x2: 0.0, bounds_y2: 0.0,
                        rotation: 0.0, mirror: false, device: "".to_string(),
                    });
                }
                continue;
            }
            if let Some(ref refdes) = current_part {
                let toks: Vec<&str> = line.split_whitespace().collect();
                if toks.len() >= 6 && toks[0].chars().all(|c| c.is_numeric()) {
                    let pin_name = toks[1].to_string();
                    if let (Ok(x), Ok(y)) = (toks[2].parse::<f64>(), toks[3].parse::<f64>()) {
                        let net = if toks[5] == "(NC)" { "".to_string() } else { toks[5].to_string() };
                        pins.push(BoardPin {
                            name: format!("{}.{}", refdes, pin_name),
                            x: x * 1000.0,
                            y: y * 1000.0,
                            net,
                            side: current_side.clone(),
                        });
                    }
                }
            }
        }
        (pins, components)
    }
}