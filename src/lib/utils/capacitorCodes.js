/**
 * Fixitron — парсеры и форматтеры для конденсаторов.
 */

// ============================================================
// Форматтеры
// ============================================================

/**
 * Форматирует ёмкость в удобочитаемый вид.
 * Возвращает объект { value: '3.1', unit: 'uf' }
 */
export function formatFarad(value) {
  if (value == null || isNaN(value)) return { value: '—', unit: '' };
  if (value === 0) return { value: '0', unit: 'f' };

  const abs = Math.abs(value);

  if (abs >= 1)      return { value: trimNum(value), unit: 'f' };
  if (abs >= 1e-3)   return { value: trimNum(value * 1e3), unit: 'mf' };
  if (abs >= 1e-6)   return { value: trimNum(value * 1e6), unit: 'uf' };
  if (abs >= 1e-9)   return { value: trimNum(value * 1e9), unit: 'nf' };
  if (abs >= 1e-12)  return { value: trimNum(value * 1e12), unit: 'pf' };
  return { value: trimNum(value * 1e12), unit: 'pf' };
}

/**
 * Компактный формат: 100н, 1мк, 4.7п, 2.2м.
 * Оставляем строкой, так как это техническая нотация.
 */
export function formatFaradCompact(value) {
  if (value == null || isNaN(value)) return '—';
  if (value === 0) return '0';

  const abs = Math.abs(value);
  if (abs >= 1)      return `${trimNum(value)}Ф`;
  if (abs >= 1e-3)   return `${trimNum(value * 1e3)}м`;
  if (abs >= 1e-6)   return `${trimNum(value * 1e6)}мк`;
  if (abs >= 1e-9)   return `${trimNum(value * 1e9)}н`;
  if (abs >= 1e-12)  return `${trimNum(value * 1e12)}п`;
  return `${trimNum(value * 1e12)}п`;
}

function trimNum(n) {
  const rounded = Math.round(n * 100) / 100;
  return String(rounded);
}

/**
 * Обратный парсер: "100н" / "0.1 мкФ" / "4.7p" → число (Фарады).
 */
export function parseFarad(input) {
  if (input == null) return null;
  let str = String(input).trim().replace(/\s+/g, '').replace(',', '.');
  if (!str) return null;

  // R-нотация: "4R7" → 4.7 (пФ по умолчанию)
  if (/^\d+[rR]\d*$/.test(str)) {
    const parts = str.toLowerCase().split('r');
    const num = parseFloat(`${parts[0]}.${parts[1] || '0'}`);
    return isNaN(num) ? null : num * 1e-12;
  }

  const match = str.match(/^(-?\d+(?:[.,]\d+)?)([A-Za-zА-Яа-яЁё]*)$/);
  if (!match) return null;

  const num = parseFloat(match[1].replace(',', '.'));
  const unitLower = match[2].toLowerCase();
  if (isNaN(num)) return null;

  switch (unitLower) {
    case '':
    case 'p':
    case 'пф':
    case 'pf':
      return num * 1e-12;
    case 'n':
    case 'нф':
    case 'nf':
      return num * 1e-9;
    case 'u':
    case 'мкф':
    case 'uf':
    case 'мк':
      return num * 1e-6;
    case 'm':
    case 'мф':
    case 'mf':
      return num * 1e-3;
    case 'f':
    case 'ф':
      return num;
    default:
      return null;
  }
}

// ============================================================
// Парсер буквенно-цифровой маркировки (керамика, плёнка)
// ============================================================

/**
 * Парсит код маркировки конденсатора.
 */
export function parseCapacitorMarking(raw) {
  if (raw == null) return null;
  let code = String(raw).trim();
  if (!code) return null;

  // 1. Извлекаем напряжение ДО удаления пробелов
  let voltage = null;
  const voltMatch = code.match(/(\d+(?:\.\d+)?)\s*[VВ]/i);
  if (voltMatch) {
    voltage = parseFloat(voltMatch[1]);
    code = code.replace(voltMatch[0], '');
  }

  // 2. Убираем пробелы и приводим к верхнему регистру
  code = code.replace(/\s+/g, '').toUpperCase();
  if (!code) return null;

  // 3. Извлекаем допуск (буква в конце: J=5%, K=10%, M=20%, G=2%, F=1%)
  const toleranceMap = { J: 5, K: 10, M: 20, G: 2, F: 1 };
  let tolerance = null;
  const tolMatch = code.match(/([JKMG F])$/);
  if (tolMatch && toleranceMap[tolMatch[1]]) {
    tolerance = toleranceMap[tolMatch[1]];
    code = code.slice(0, -1);
  }

  let value = null;

  // 4. R-нотация: 4R7 → 4.7 пФ, 0R5 → 0.5 пФ
  if (/^\d+[R]\d+$/.test(code)) {
    value = parseFloat(code.replace('R', '.')) * 1e-12;
  }
  // 5. Буквы p, n, u на месте запятой: 3n3 → 3.3 нФ, 6p8 → 6.8 пФ, n10 → 0.1 нФ
  else if (/^\d*[PNU]\d+$/.test(code) || /^[PNU]\d+$/.test(code)) {
    const mult = code.includes('P') ? 1e-12 : code.includes('N') ? 1e-9 : 1e-6;
    value = parseFloat(code.replace(/[PNU]/, '.')) * mult;
  }
  // 6. Чисто цифры: 104 → 10 * 10^4 пФ = 100 нФ
  else if (/^\d{3}$/.test(code)) {
    const base = parseInt(code.slice(0, 2), 10);
    const mult = parseInt(code[2], 10);
    value = base * Math.pow(10, mult) * 1e-12;
  }

  if (value === null) return null;

  return { value, tolerance, voltage, raw: String(raw).trim() };
}

// ============================================================
// SMD-парсер (танталовые / электролитические)
// ============================================================

const SMD_MANTISSA = {
  A: 1,   B: 1.1, C: 1.2, D: 1.3, E: 1.5, F: 1.6, G: 1.8, H: 2,   J: 2.2,
  K: 2.4, L: 2.7, M: 3,   N: 3.3, P: 3.6, Q: 3.9, R: 4.3, S: 4.7, T: 5.1,
  U: 5.6, V: 6.2, W: 6.8, X: 7.5, Y: 8.2, Z: 9.1
};

export function parseSmdCapacitor(raw) {
  if (raw == null) return null;
  const code = String(raw).trim().toUpperCase();
  if (!code) return null;

  // Буква + цифра: J3, A0, Z5
  if (/^[A-Z]\d$/.test(code)) {
    const mant = SMD_MANTISSA[code[0]];
    if (mant == null) return null;
    const mult = parseInt(code[1], 10);
    // Значение в пФ
    const value = mant * Math.pow(10, mult) * 1e-12;
    return { value, type: 'tant', raw: code, warning: null };
  }

  // 3-значный код (105): как керамика, но может быть в мкФ
  if (/^\d{3}$/.test(code)) {
    const base = parseInt(code.slice(0, 2), 10);
    const mult = parseInt(code[2], 10);
    const valueP = base * Math.pow(10, mult) * 1e-12;
    return {
      value: valueP,
      type: 'ceramic',
      raw: code,
      warning: 'smd_ceramic_warning'
    };
  }

  return null;
}

// ============================================================
// Цветовая маркировка конденсаторов
// ============================================================

/**
 * Цвета для конденсаторов (отличаются от резисторов).
 * Множитель — это степень 10 от базового значения в пФ.
 */
export const CAPACITOR_COLORS = [
  { id: 'black',  name: 'Чёрный',     digit: 0, multiplier: 1e0,  hex: '#0f0f0f' },
  { id: 'brown',  name: 'Коричневый', digit: 1, multiplier: 1e1,  hex: '#7a4a1e' },
  { id: 'red',    name: 'Красный',    digit: 2, multiplier: 1e2,  hex: '#d32f2f' },
  { id: 'orange', name: 'Оранжевый',  digit: 3, multiplier: 1e3,  hex: '#f57c00' },
  { id: 'yellow', name: 'Жёлтый',     digit: 4, multiplier: 1e4,  hex: '#fbc02d' },
  { id: 'green',  name: 'Зелёный',    digit: 5, multiplier: 1e5,  hex: '#388e3c' },
  { id: 'blue',   name: 'Синий',      digit: 6, multiplier: 1e6,  hex: '#1976d2' },
  { id: 'violet', name: 'Фиолетовый', digit: 7, multiplier: 1e7,  hex: '#7b1fa2' },
  { id: 'gray',   name: 'Серый',      digit: 8, multiplier: 1e8,  hex: '#757575' },
  { id: 'white',  name: 'Белый',      digit: 9, multiplier: 1e9,  hex: '#f5f5f5' }
];

const CAP_COLOR_BY_ID = new Map(CAPACITOR_COLORS.map((c) => [c.id, c]));

export function parseCapacitorColors(colors) {
  if (!Array.isArray(colors)) return null;
  const n = colors.length;
  if (n < 3 || n > 5) return null;

  const metas = colors.map((id) => CAP_COLOR_BY_ID.get(String(id).toLowerCase()));
  if (metas.some((m) => !m)) return null;

  const [c1, c2, c3] = metas;
  if (c1.digit == null || c2.digit == null || c3.multiplier == null) return null;

  let value = (c1.digit * 10 + c2.digit) * c3.multiplier * 1e-12;

  // Допуск (4-я полоса) — упрощённая карта
  let tolerance = null;
  if (n >= 4) {
    const tolColor = metas[3].id;
    const tolMap = { white: 10, black: 20, green: 5, red: 2, brown: 1 };
    tolerance = tolMap[tolColor] ?? null;
  }

  // Напряжение (5-я полоса) — упрощённая карта
  let voltage = null;
  if (n >= 5) {
    const vColor = metas[4].id;
    const vMap = { red: 250, yellow: 400, blue: 630, green: 100 };
    voltage = vMap[vColor] ?? null;
  }

  return {
    value,
    tolerance,
    voltage,
    bands: n,
    warning: 'color_interpretation'
  };
}