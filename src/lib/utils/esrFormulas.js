/**
 * ESR-таблицы для электролитических конденсаторов.
 * Логика: поиск ближайших строк в таблице + логарифмическая интерполяция.
 */

import ESR_TABLE from '../data/esrTable.json';

/**
 * Возвращает список типов конденсаторов для селектора.
 */
export function getEsrTypes() {
  return ['general', 'lowEsr', 'audio', 'highTemp'];
}

/**
 * Маппинг типа конденсатора → суффикс ключа локализации.
 * Используется для отрисовки заголовка результата.
 */
export const ESR_TYPE_KEYS = {
  general:  'esrTypeGeneral',
  lowEsr:   'esrTypeLowEsr',
  audio:    'esrTypeAudio',
  highTemp: 'esrTypeHighTemp'
};

/**
 * Ищет ESR-диапазон для заданной ёмкости (мкФ) и напряжения (В).
 * 
 * @param {number} capacitance - мкФ
 * @param {number} voltage - В
 * @param {string} type - general | lowEsr | audio | highTemp
 * @returns {{min:number, max:number, bad:number, rowC:number, rowV:number, interpolated:boolean} | null}
 */
export function findEsr(capacitance, voltage, type = 'general') {
  if (!capacitance || capacitance <= 0 || !voltage || voltage <= 0) return null;
  const table = ESR_TABLE[type];
  if (!Array.isArray(table) || table.length === 0) return null;

  // Сортируем по ёмкости (на случай если в JSON порядок не гарантирован)
  const sorted = [...table].sort((a, b) => a.c - b.c);

  // 1. Находим два соседних значения ёмкости, между которыми лежит capacitance
  let lower = null;
  let upper = null;
  for (let i = 0; i < sorted.length; i++) {
    if (sorted[i].c <= capacitance) lower = sorted[i];
    if (sorted[i].c >= capacitance && !upper) upper = sorted[i];
  }

  // Крайние случаи — берём ближайшее
  if (!lower) lower = sorted[0];
  if (!upper) upper = sorted[sorted.length - 1];

  // 2. Интерполируем по логарифму ёмкости
  let min, max, bad, rowC, rowV, interpolated = false;
  if (lower === upper) {
    min = lower.min; max = lower.max; bad = lower.bad;
    rowC = lower.c; rowV = lower.v;
  } else {
    const t = (Math.log(capacitance) - Math.log(lower.c)) /
              (Math.log(upper.c) - Math.log(lower.c));
    min = lower.min + (upper.min - lower.min) * t;
    max = lower.max + (upper.max - lower.max) * t;
    bad = lower.bad + (upper.bad - lower.bad) * t;
    rowC = capacitance;
    rowV = voltage;
    interpolated = true;
  }

  // 3. Корректируем по напряжению (при большем V — ESR немного выше)
  //    Простая поправка: если напряжение отличается от опорного (25V) 
  //    в N раз, ESR умножается на sqrt(N) — это грубо, но для ремонта ок.
  const refV = rowV || 25;
  const vFactor = Math.sqrt(Math.max(voltage, 1) / refV);
  if (Math.abs(vFactor - 1) > 0.15 && !interpolated) {
    // применяем только если не интерполировали (там уже сглажено)
    min *= vFactor;
    max *= vFactor;
    bad *= vFactor;
  }

  return { min, max, bad, rowC, rowV, interpolated };
}

/**
 * Оценка результата измерения.
 * 
 * @param {number} measured - Ом
 * @param {{min:number, max:number, bad:number}} range
 * @returns {{level: 'low'|'ok'|'high'|'bad'|'unknown', key: string}}
 */
export function judgeEsr(measured, range) {
  if (measured == null || isNaN(measured) || !range) {
    return { level: 'unknown', key: 'esrJudgeUnknown' };
  }
  if (measured < range.min) return { level: 'low', key: 'esrJudgeLow' };
  if (measured <= range.max) return { level: 'ok', key: 'esrJudgeOk' };
  if (measured <= range.bad) return { level: 'high', key: 'esrJudgeHigh' };
  return { level: 'bad', key: 'esrJudgeBad' };
}

/**
 * Формат ESR в Ом/мОм с учётом языка.
 * 
 * @param {number} ohm 
 * @param {object} units - strings.units (содержит mohm_short и ohm)
 * @returns {string}
 */
export function formatEsr(ohm, units) {
  if (ohm == null || isNaN(ohm)) return '—';
  const mohm = units?.mohm_short ?? 'mΩ';
  const om = units?.ohm ?? 'Ω';
  if (ohm < 1) return `${Math.round(ohm * 1000)} ${mohm}`;
  if (ohm < 10) return `${ohm.toFixed(2)} ${om}`;
  return `${ohm.toFixed(1)} ${om}`;
}