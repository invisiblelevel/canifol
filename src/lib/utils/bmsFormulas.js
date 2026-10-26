/**
 * Fixitron — формулы для BMS.
 * Основано на документации Texas Instruments (BQ79616).
 */

/**
 * Расчёт балансировочного резистора (Passive Balancing).
 * Формула из TI: R_CB = 1/2 * (V_CELL / I_CB - R_DS(on))
 * 
 * @param {number} vCell — напряжение банки (В)
 * @param {number} iCbMa — желаемый ток балансировки (мА)
 * @param {number} rDsOn — сопротивление ключа BMS (Ом)
 * @returns {{ rCb: number, powerW: number, iCbWorst: number } | null}
 */
export function calcBalancingResistor(vCell, iCbMa, rDsOn = 0) {
  if (!isFinite(vCell) || !isFinite(iCbMa) || vCell <= 0 || iCbMa <= 0) return null;
  if (!isFinite(rDsOn) || rDsOn < 0) return null;

  const iCb = iCbMa / 1000; // мА → А

  // R_CB = 0.5 * (V_CELL / I_CB - R_DS(on))
  const rCb = 0.5 * (vCell / iCb - rDsOn);
  if (rCb <= 0) return null;

  // Худший случай: R_DS(on) = 0 (или минимальное), ток будет максимальным
  // I_worst = V_CELL / (2 * R_CB + R_DS_on_min)
  // Для простоты считаем R_DS_on_min = 0, тогда I_worst = V_CELL / (2 * R_CB)
  const iCbWorst = vCell / (2 * rCb);

  // Мощность на резисторе в худшем случае: P = V_CELL * I_CB (на один резистор)
  // Но у нас два резистора, поэтому на каждом P = V_CELL * I_CB / 2? 
  // По даташиту TI: R_CB Power rating = V_BAT * I_CB (для одного резистора, с запасом)
  // Будем считать с запасом ×2 для надёжности.
  const powerW = vCell * iCbWorst;

  return { rCb, powerW, iCbWorst };
}
/**
 * Расчёт конфигурации батареи (S/P).
 * @param {number} s — количество последовательных
 * @param {number} p — количество параллельных
 * @param {number} vCell — напряжение одной банки (В)
 * @param {number} capCell — ёмкость одной банки (мА·ч)
 * @param {number} cRate — C-rate (например, 1 = 1C)
 * @returns {{ voltage: number, capacityAh: number, energyWh: number, maxCurrent: number } | null}
 */
export function calcBatteryConfig(s, p, vCell, capCell, cRate = 1) {
  if (!isFinite(s) || !isFinite(p) || !isFinite(vCell) || !isFinite(capCell) || s <= 0 || p <= 0 || vCell <= 0 || capCell <= 0) return null;
  if (!isFinite(cRate) || cRate <= 0) return null;

  const voltage = s * vCell;
  const capacityAh = (p * capCell) / 1000;
  const energyWh = voltage * capacityAh;
  const maxCurrent = capacityAh * cRate;

  return { voltage, capacityAh, energyWh, maxCurrent };
}
/**
 * Расчёт шунта для измерения тока.
 * @param {number} iMax — максимальный ток (А)
 * @param {number} iMin — минимальный ток (А)
 * @param {number} vOsUv — напряжение смещения усилителя (мкВ)
 * @returns {{ rShunt: number, powerW: number, vSenseMaxMv: number, minDetectableMa: number, isSensitive: boolean } | null}
 */
export function calcCurrentShunt(iMax, iMin, vOsUv = 100) {
  if (!isFinite(iMax) || !isFinite(iMin) || iMax <= 0 || iMin <= 0) return null;
  if (!isFinite(vOsUv) || vOsUv <= 0) return null;

  // Максимальное падение напряжения на шунте (типично 100 мВ = 0.1 В)
  const vSenseMax = 0.1; // 100 мВ
  
  // Рекомендуемый номинал шунта (Ом)
  let rShunt = vSenseMax / iMax;
  
  // Если шунт слишком маленький, чтобы различить iMin при данном Vos
  const vOs = vOsUv * 1e-6; // мкВ → В
  const minDetectable = vOs / rShunt;
  
  // Мощность на шунте
  const powerW = iMax * iMax * rShunt;
  
  // Падение напряжения при максимальном токе (мВ)
  const vSenseMaxMv = iMax * rShunt * 1000;
  
  // Минимальный detectable ток (мА)
  const minDetectableMa = minDetectable * 1000;
  
  // Проверка: хватит ли чувствительности для iMin
  const isSensitive = minDetectable <= iMin;
  
  return {
    rShunt: rShunt * 1000, // Ом → мОм
    powerW,
    vSenseMaxMv,
    minDetectableMa,
    isSensitive
  };
}
/**
 * Расчёт количества MOSFET для защиты BMS.
 * @param {number} iMax — максимальный ток (А)
 * @param {number} rDsOnMohm — R_ds(on) одного MOSFET (мОм)
 * @param {number} rthJa — тепловое сопротивление (C/Вт)
 * @param {number} tAmbient — температура среды (C)
 * @param {number} tJmax — макс. температура кристалла (C)
 * @returns {{ singleLoss: number, singleTj: number, recommendedCount: number, lossPerFet: number, tjPerFet: number } | null}
 */
export function calcMosfetProtection(iMax, rDsOnMohm, rthJa, tAmbient, tJmax) {
  if (!isFinite(iMax) || !isFinite(rDsOnMohm) || !isFinite(rthJa) || !isFinite(tAmbient) || !isFinite(tJmax)) return null;
  if (iMax <= 0 || rDsOnMohm <= 0 || rthJa <= 0) return null;
  if (tJmax <= tAmbient) return null;

  // R_ds(on) в Омах
  const rDsOn = rDsOnMohm / 1000;

  // Потери на одном MOSFET
  const singleLoss = iMax * iMax * rDsOn;

  // Температура кристалла одного MOSFET
  const singleTj = tAmbient + singleLoss * rthJa;

  // Если одного хватает
  if (singleTj <= tJmax) {
    return {
      singleLoss,
      singleTj,
      recommendedCount: 1,
      lossPerFet: singleLoss,
      tjPerFet: singleTj
    };
  }

  // Ищем минимальное N, при котором Tj <= tJmax
  // При N параллельно: P_loss на один = (I_max / N)^2 * R_ds(on) = I_max^2 * R_ds(on) / N^2
  // T_j = T_a + P_loss_per_fet * R_th(j-a)
  // T_j = T_a + (I_max^2 * R_ds(on) / N^2) * R_th(j-a)
  // Решаем относительно N:
  // N^2 >= (I_max^2 * R_ds(on) * R_th(j-a)) / (T_j_max - T_a)
  
  const numerator = iMax * iMax * rDsOn * rthJa;
  const denominator = tJmax - tAmbient;
  const nSquared = numerator / denominator;
  const recommendedCount = Math.ceil(Math.sqrt(nSquared));

  // Пересчитываем для этого N
  const lossPerFet = (iMax * iMax * rDsOn) / (recommendedCount * recommendedCount);
  const tjPerFet = tAmbient + lossPerFet * rthJa;

  return {
    singleLoss,
    singleTj,
    recommendedCount: Math.max(1, recommendedCount),
    lossPerFet,
    tjPerFet
  };
}