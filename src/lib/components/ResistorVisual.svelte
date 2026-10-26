<script>
  import { RESISTOR_COLORS } from '../utils/resistorCodes.js';

  /**
   * ResistorVisual — объёмный SVG-резистор с цветными полосами.
   *
   * Пропсы:
   *  - bands: string[] — массив id цветов (например ['brown','black','red','gold'])
   *  - width: number — ширина в px (по умолчанию 320)
   *  - height: number — высота в px (по умолчанию 80)
   *  - gapAfterFirst: boolean — добавить визуальный пробел после первой полосы
   *                    (для 4+ полос: отделяем цифры от множителя/допуска)
   */

  let { bands = [], width = 320, height = 80, gapAfterFirst = true } = $props();

  // Получаем метаданные по цвету
  function colorById(id) {
    return RESISTOR_COLORS.find((c) => c.id === id);
  }
  function fill(id) {
    const c = colorById(id);
    return c ? c.hex : '#444';
  }
  function strokeFor(id) {
    // Для белых/жёлтых/серебряных/золотых добавляем обводку, чтобы были видны
    if (['white', 'yellow', 'silver', 'gold'].includes(id)) {
      return 'rgba(0, 0, 0, 0.35)';
    }
    return 'rgba(0, 0, 0, 0.2)';
  }

  // Геометрия
  const VB_W = 320;
  const VB_H = 80;

  const bodyX = 60;             // начало корпуса
  const bodyW = 200;            // ширина корпуса
  const bodyY = 14;             // верх корпуса
  const bodyH = 52;             // высота корпуса
  const bodyRx = 26;            // радиус скругления корпуса

  const leadY = bodyY + bodyH / 2;  // ось выводов
  const leadLen = 60;

  // Полосы
  const bandWidth = 10;
  const bandHeight = bodyH - 4;
  const bandY = bodyY + 2;

  // Позиции полос внутри корпуса (в % от bodyW)
  // Раскладка: если 3 полосы — [20, 40, 60]%
  //           если 4 — [15, 33, 52, 72]%
  //           если 5 — [12, 28, 44, 60, 76]%
  //           если 6 — [10, 24, 38, 52, 66, 80]%
  function bandPositions(n) {
    const layouts = {
      3: [0.22, 0.42, 0.62],
      4: [0.16, 0.34, 0.54, 0.74],
      5: [0.12, 0.28, 0.44, 0.60, 0.76],
      6: [0.10, 0.24, 0.38, 0.52, 0.66, 0.80]
    };
    return (layouts[n] ?? layouts[4]).map((p) => bodyX + bodyW * p - bandWidth / 2);
  }

  const positions = $derived(bandPositions(bands.length));

  // Признак наличия пробела после первой полосы (для 4-6 полос)
  const hasGap = $derived(gapAfterFirst && bands.length >= 4);
</script>

<div class="resistor-visual" style={`max-width: ${width}px;`}>
  <svg
    viewBox={`0 0 ${VB_W} ${VB_H}`}
    preserveAspectRatio="xMidYMid meet"
    role="img"
    aria-label="Цветовая маркировка резистора"
  >
    <defs>
      <!-- Градиент корпуса -->
      <linearGradient id="bodyGrad" x1="0" y1="0" x2="0" y2="1">
        <stop offset="0%" stop-color="#e9e4d2" />
        <stop offset="35%" stop-color="#d8d1bb" />
        <stop offset="60%" stop-color="#c0b89e" />
        <stop offset="100%" stop-color="#a39b81" />
      </linearGradient>

      <!-- Тень корпуса -->
      <radialGradient id="bodyShadow" cx="0.5" cy="1.1" r="0.8">
        <stop offset="0%" stop-color="rgba(0,0,0,0.35)" />
        <stop offset="100%" stop-color="rgba(0,0,0,0)" />
      </radialGradient>

      <!-- Тень для полос (лёгкая по краям) -->
      <linearGradient id="bandEdge" x1="0" y1="0" x2="1" y2="0">
        <stop offset="0%" stop-color="rgba(0,0,0,0.35)" />
        <stop offset="15%" stop-color="rgba(0,0,0,0)" />
        <stop offset="85%" stop-color="rgba(0,0,0,0)" />
        <stop offset="100%" stop-color="rgba(0,0,0,0.35)" />
      </linearGradient>

      <!-- Общий фильтр с тенью -->
      <filter id="softShadow" x="-10%" y="-10%" width="120%" height="130%">
        <feDropShadow dx="0" dy="3" stdDeviation="3" flood-color="#000" flood-opacity="0.45" />
      </filter>

      <!-- Внутренний блик на корпусе -->
      <linearGradient id="bodyHighlight" x1="0" y1="0" x2="0" y2="1">
        <stop offset="0%" stop-color="rgba(255,255,255,0.35)" />
        <stop offset="25%" stop-color="rgba(255,255,255,0.05)" />
        <stop offset="100%" stop-color="rgba(255,255,255,0)" />
      </linearGradient>
    </defs>

    <!-- Тень под резистором -->
    <ellipse
      cx={VB_W / 2}
      cy={bodyY + bodyH + 6}
      rx={bodyW / 2 + 20}
      ry="6"
      fill="url(#bodyShadow)"
    />

    <!-- Вывод слева -->
    <rect
      x={bodyX - leadLen + 4}
      y={leadY - 2}
      width={leadLen}
      height="4"
      fill="#a8a8a8"
      rx="2"
    />
    <!-- Вывод справа -->
    <rect
      x={bodyX + bodyW}
      y={leadY - 2}
      width={leadLen - 4}
      height="4"
      fill="#a8a8a8"
      rx="2"
    />

    <!-- Корпус -->
    <g filter="url(#softShadow)">
      <rect
        x={bodyX}
        y={bodyY}
        width={bodyW}
        height={bodyH}
        rx={bodyRx}
        fill="url(#bodyGrad)"
        stroke="#7d7560"
        stroke-width="0.8"
      />
      <!-- Внутренний блик -->
      <rect
        x={bodyX + 2}
        y={bodyY + 2}
        width={bodyW - 4}
        height={bodyH / 2}
        rx={bodyRx - 2}
        fill="url(#bodyHighlight)"
        opacity="0.7"
      />
    </g>

    <!-- Полосы -->
    {#each bands as colorId, i (i)}
      {@const x = positions[i]}
      {@const gapOffset = hasGap && i > 0 ? 6 : 0}
      <g>
        <!-- Тень полосы по краям (для объёма) -->
        <rect
          x={x + gapOffset}
          y={bandY}
          width={bandWidth}
          height={bandHeight}
          fill="url(#bandEdge)"
          opacity="0.9"
          rx="2"
        />
        <!-- Сама полоса -->
        <rect
          x={x + gapOffset}
          y={bandY}
          width={bandWidth}
          height={bandHeight}
          fill={fill(colorId)}
          stroke={strokeFor(colorId)}
          stroke-width="0.6"
          rx="2"
        />
      </g>
    {/each}

    <!-- Лёгкий блик поверх полос (объём) -->
    <rect
      x={bodyX}
      y={bodyY}
      width={bodyW}
      height="6"
      rx={bodyRx}
      fill="rgba(255,255,255,0.12)"
    />
  </svg>
</div>

<style>
  .resistor-visual {
    width: 100%;
    margin: 0 auto;
    display: flex;
    justify-content: center;
  }

  .resistor-visual svg {
    width: 100%;
    height: auto;
    display: block;
    overflow: visible;
  }
</style>