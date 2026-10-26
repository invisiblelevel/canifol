/**
 * Canifol — парсеры и форматтеры для катушек индуктивности.
 */

// ============================================================
// Форматтеры
// ============================================================

export function formatHenry(value) {
  if (value == null || isNaN(value)) return { value: '—', unit: '' };
  if (value === 0) return { value: '0', unit: 'h' };

  const abs = Math.abs(value);

  if (abs >= 1)      return { value: trimNum(value), unit: 'h' };
  if (abs >= 1e-3)   return { value: trimNum(value * 1e3), unit: 'mh' };
  if (abs >= 1e-6)   return { value: trimNum(value * 1e6), unit: 'uh' };
  if (abs >= 1e-9)   return { value: trimNum(value * 1e9), unit: 'nh' };
  return { value: trimNum(value * 1e9), unit: 'nh' };
}

export function formatHenryCompact(value) {
  if (value == null || isNaN(value)) return '—';
  if (value === 0) return '0';

  const abs = Math.abs(value);

  if (abs >= 1)      return `${trimNum(value)}Г`;
  if (abs >= 1e-3)   return `${trimNum(value * 1e3)}мг`;
  if (abs >= 1e-6)   return `${trimNum(value * 1e6)}мкг`;
  if (abs >= 1e-9)   return `${trimNum(value * 1e9)}нг`;
  return `${trimNum(value * 1e9)}нг`;
}

function trimNum(n) {
  const rounded = Math.round(n * 100) / 100;
  return String(rounded);
}

export function parseHenry(input) {
  if (input == null) return null;
  let str = String(input).trim().replace(/\s+/g, '').replace(',', '.');
  if (!str) return null;

  const match = str.match(/^(-?\d+(?:[.,]\d+)?)([A-Za-zА-Яа-яЁё]*)$/);
  if (!match) return null;

  const num = parseFloat(match[1].replace(',', '.'));
  const unitLower = match[2].toLowerCase();
  if (isNaN(num)) return null;

  switch (unitLower) {
    case '': case 'h': case 'гн': return num;
    case 'm': case 'мг': case 'мгн': case 'mh': return num * 1e-3;
    case 'u': case 'мкг': case 'мкгн': case 'uh': case 'mk': return num * 1e-6;
    case 'n': case 'нг': case 'нгн': case 'nh': return num * 1e-9;
    default: return null;
  }
}

// ============================================================
// Парсер SMD-кодов индуктивностей
// ============================================================

export function parseSmdInductor(raw) {
  if (raw == null) return null;
  const code = String(raw).trim().toUpperCase();
  if (!code) return null;

  if (/^\d*R\d+$/.test(code) || /^\d+R\d*$/.test(code)) {
    const normalized = code.startsWith('R') ? `0${code}` : code;
    const num = parseFloat(normalized.replace('R', '.'));
    if (isNaN(num)) return null;
    return { value: num * 1e-6, standard: 'R-нотация', raw: code };
  }

  if (/^\d{3}$/.test(code)) {
    const base = parseInt(code.slice(0, 2), 10);
    const mult = parseInt(code[2], 10);
    const valueUh = base * Math.pow(10, mult);
    return { value: valueUh * 1e-6, standard: '3-значный', raw: code };
  }

  if (/^\d{4}$/.test(code)) {
    const base = parseInt(code.slice(0, 3), 10);
    const mult = parseInt(code[3], 10);
    const valueUh = base * Math.pow(10, mult);
    return { value: valueUh * 1e-6, standard: '4-значный', raw: code };
  }

  return null;
}

// ============================================================
// Цветовая маркировка катушек (MIL)
// ============================================================

export const INDUCTOR_COLORS = [
  { id: 'black',  name: 'Чёрный',     digit: 0, multiplier: 1e0, tolerance: null, hex: '#0f0f0f' },
  { id: 'brown',  name: 'Коричневый', digit: 1, multiplier: 1e1, tolerance: 1,    hex: '#7a4a1e' },
  { id: 'red',    name: 'Красный',    digit: 2, multiplier: 1e2, tolerance: 2,    hex: '#d32f2f' },
  { id: 'orange', name: 'Оранжевый',  digit: 3, multiplier: 1e3, tolerance: 3,    hex: '#f57c00' },
  { id: 'yellow', name: 'Жёлтый',     digit: 4, multiplier: 1e4, tolerance: 4,    hex: '#fbc02d' },
  { id: 'green',  name: 'Зелёный',    digit: 5, multiplier: 1e5, tolerance: 5,    hex: '#388e3c' },
  { id: 'blue',   name: 'Синий',      digit: 6, multiplier: 1e6, tolerance: 6,    hex: '#1976d2' },
  { id: 'violet', name: 'Фиолетовый', digit: 7, multiplier: 1e7, tolerance: 7,    hex: '#7b1fa2' },
  { id: 'gray',   name: 'Серый',      digit: 8, multiplier: 1e8, tolerance: 8,    hex: '#757575' },
  { id: 'white',  name: 'Белый',      digit: 9, multiplier: 1e9, tolerance: 9,    hex: '#f5f5f5' },
  { id: 'gold',   name: 'Золотой',    digit: null, multiplier: 1e-1, tolerance: 5, hex: '#c9a227' },
  { id: 'silver', name: 'Серебряный', digit: null, multiplier: 1e-2, tolerance: 10, hex: '#b0b0b0' }
];

const INDUCTOR_COLOR_BY_ID = new Map(INDUCTOR_COLORS.map((c) => [c.id, c]));

export function parseInductorColors(colors) {
  if (!Array.isArray(colors)) return null;
  const n = colors.length;
  if (n < 3 || n > 5) return null;

  const metas = colors.map((id) => INDUCTOR_COLOR_BY_ID.get(String(id).toLowerCase()));
  if (metas.some((m) => !m)) return null;

  const [d1, d2, m] = metas;
  if (d1.digit == null || d2.digit == null || m.multiplier == null) return null;

  const base = d1.digit * 10 + d2.digit;
  const mult = m.multiplier;
  const valueUh = base * mult;

  let tolerance = null;
  if (n >= 4) {
    tolerance = metas[3].tolerance;
  }

  return {
    value: valueUh * 1e-6,
    tolerance,
    bands: n,
    warning: null
  };
}