/**
 * Fixitron — парсеры и форматтеры для резисторов.
 *
 * Чистая логика без Svelte. Компоненты только вызывают эти функции.
 *
 * Основные экспорты:
 *  - parseSmd(code)              — SMD-код (3, 4 знака, EIA-96, R-нотация)
 *  - parseColorBands(colors)     — цветовая маркировка (3-6 полос)
 *  - formatOhm(value)            — "10 кОм", "4.7 кОм", "1 МОм"
 *  - formatOhmCompact(value)     — "10к", "4.7к", "1М"
 *  - parseOhm(string)            — "10к" → 10000
 *  - RESISTOR_COLORS             — список цветов с метаданными
 *  - EIA96_CODES                 — таблица EIA-96 (код → значение)
 */

// ============================================================
// Константы
// ============================================================

/**
 * Цвета резисторов.
 *  - digit:     значение цифры (для полос 1-3)
 *  - multiplier: множитель (степень 10) для полосы-множителя
 *  - tolerance: допуск в % (null если цвет не используется как допуск)
 *  - tcr:       ТКС в ppm/°C (null если не используется как ТКС)
 *  - hex:       цвет для UI
 */
export const RESISTOR_COLORS = [
  { id: 'black',  name: 'Чёрный',       digit: 0, multiplier: 1e0,  tolerance: null, tcr: 250,  hex: '#0f0f0f' },
  { id: 'brown',  name: 'Коричневый',   digit: 1, multiplier: 1e1,  tolerance: 1,    tcr: 100,  hex: '#7a4a1e' },
  { id: 'red',    name: 'Красный',      digit: 2, multiplier: 1e2,  tolerance: 2,    tcr: 50,   hex: '#d32f2f' },
  { id: 'orange', name: 'Оранжевый',    digit: 3, multiplier: 1e3,  tolerance: null, tcr: 15,   hex: '#f57c00' },
  { id: 'yellow', name: 'Жёлтый',       digit: 4, multiplier: 1e4,  tolerance: null, tcr: 25,   hex: '#fbc02d' },
  { id: 'green',  name: 'Зелёный',      digit: 5, multiplier: 1e5,  tolerance: 0.5,  tcr: null, hex: '#388e3c' },
  { id: 'blue',   name: 'Синий',        digit: 6, multiplier: 1e6,  tolerance: 0.25, tcr: 10,   hex: '#1976d2' },
  { id: 'violet', name: 'Фиолетовый',   digit: 7, multiplier: 1e7,  tolerance: 0.1,  tcr: 5,    hex: '#7b1fa2' },
  { id: 'gray',   name: 'Серый',        digit: 8, multiplier: 1e8,  tolerance: 0.05, tcr: 1,    hex: '#757575' },
  { id: 'white',  name: 'Белый',        digit: 9, multiplier: 1e9,  tolerance: null, tcr: null, hex: '#f5f5f5' },
  { id: 'gold',   name: 'Золотой',      digit: null, multiplier: 1e-1, tolerance: 5, tcr: null, hex: '#c9a227' },
  { id: 'silver', name: 'Серебряный',   digit: null, multiplier: 1e-2, tolerance: 10, tcr: null, hex: '#b0b0b0' }
];

const COLOR_BY_ID = new Map(RESISTOR_COLORS.map((c) => [c.id, c]));

/**
 * Таблица EIA-96: двухзначный код → базовое значение.
 * Стандарт для точных резисторов (1%).
 * Значения: от 100 до 976 (округлённые по ряду E96).
 */
export const EIA96_CODES = {
  '01': 100, '02': 102, '03': 105, '04': 107, '05': 110, '06': 113, '07': 115, '08': 118,
  '09': 121, '10': 124, '11': 127, '12': 130, '13': 133, '14': 137, '15': 140, '16': 143,
  '17': 147, '18': 150, '19': 154, '20': 158, '21': 162, '22': 165, '23': 169, '24': 174,
  '25': 178, '26': 182, '27': 187, '28': 191, '29': 196, '30': 200, '31': 205, '32': 210,
  '33': 215, '34': 221, '35': 226, '36': 232, '37': 237, '38': 243, '39': 249, '40': 255,
  '41': 261, '42': 267, '43': 274, '44': 280, '45': 287, '46': 294, '47': 301, '48': 309,
  '49': 316, '50': 324, '51': 332, '52': 340, '53': 348, '54': 357, '55': 365, '56': 374,
  '57': 383, '58': 392, '59': 402, '60': 412, '61': 422, '62': 432, '63': 442, '64': 453,
  '65': 464, '66': 475, '67': 487, '68': 499, '69': 511, '70': 523, '71': 536, '72': 549,
  '73': 562, '74': 576, '75': 590, '76': 604, '77': 619, '78': 634, '79': 649, '80': 665,
  '81': 681, '82': 698, '83': 715, '84': 732, '85': 750, '86': 768, '87': 787, '88': 806,
  '89': 825, '90': 845, '91': 866, '92': 887, '93': 909, '94': 931, '95': 953, '96': 976
};

/**
 * Буквы-множители EIA-96.
 *  Y=1e-2, X=1e-1, A=1e0, B=1e1, C=1e2, D=1e3, E=1e4, F=1e5, ...
 *  Важно: в EIA-96 нет I, O, Q, S, Z (не используются).
 */
const EIA96_MULTIPLIERS = {
  Y: 1e-2, R: 1e-2,
  X: 1e-1, S: 1e-1,
  A: 1e0,
  B: 1e1, H: 1e1,
  C: 1e2,
  D: 1e3,
  E: 1e4,
  F: 1e5
};

// ============================================================
// Утилиты форматирования
// ============================================================

/**
 * Форматирует сопротивление в удобочитаемый вид с полным названием единицы.
 *  10000    → "10 кОм"
 *  4700     → "4.7 кОм"
 *  1000000  → "1 МОм"
 *  0.22     → "220 мОм"
 *  0        → "0 Ом"
 */
export function formatOhm(value) {
  if (value == null || isNaN(value)) return '—';
  if (value === 0) return '0 Ом';

  const abs = Math.abs(value);

  if (abs >= 1e6) return `${trimNum(value / 1e6)} МОм`;
  if (abs >= 1e3) return `${trimNum(value / 1e3)} кОм`;
  if (abs >= 1)   return `${trimNum(value)} Ом`;
  if (abs >= 1e-3) return `${trimNum(value * 1e3)} мОм`;
  return `${trimNum(value)} Ом`;
}

/**
 * Компактный формат: 10к, 4.7к, 1М, 220м, 0R22.
 */
export function formatOhmCompact(value) {
  if (value == null || isNaN(value)) return '—';
  if (value === 0) return '0R';

  const abs = Math.abs(value);

  if (abs >= 1e6) return `${trimNum(value / 1e6)}М`;
  if (abs >= 1e3) return `${trimNum(value / 1e3)}к`;
  if (abs >= 1)   return `${trimNum(value)}R`;
  if (abs >= 1e-3) return `${trimNum(value * 1e3)}м`;
  return `${trimNum(value)}R`;
}

/**
 * Тримминг числа до 3 значащих цифр без хвостовых нулей.
 * 10.0 → "10", 4.70 → "4.7", 1.000 → "1"
 */
function trimNum(n) {
  const rounded = Math.round(n * 100) / 100;
  return String(rounded);
}

/**
 * Обратный парсер: "10к" / "4.7 кОм" / "1M" / "0R22" → число (Омы).
 * Возвращает null, если не удалось распарсить.
 *
 * Правила единиц (регистр важен для М/м):
 *   - "M" / "М" (заглавная) и "meg" / "мега"  → мега (×1e6)
 *   - "m" / "м" (строчная), "mom" / "мом"     → милли (×1e-3)
 *   - "k" / "к", "ком"                         → кило (×1e3)
 *   - "" / "r" / "ом" / "o"                    → Омы
 */
export function parseOhm(input) {
  if (input == null) return null;

  // Работаем с оригиналом, чтобы поймать регистр М/м.
  let str = String(input).trim().replace(/\s+/g, '');
  if (!str) return null;

  // R-нотация: "4R7", "0R22", "4r70" — регистр R не важен
  if (/^\d+[rR]\d*$/.test(str)) {
    const parts = str.toLowerCase().split('r');
    const intPart = parts[0];
    const fracPart = parts[1] || '0';
    const num = parseFloat(`${intPart}.${fracPart || '0'}`);
    return isNaN(num) ? null : num;
  }

  // Отделяем число от суффикса-единицы
  const match = str.match(/^(-?\d+(?:[.,]\d+)?)([A-Za-zА-Яа-яЁё]*)$/);
  if (!match) return null;

  const num = parseFloat(match[1].replace(',', '.'));
  const unitRaw = match[2];
  if (isNaN(num)) return null;

  const unitLower = unitRaw.toLowerCase();

  // Мега — только если в оригинале есть заглавная M/М, либо явное "meg"/"мега"
  const isMega =
    unitRaw === 'M' ||
    unitRaw === 'М' ||
    unitLower === 'meg' ||
    unitLower === 'мега' ||
    unitLower === 'mom' && unitRaw[0] === 'M' ||
    unitLower === 'мом' && unitRaw[0] === 'М';

  if (isMega) return num * 1e6;

  switch (unitLower) {
    case '':
    case 'r':
    case 'ом':
    case 'o':
      return num;
    case 'k':
    case 'ком':
    case 'к':
      return num * 1e3;
    case 'm':
    case 'м':
    case 'mom':
    case 'мом':
      return num * 1e-3;
    default:
      return null;
  }
}

// ============================================================
// Парсер SMD-кодов
// ============================================================

/**
 * Универсальный парсер SMD-кода резистора.
 * Поддерживает:
 *  - 3-значные:  "103" → 10 кОм, "4R7" → 4.7 Ом, "0R0" → 0
 *  - 4-значные:  "1002" → 10 кОм, "4R70" → 4.7 Ом
 *  - EIA-96:     "01C" → 10 кОм
 *
 * Возвращает:
 *   { value: number, tolerance: number|null, standard: '3digit'|'4digit'|'eia96', raw: string }
 *   или null, если код некорректен.
 */
export function parseSmd(raw) {
  if (raw == null) return null;
  const code = String(raw).trim().toUpperCase();
  if (!code) return null;

  // EIA-96: 2 цифры + буква
  if (/^\d{2}[A-Z]$/.test(code)) {
    const base = EIA96_CODES[code.slice(0, 2)];
    const letter = code[2];
    if (base == null) return null;
    const mult = EIA96_MULTIPLIERS[letter];
    if (mult == null) return null;
    return {
      value: base * mult,
      tolerance: 1,
      standard: 'eia96',
      raw: code
    };
  }

  // R-нотация внутри 3-4 знаков: "4R7", "0R22", "4R70"
  if (/^\d*R\d+$/.test(code) || /^\d+R\d*$/.test(code)) {
    // Заменяем R на точку: "4R7" → 4.7, "0R22" → 0.22, "4R70" → 4.70
    const asNum = code.replace('R', '.');
    const num = parseFloat(asNum);
    if (isNaN(num)) return null;
    return {
      value: num,
      tolerance: null,
      standard: code.length <= 4 ? '3digit' : '4digit',
      raw: code
    };
  }

  // Чисто числовые 3-4 знака
  if (/^\d{3}$/.test(code)) {
    const base = parseInt(code.slice(0, 2), 10);
    const mult = parseInt(code[2], 10);
    return {
      value: base * Math.pow(10, mult),
      tolerance: null,
      standard: '3digit',
      raw: code
    };
  }
  if (/^\d{4}$/.test(code)) {
    const base = parseInt(code.slice(0, 3), 10);
    const mult = parseInt(code[3], 10);
    return {
      value: base * Math.pow(10, mult),
      tolerance: null,
      standard: '4digit',
      raw: code
    };
  }

  return null;
}

// ============================================================
// Парсер цветовой маркировки
// ============================================================

/**
 * Парсит цветовую маркировку резистора.
 *
 * @param {string[]} colors — массив id цветов, например ['brown', 'black', 'red', 'gold']
 *   Поддерживается 3, 4, 5 или 6 полос.
 *
 * @returns {{ value, tolerance, tcr, bands }} | null
 *   bands: количество полос, для справки
 */
export function parseColorBands(colors) {
  if (!Array.isArray(colors)) return null;
  const n = colors.length;
  if (n < 3 || n > 6) return null;

  const metas = colors.map((id) => COLOR_BY_ID.get(String(id).toLowerCase()));
  if (metas.some((m) => !m)) return null;

  // --- 3 полосы: 2 цифры + множитель (без допуска) ---
  if (n === 3) {
    const [d1, d2, m] = metas;
    if (d1.digit == null || d2.digit == null || m.multiplier == null) return null;
    return {
      value: (d1.digit * 10 + d2.digit) * m.multiplier,
      tolerance: null,
      tcr: null,
      bands: 3
    };
  }

  // --- 4 полосы: 2 цифры + множитель + допуск ---
  if (n === 4) {
    const [d1, d2, m, tol] = metas;
    if (d1.digit == null || d2.digit == null || m.multiplier == null) return null;
    return {
      value: (d1.digit * 10 + d2.digit) * m.multiplier,
      tolerance: tol.tolerance,
      tcr: null,
      bands: 4
    };
  }

  // --- 5 полос: 3 цифры + множитель + допуск ---
  if (n === 5) {
    const [d1, d2, d3, m, tol] = metas;
    if (d1.digit == null || d2.digit == null || d3.digit == null || m.multiplier == null) return null;
    return {
      value: (d1.digit * 100 + d2.digit * 10 + d3.digit) * m.multiplier,
      tolerance: tol.tolerance,
      tcr: null,
      bands: 5
    };
  }

  // --- 6 полос: 3 цифры + множитель + допуск + ТКС ---
  if (n === 6) {
    const [d1, d2, d3, m, tol, tcr] = metas;
    if (d1.digit == null || d2.digit == null || d3.digit == null || m.multiplier == null) return null;
    return {
      value: (d1.digit * 100 + d2.digit * 10 + d3.digit) * m.multiplier,
      tolerance: tol.tolerance,
      tcr: tcr.tcr,
      bands: 6
    };
  }

  return null;
}