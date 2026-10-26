<script>
  import { INDUCTOR_COLORS } from '../utils/inductorCodes.js';

  let { bands = [], width = 160, height = 360 } = $props();

  function colorById(id) {
    return INDUCTOR_COLORS.find((c) => c.id === id);
  }
  function fill(id) {
    const c = colorById(id);
    return c ? c.hex : '#444';
  }
  function strokeFor(id) {
    if (['white', 'yellow', 'silver', 'gold'].includes(id)) {
      return 'rgba(0, 0, 0, 0.45)';
    }
    return 'rgba(0, 0, 0, 0.3)';
  }

  // Геометрия (вертикальный толстый бочонок)
  const VB_W = 160;
  const VB_H = 360;

  // Корпус (бочонок)
  const bodyW = 100; // Толщина
  const bodyH = 200; // Высота
  const bodyX = (VB_W - bodyW) / 2; // 30
  const bodyY = (VB_H - bodyH) / 2; // 80
  const bodyRx = 16; // Сильное скругление для объёма

  // Выводы (толстые)
  const leadLen = 50;
  const leadW = 8;
  const leadX = VB_W / 2 - leadW / 2;

  // Полосы (горизонтальные)
  const bandHeight = 20;
  const bandWidth = bodyW - 8; // Чуть уже корпуса
  const bandX = bodyX + 4;

  function bandPositions(n) {
    const layouts = {
      3: [0.35, 0.50, 0.65],
      4: [0.25, 0.42, 0.58, 0.75]
    };
    return (layouts[n] ?? layouts[3]).map((p) => bodyY + bodyH * p - bandHeight / 2);
  }

  const positions = $derived(bandPositions(bands.length));

  // Витки (плотная намотка)
  const coils = $derived.by(() => {
    const list = [];
    const count = 18; // Много витков
    for (let i = 0; i < count; i++) {
      const y = bodyY + 10 + ((bodyH - 20) / count) * i;
      list.push(y);
    }
    return list;
  });
</script>

<div class="inductor-visual" style={`height: ${height}px; width: ${width}px;`}>
  <svg
    viewBox={`0 0 ${VB_W} ${VB_H}`}
    preserveAspectRatio="xMidYMid meet"
    role="img"
    aria-label="Цветовая маркировка катушки"
  >
    <defs>
      <!-- Градиент для объёмного бочонка -->
      <linearGradient id="indBody" x1="0" y1="0" x2="1" y2="0">
        <stop offset="0%" stop-color="#1a1a1a" />
        <stop offset="15%" stop-color="#333" />
        <stop offset="40%" stop-color="#666" />
        <stop offset="55%" stop-color="#777" />
        <stop offset="85%" stop-color="#333" />
        <stop offset="100%" stop-color="#111" />
      </linearGradient>

      <!-- Градиент для щечек (торцы) -->
      <linearGradient id="indCap" x1="0" y1="0" x2="1" y2="0">
        <stop offset="0%" stop-color="#111" />
        <stop offset="50%" stop-color="#333" />
        <stop offset="100%" stop-color="#111" />
      </linearGradient>

      <!-- Тень под катушкой -->
      <radialGradient id="indShadow" cx="0.5" cy="0.5" r="0.5">
        <stop offset="0%" stop-color="rgba(0,0,0,0.6)" />
        <stop offset="100%" stop-color="rgba(0,0,0,0)" />
      </radialGradient>

      <filter id="indShadowFilter" x="-20%" y="-10%" width="140%" height="120%">
        <feDropShadow dx="0" dy="10" stdDeviation="8" flood-color="#000" flood-opacity="0.7" />
      </filter>
      
      <!-- Тень для полос -->
      <linearGradient id="indBandEdge" x1="0" y1="0" x2="0" y2="1">
        <stop offset="0%" stop-color="rgba(0,0,0,0.5)" />
        <stop offset="20%" stop-color="rgba(0,0,0,0)" />
        <stop offset="80%" stop-color="rgba(0,0,0,0)" />
        <stop offset="100%" stop-color="rgba(0,0,0,0.5)" />
      </linearGradient>
    </defs>

    <!-- Тень на полу (эллипс) -->
    <ellipse
      cx={VB_W / 2}
      cy={bodyY + bodyH + 35}
      rx={bodyW / 2 + 10}
      ry="8"
      fill="url(#indShadow)"
    />

    <!-- Вывод сверху -->
    <rect
      x={leadX}
      y={bodyY - leadLen + 10}
      width={leadW}
      height={leadLen}
      fill="#888"
      rx="2"
    />
    <!-- Вывод снизу -->
    <rect
      x={leadX}
      y={bodyY + bodyH - 10}
      width={leadW}
      height={leadLen}
      fill="#888"
      rx="2"
    />

    <!-- Основной корпус -->
    <g filter="url(#indShadowFilter)">
      <!-- Задняя часть бочонка (для объёма) -->
      <rect
        x={bodyX}
        y={bodyY}
        width={bodyW}
        height={bodyH}
        rx={bodyRx}
        fill="url(#indBody)"
        stroke="#111"
        stroke-width="1.5"
      />

      <!-- Намотка (витки) -->
      {#each coils as y}
        <path
          d={`M ${bodyX + 4} ${y} Q ${VB_W / 2} ${y + 6} ${bodyX + bodyW - 4} ${y}`}
          fill="none"
          stroke="rgba(255,255,255,0.15)"
          stroke-width="2"
        />
        <path
          d={`M ${bodyX + 4} ${y + 2} Q ${VB_W / 2} ${y + 8} ${bodyX + bodyW - 4} ${y + 2}`}
          fill="none"
          stroke="rgba(0,0,0,0.4)"
          stroke-width="1.5"
        />
      {/each}

      <!-- Верхняя щечка (торцевая часть) -->
      <rect
        x={bodyX - 4}
        y={bodyY - 8}
        width={bodyW + 8}
        height="16"
        rx="6"
        fill="url(#indCap)"
        stroke="#111"
        stroke-width="1"
      />
      <!-- Нижняя щечка -->
      <rect
        x={bodyX - 4}
        y={bodyY + bodyH - 8}
        width={bodyW + 8}
        height="16"
        rx="6"
        fill="url(#indCap)"
        stroke="#111"
        stroke-width="1"
      />
    </g>

    <!-- Цветные полосы -->
    {#each bands as colorId, i (i)}
      {@const y = positions[i]}
      <g>
        <rect
          x={bandX}
          y={y}
          width={bandWidth}
          height={bandHeight}
          fill="url(#indBandEdge)"
          opacity="0.95"
          rx="3"
        />
        <rect
          x={bandX}
          y={y}
          width={bandWidth}
          height={bandHeight}
          fill={fill(colorId)}
          stroke={strokeFor(colorId)}
          stroke-width="1"
          rx="3"
        />
      </g>
    {/each}

    <!-- Блик сверху для глянца -->
    <rect
      x={bodyX + 10}
      y={bodyY + 4}
      width={bodyW - 20}
      height="6"
      rx="3"
      fill="rgba(255,255,255,0.2)"
    />
  </svg>
</div>

<style>
  .inductor-visual {
    margin: 0 auto;
    display: flex;
    justify-content: center;
    align-items: center;
  }

  .inductor-visual svg {
    width: 100%;
    height: 100%;
    display: block;
    overflow: visible;
  }
</style>