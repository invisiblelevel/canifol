/**
 * Fixitron — электрические формулы.
 *
 * Чистая логика без Svelte.
 *
 * Все функции принимают числа в базовых единицах:
 *  - напряжение: В
 *  - ток: А
 *  - сопротивление: Ом
 *  - мощность: Вт
 *
 * Возвращают объект с результатами или null, если рассчитать нельзя.
 */

/**
 * Закон Ома + мощность.
 * На вход — объект с ЛЮБЫМИ ДВУМЯ заполненными полями из { u, i, r, p }.
 * Остальные — undefined/null.
 *
 * Возвращает { u, i, r, p } со всеми заполненными значениями,
 * или null, если входных данных недостаточно или они противоречивы.
 *
 * Формулы:
 *   u = i * r
 *   i = u / r
 *   r = u / i
 *   p = u * i = u²/r = i²*r
 */
export function solveOhmLaw({ u, i, r, p }) {
  const given = { u, i, r, p };
  const known = Object.keys(given).filter((k) => isFinite(given[k]) && given[k] !== null && given[k] !== undefined);
  if (known.length !== 2) return null;

  const [a, b] = known;
  const out = { u: null, i: null, r: null, p: null };
  out[a] = given[a];
  out[b] = given[b];

  // Вспомогательная проверка на нули и отрицательные сопротивления
  const safeDiv = (num, den) => (den === 0 ? null : num / den);

  try {
    // U, I → R, P
    if (a === 'u' && b === 'i' || a === 'i' && b === 'u') {
      out.r = safeDiv(out.u, out.i);
      out.p = out.u * out.i;
    }
    // U, R → I, P
    else if (a === 'u' && b === 'r' || a === 'r' && b === 'u') {
      out.i = safeDiv(out.u, out.r);
      out.p = out.i != null ? out.u * out.i : null;
    }
    // U, P → I, R
    else if (a === 'u' && b === 'p' || a === 'p' && b === 'u') {
      out.i = safeDiv(out.p, out.u);
      out.r = out.i != null ? safeDiv(out.u, out.i) : null;
    }
    // I, R → U, P
    else if (a === 'i' && b === 'r' || a === 'r' && b === 'i') {
      out.u = out.i * out.r;
      out.p = out.i * out.i * out.r;
    }
    // I, P → U, R
    else if (a === 'i' && b === 'p' || a === 'p' && b === 'i') {
      out.u = safeDiv(out.p, out.i);
      out.r = out.i !== 0 ? safeDiv(out.u, out.i) : null;
    }
    // R, P → U, I
    else if (a === 'r' && b === 'p' || a === 'p' && b === 'r') {
      // U = √(P·R), I = √(P/R)
      if (out.r >= 0 && out.p >= 0) {
        out.u = Math.sqrt(out.p * out.r);
        out.i = out.r !== 0 ? Math.sqrt(out.p / out.r) : null;
      }
    }
  } catch {
    return null;
  }

  // Проверка: все значения должны быть валидными числами (или null)
  return out;
}

/**
 * Форматирование напряжения.
 *  12      → "12 В"
 *  0.5     → "500 мВ"
 *  5000    → "5 кВ"
 */
export function formatVoltage(v) {
  if (v == null || isNaN(v)) return '—';
  const abs = Math.abs(v);
  if (abs >= 1000) return `${trim(v / 1000)} кВ`;
  if (abs >= 1)    return `${trim(v)} В`;
  if (abs >= 1e-3) return `${trim(v * 1e3)} мВ`;
  return `${trim(v)} В`;
}

/**
 * Форматирование тока.
 *  0.5    → "500 мА"
 *  0.01   → "10 мА"
 *  1      → "1 А"
 */
export function formatCurrent(a) {
  if (a == null || isNaN(a)) return '—';
  const abs = Math.abs(a);
  if (abs >= 1)     return `${trim(a)} А`;
  if (abs >= 1e-3)  return `${trim(a * 1e3)} мА`;
  if (abs >= 1e-6)  return `${trim(a * 1e6)} мкА`;
  return `${trim(a)} А`;
}

/**
 * Форматирование мощности.
 *  0.25    → "250 мВт"
 *  2       → "2 Вт"
 */
export function formatPower(w) {
  if (w == null || isNaN(w)) return '—';
  const abs = Math.abs(w);
  if (abs >= 1000)   return `${trim(w / 1000)} кВт`;
  if (abs >= 1)      return `${trim(w)} Вт`;
  if (abs >= 1e-3)   return `${trim(w * 1e3)} мВт`;
  if (abs >= 1e-6)   return `${trim(w * 1e6)} мкВт`;
  return `${trim(w)} Вт`;
}

function trim(n) {
  const rounded = Math.round(n * 1000) / 1000;
  return String(rounded);
}

/**
 * Суммарное сопротивление при последовательном соединении.
 * @param {number[]} values — массив сопротивлений в Омах
 * @returns {number|null}
 */
export function sumSeries(values) {
  if (!Array.isArray(values) || values.length === 0) return null;
  let sum = 0;
  for (const v of values) {
    if (!isFinite(v) || v < 0) return null;
    sum += v;
  }
  return sum;
}

/**
 * Суммарное сопротивление при параллельном соединении.
 * @param {number[]} values — массив сопротивлений в Омах
 * @returns {number|null}
 *
 * Особые случаи:
 *  - если хоть один резистор = 0 → результат 0 (закорот)
 *  - если хотя бы одно значение невалидно → null
 */
export function sumParallel(values) {
  if (!Array.isArray(values) || values.length === 0) return null;
  let invSum = 0;
  for (const v of values) {
    if (!isFinite(v) || v < 0) return null;
    if (v === 0) return 0; // закорот
    invSum += 1 / v;
  }
  if (invSum === 0) return null;
  return 1 / invSum;
}

/**
 * Делитель напряжения: даны 3 из 4 (uin, uout, r1, r2),
 * считаем 4-е + ток через делитель.
 *
 * @param {{ uin?, uout?, r1?, r2? }} given
 * @returns {{ uin, uout, r1, r2, current, ratio } | null}
 */
export function solveVoltageDivider({ uin, uout, r1, r2 }) {
  const given = { uin, uout, r1, r2 };
  const known = Object.keys(given).filter(
    (k) => given[k] !== null && given[k] !== undefined && isFinite(given[k])
  );
  if (known.length !== 3) return null;

  const out = { uin, uout, r1, r2 };

  // Uin, Uout, R2 → R1
  if (known.includes('uin') && known.includes('uout') && known.includes('r2') && out.r1 == null) {
    if (out.uout === 0) return null; // делим на 0
    if (out.uout > out.uin) return null; // невозможно
    out.r1 = out.r2 * (out.uin / out.uout - 1);
  }
  // Uin, Uout, R1 → R2
  else if (known.includes('uin') && known.includes('uout') && known.includes('r1') && out.r2 == null) {
    if (out.uout === 0) return null;
    if (out.uout > out.uin) return null;
    const denom = out.uin / out.uout - 1;
    if (denom === 0) return null;
    out.r2 = out.r1 / denom;
  }
  // Uout, R1, R2 → Uin
  else if (known.includes('uout') && known.includes('r1') && known.includes('r2') && out.uin == null) {
    if (out.r2 === 0) return null;
    out.uin = (out.uout * (out.r1 + out.r2)) / out.r2;
  }
  // Uin, R1, R2 → Uout
  else if (known.includes('uin') && known.includes('r1') && known.includes('r2') && out.uout == null) {
    const sum = out.r1 + out.r2;
    if (sum === 0) return null;
    out.uout = (out.uin * out.r2) / sum;
  }
  else {
    return null;
  }

  // Валидация результата
  if (out.r1 != null && out.r1 < 0) return null;
  if (out.r2 != null && out.r2 < 0) return null;
  if (out.uout != null && out.uout > out.uin) return null;

  // Дополнительные величины
  const sumR = (out.r1 ?? 0) + (out.r2 ?? 0);
  const current = sumR > 0 ? out.uin / sumR : null;
  const ratio = out.uin !== 0 ? out.uout / out.uin : null;

  return { ...out, current, ratio };
}

/**
 * Подбор резистора по мощности/току: известны U, I или R (любые 2).
 * Считаем остальные + мощность, которую надо рассеять.
 *
 * @param {{ u?, i?, r? }} given
 * @returns {{ u, i, r, p, recommendedPower } | null}
 */
export function solveResistorPower({ u, i, r }) {
  const given = { u, i, r };
  const known = Object.keys(given).filter(
    (k) => given[k] !== null && given[k] !== undefined && isFinite(given[k])
  );
  if (known.length !== 2) return null;

  const out = { u, i, r, p: null };

  if (known.includes('u') && known.includes('i') && out.r == null) {
    if (out.i === 0) return null;
    out.r = out.u / out.i;
    out.p = out.u * out.i;
  } else if (known.includes('u') && known.includes('r') && out.i == null) {
    if (out.r === 0) return null;
    out.i = out.u / out.r;
    out.p = out.u * out.i;
  } else if (known.includes('i') && known.includes('r') && out.u == null) {
    out.u = out.i * out.r;
    out.p = out.i * out.i * out.r;
  } else {
    return null;
  }

  // Рекомендуемая мощность резистора — стандартный ряд с запасом ×1.5
  const POWER_RATINGS = [0.125, 0.25, 0.5, 1, 2, 5, 10, 25];
  const needed = out.p * 1.5;
  const recommendedPower = POWER_RATINGS.find((p) => p >= needed) ?? null;

  return { ...out, recommendedPower };
}

/**
 * Ближайшие стандартные номиналы E24 к заданному значению.
 * Возвращает массив из нескольких ближайших (снизу и сверху).
 *
 * @param {number} value — целевое сопротивление в Омах
 * @param {number} count — сколько ближайших вернуть (по умолчанию 3)
 * @returns {number[]}
 */
export function nearestE24(value, count = 3) {
  if (!isFinite(value) || value <= 0) return [];

  const E24 = [10, 11, 12, 13, 15, 16, 18, 20, 22, 24, 27, 30, 33, 36, 39, 43, 47, 51, 56, 62, 68, 75, 82, 91];

  const decade = Math.floor(Math.log10(value));
  const candidates = [];

  for (let d = decade - 1; d <= decade + 1; d++) {
    const factor = Math.pow(10, d) / 10; // делим на 10, потому что база E24 начинается с 10
    for (const base of E24) {
      candidates.push(base * factor);
    }
  }

  candidates.sort((a, b) => Math.abs(a - value) - Math.abs(b - value));
  return candidates.slice(0, count);
}
/**
 * Реактивное сопротивление конденсатора (Xc).
 * Xc = 1 / (2π * f * C)
 * @param {number} f — частота, Гц
 * @param {number} c — ёмкость, Ф
 * @returns {number|null}
 */
export function capacitiveReactance(f, c) {
  if (!isFinite(f) || !isFinite(c) || f <= 0 || c <= 0) return null;
  return 1 / (2 * Math.PI * f * c);
}

/**
 * Суммарная ёмкость при последовательном соединении.
 * 1/C = 1/C1 + 1/C2 + ... → C = 1 / Σ(1/Ci)
 */
export function sumSeriesC(values) {
  if (!Array.isArray(values) || values.length === 0) return null;
  let invSum = 0;
  for (const v of values) {
    if (!isFinite(v) || v <= 0) return null;
    invSum += 1 / v;
  }
  if (invSum === 0) return null;
  return 1 / invSum;
}

/**
 * Суммарная ёмкость при параллельном соединении.
 * C = C1 + C2 + ...
 */
export function sumParallelC(values) {
  if (!Array.isArray(values) || values.length === 0) return null;
  let sum = 0;
  for (const v of values) {
    if (!isFinite(v) || v < 0) return null;
    sum += v;
  }
  return sum;
}

/**
 * Форматирование частоты.
 * 1000 → "1 кГц", 1000000 → "1 МГц"
 */
export function formatFrequency(hz) {
  if (hz == null || isNaN(hz)) return '—';
  const abs = Math.abs(hz);
  if (abs >= 1e6) return `${trimNum(hz / 1e6)} МГц`;
  if (abs >= 1e3) return `${trimNum(hz / 1e3)} кГц`;
  return `${trimNum(hz)} Гц`;
}
/**
 * Реактивное сопротивление катушки (Xl).
 * Xl = 2π * f * L
 */
export function inductiveReactance(f, l) {
  if (!isFinite(f) || !isFinite(l) || f <= 0 || l <= 0) return null;
  return 2 * Math.PI * f * l;
}

/**
 * Суммарная индуктивность при последовательном соединении.
 * L = L1 + L2 + ...
 */
export function sumSeriesL(values) {
  if (!Array.isArray(values) || values.length === 0) return null;
  let sum = 0;
  for (const v of values) {
    if (!isFinite(v) || v < 0) return null;
    sum += v;
  }
  return sum;
}

/**
 * Суммарная индуктивность при параллельном соединении.
 * 1/L = 1/L1 + 1/L2 + ...
 */
export function sumParallelL(values) {
  if (!Array.isArray(values) || values.length === 0) return null;
  let invSum = 0;
  for (const v of values) {
    if (!isFinite(v) || v <= 0) return null;
    invSum += 1 / v;
  }
  if (invSum === 0) return null;
  return 1 / invSum;
}

/**
 * Расчёт индуктивности однослойной катушки по формуле Уилера.
 * L (мкГн) = (r² * N²) / (9r + 10l)
 * где r — радиус (дюймы), l — длина намотки (дюймы), N — количество витков.
 * 
 * Входные данные в мм, N — количество витков.
 * Возвращает Генри.
 */
export function wheelerSingleLayer(diameterMm, lengthMm, turns) {
  if (!isFinite(diameterMm) || !isFinite(lengthMm) || !isFinite(turns)) return null;
  if (diameterMm <= 0 || lengthMm <= 0 || turns <= 0) return null;

  // Переводим мм в дюймы
  const radiusIn = (diameterMm / 2) / 25.4;
  const lengthIn = lengthMm / 25.4;

  // Формула Уилера для однослойной: L (мкГн) = (r² * N²) / (9r + 10l)
  const lUh = (radiusIn * radiusIn * turns * turns) / (9 * radiusIn + 10 * lengthIn);

  return lUh * 1e-6; // мкГн → Гн
}

/**
 * Расчёт индуктивности многослойной катушки по формуле Уилера.
 * L (мкГн) = (0.8 * r² * N²) / (6r + 9l + 10c)
 * где r — радиус, l — длина, c — толщина намотки (дюймы).
 * 
 * Входные данные в мм, N — количество витков, layers — количество слоёв.
 * Возвращает Генри.
 */
export function wheelerMultiLayer(diameterMm, lengthMm, layers, turns) {
  if (!isFinite(diameterMm) || !isFinite(lengthMm) || !isFinite(layers) || !isFinite(turns)) return null;
  if (diameterMm <= 0 || lengthMm <= 0 || layers <= 0 || turns <= 0) return null;

  // Переводим мм в дюймы
  const radiusIn = (diameterMm / 2) / 25.4;
  const lengthIn = lengthMm / 25.4;
  const thicknessIn = lengthIn / layers; // толщина намотки (приближение)

  const lUh = (0.8 * radiusIn * radiusIn * turns * turns) / 
              (6 * radiusIn + 9 * lengthIn + 10 * thicknessIn);

  return lUh * 1e-6;
}