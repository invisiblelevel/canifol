/**
 * База популярных MOSFET для BMS.
 * Включает мощные (TO-220, D2PAK) и мелкие (SOT-23, DFN) модели.
 */
export const MOSFETS = [
  // --- Мощные, классика (TO-220 / D2PAK) ---
  { id: 'irf3205', name: 'IRF3205', vds: 55, idCont: 110, rdsOn: 8 },
  { id: 'irfz44n', name: 'IRFZ44N', vds: 55, idCont: 49, rdsOn: 17.5 },
  { id: 'irf1404', name: 'IRF1404', vds: 40, idCont: 202, rdsOn: 4 },
  { id: 'irf1405', name: 'IRF1405', vds: 55, idCont: 169, rdsOn: 5.3 },
  { id: 'irfb4110', name: 'IRFB4110', vds: 100, idCont: 180, rdsOn: 4.5 },
  { id: 'irlr7843', name: 'IRLR7843', vds: 30, idCont: 161, rdsOn: 3.3 },

  // --- Мощные монстры (большие токи) ---
  { id: 'irlb3034', name: 'IRLB3034', vds: 40, idCont: 343, rdsOn: 1.7 },
  { id: 'irfb7530', name: 'IRFB7530', vds: 60, idCont: 295, rdsOn: 2 },
  { id: 'irf1324', name: 'IRF1324', vds: 24, idCont: 353, rdsOn: 1.5 },
  { id: 'irlb3036', name: 'IRLB3036', vds: 60, idCont: 270, rdsOn: 2.4 },
  { id: 'irl3803', name: 'IRL3803', vds: 30, idCont: 140, rdsOn: 5.5 },

  // --- Мелкие SOT-23 (для слаботочных BMS и защиты ячеек) ---
  { id: 'ao3400', name: 'AO3400', vds: 30, idCont: 5.8, rdsOn: 28 },
  { id: 'si2302', name: 'SI2302', vds: 20, idCont: 2.9, rdsOn: 75 }, // SOT-23, очень распространён [citation:4]
  { id: 'irlml6244', name: 'IRLML6244', vds: 20, idCont: 6.3, rdsOn: 27 }, // SOT-23, низкое Rds [citation:5]
  { id: '2n7002', name: '2N7002', vds: 60, idCont: 0.3, rdsOn: 3500 }, // SOT-23, сигнальный, часто в ремонте [citation:4][citation:14]
  { id: 'bss138', name: 'BSS138', vds: 50, idCont: 0.2, rdsOn: 3000 }, // SOT-23, сигнальный [citation:4]
  { id: 'ao3402', name: 'AO3402', vds: 30, idCont: 4, rdsOn: 52 }, // SOT-23 [citation:4]
  { id: 'si2302dds', name: 'SI2302DDS', vds: 20, idCont: 2.9, rdsOn: 75 }, // SOT-23 [citation:4]
];