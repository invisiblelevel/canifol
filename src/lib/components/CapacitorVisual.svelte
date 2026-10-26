<script>
  import { CAPACITOR_COLORS } from '../utils/capacitorCodes.js';

  let { bands = [], width = 320, height = 100 } = $props();

  function colorById(id) {
    return CAPACITOR_COLORS.find((c) => c.id === id);
  }
  function fill(id) {
    const c = colorById(id);
    return c ? c.hex : '#444';
  }
  function strokeFor(id) {
    if (['white', 'yellow', 'silver', 'gold'].includes(id)) {
      return 'rgba(0, 0, 0, 0.35)';
    }
    return 'rgba(0, 0, 0, 0.2)';
  }

  // Геометрия (вид "подушка" — плёночный конденсатор)
  const VB_W = 320;
  const VB_H = 100;

  const bodyX = 50;
  const bodyW = 220;
  const bodyY = 18;
  const bodyH = 64;
  const bodyRx = 12;

  const leadY = bodyY + bodyH / 2;
  const leadLen = 50;

  const bandWidth = 12;
  const bandHeight = bodyH - 6;
  const bandY = bodyY + 3;

  function bandPositions(n) {
    // Полосы идут вертикально по центру корпуса
    const layouts = {
      3: [0.30, 0.50, 0.70],
      4: [0.22, 0.40, 0.60, 0.78],
      5: [0.16, 0.31, 0.48, 0.65, 0.81]
    };
    return (layouts[n] ?? layouts[3]).map((p) => bodyX + bodyW * p - bandWidth / 2);
  }

  const positions = $derived(bandPositions(bands.length));
</script>

<div class="cap-visual" style={`max-width: ${width}px;`}>
  <svg
    viewBox={`0 0 ${VB_W} ${VB_H}`}
    preserveAspectRatio="xMidYMid meet"
    role="img"
    aria-label="Цветовая маркировка конденсатора"
  >
    <defs>
      <linearGradient id="capBody" x1="0" y1="0" x2="0" y2="1">
        <stop offset="0%" stop-color="#e8dfc8" />
        <stop offset="40%" stop-color="#d4c8a8" />
        <stop offset="70%" stop-color="#b8aa88" />
        <stop offset="100%" stop-color="#968a6b" />
      </linearGradient>

      <linearGradient id="capHighlight" x1="0" y1="0" x2="0" y2="1">
        <stop offset="0%" stop-color="rgba(255,255,255,0.45)" />
        <stop offset="30%" stop-color="rgba(255,255,255,0.05)" />
        <stop offset="100%" stop-color="rgba(255,255,255,0)" />
      </linearGradient>

      <radialGradient id="capShadow" cx="0.5" cy="1.1" r="0.8">
        <stop offset="0%" stop-color="rgba(0,0,0,0.35)" />
        <stop offset="100%" stop-color="rgba(0,0,0,0)" />
      </radialGradient>

      <linearGradient id="capBandEdge" x1="0" y1="0" x2="1" y2="0">
        <stop offset="0%" stop-color="rgba(0,0,0,0.35)" />
        <stop offset="15%" stop-color="rgba(0,0,0,0)" />
        <stop offset="85%" stop-color="rgba(0,0,0,0)" />
        <stop offset="100%" stop-color="rgba(0,0,0,0.35)" />
      </linearGradient>

      <filter id="capShadowFilter" x="-10%" y="-10%" width="120%" height="130%">
        <feDropShadow dx="0" dy="3" stdDeviation="3" flood-color="#000" flood-opacity="0.45" />
      </filter>
    </defs>

    <ellipse
      cx={VB_W / 2}
      cy={bodyY + bodyH + 6}
      rx={bodyW / 2 + 15}
      ry="5"
      fill="url(#capShadow)"
    />

    <rect x={bodyX - leadLen + 4} y={leadY - 2} width={leadLen} height="4" fill="#a8a8a8" rx="2" />
    <rect x={bodyX + bodyW} y={leadY - 2} width={leadLen - 4} height="4" fill="#a8a8a8" rx="2" />

    <g filter="url(#capShadowFilter)">
      <rect
        x={bodyX}
        y={bodyY}
        width={bodyW}
        height={bodyH}
        rx={bodyRx}
        fill="url(#capBody)"
        stroke="#7a6f56"
        stroke-width="0.8"
      />
      <rect
        x={bodyX + 2}
        y={bodyY + 2}
        width={bodyW - 4}
        height={bodyH / 2}
        rx={bodyRx - 2}
        fill="url(#capHighlight)"
        opacity="0.7"
      />
    </g>

    {#each bands as colorId, i (i)}
      {@const x = positions[i]}
      <g>
        <rect
          x={x}
          y={bandY}
          width={bandWidth}
          height={bandHeight}
          fill="url(#capBandEdge)"
          opacity="0.9"
          rx="2"
        />
        <rect
          x={x}
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
  .cap-visual {
    width: 100%;
    margin: 0 auto;
    display: flex;
    justify-content: center;
  }

  .cap-visual svg {
    width: 100%;
    height: auto;
    display: block;
    overflow: visible;
  }
</style>