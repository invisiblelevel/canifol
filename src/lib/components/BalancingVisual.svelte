<script>
  let { rCb = 0, vCell = 4.2, iCb = 0 } = $props();

  // Геометрия SVG
  const VB_W = 500;
  const VB_H = 220;

  const cellX = 60;
  const cellY = 70;
  const cellW = 80;
  const cellH = 80;

  const res1X = 200;
  const res2X = 340;
  const resY = 100;
  const resW = 60;
  const resH = 20;

  const fetX = 280;
  const fetY = 90;
</script>

<div class="balancing-visual">
  <svg viewBox={`0 0 ${VB_W} ${VB_H}`} preserveAspectRatio="xMidYMid meet">
    <defs>
      <linearGradient id="cellGrad" x1="0" y1="0" x2="0" y2="1">
        <stop offset="0%" stop-color="#3a3a3a" />
        <stop offset="100%" stop-color="#1a1a1a" />
      </linearGradient>
      <linearGradient id="resGrad" x1="0" y1="0" x2="0" y2="1">
        <stop offset="0%" stop-color="#f5b83d" />
        <stop offset="100%" stop-color="#e6a420" />
      </linearGradient>
      <filter id="glow">
        <feGaussianBlur stdDeviation="2" result="blur" />
        <feMerge>
          <feMergeNode in="blur" />
          <feMergeNode in="SourceGraphic" />
        </feMerge>
      </filter>
    </defs>

    <!-- Банка аккумулятора -->
    <rect
      x={cellX}
      y={cellY}
      width={cellW}
      height={cellH}
      rx="8"
      fill="url(#cellGrad)"
      stroke="var(--accent)"
      stroke-width="1.5"
    />
    <text x={cellX + cellW/2} y={cellY - 10} text-anchor="middle" fill="var(--text-secondary)" font-size="12">
      {vCell.toFixed(2)} В
    </text>
    <text x={cellX + cellW/2} y={cellY + cellH/2 + 4} text-anchor="middle" fill="var(--accent)" font-size="14" font-weight="bold">
      V_CELL
    </text>

    <!-- Провод к первому резистору -->
    <line x1={cellX + cellW} y1={cellY + cellH/2} x2={res1X} y2={cellY + cellH/2} stroke="var(--text-muted)" stroke-width="1.5" />

    <!-- Первый резистор R_CB -->
    <rect
      x={res1X}
      y={resY}
      width={resW}
      height={resH}
      rx="4"
      fill="url(#resGrad)"
      stroke="#b8860b"
      stroke-width="1"
    />
    <text x={res1X + resW/2} y={resY - 8} text-anchor="middle" fill="var(--text-secondary)" font-size="11">
      R_CB
    </text>
    <text x={res1X + resW/2} y={resY + resH + 16} text-anchor="middle" fill="var(--accent)" font-size="12" font-weight="bold">
      {rCb.toFixed(1)} Ом
    </text>

    <!-- Провод к ключу -->
    <line x1={res1X + resW} y1={cellY + cellH/2} x2={fetX} y2={cellY + cellH/2} stroke="var(--text-muted)" stroke-width="1.5" />

    <!-- MOSFET ключ -->
    <rect
      x={fetX}
      y={fetY}
      width="40"
      height="40"
      rx="4"
      fill="var(--bg-elevated-2)"
      stroke="var(--text-muted)"
      stroke-width="1.5"
    />
    <text x={fetX + 20} y={fetY + 24} text-anchor="middle" fill="var(--text-secondary)" font-size="10">
      FET
    </text>

    <!-- Провод ко второму резистору -->
    <line x1={fetX + 40} y1={cellY + cellH/2} x2={res2X} y2={cellY + cellH/2} stroke="var(--text-muted)" stroke-width="1.5" />

    <!-- Второй резистор R_CB -->
    <rect
      x={res2X}
      y={resY}
      width={resW}
      height={resH}
      rx="4"
      fill="url(#resGrad)"
      stroke="#b8860b"
      stroke-width="1"
    />
    <text x={res2X + resW/2} y={resY - 8} text-anchor="middle" fill="var(--text-secondary)" font-size="11">
      R_CB
    </text>

    <!-- Провод обратно к банке (земля) -->
    <line x1={res2X + resW} y1={cellY + cellH/2} x2={VB_W - 40} y2={cellY + cellH/2} stroke="var(--text-muted)" stroke-width="1.5" />
    <line x1={VB_W - 40} y1={cellY + cellH/2} x2={VB_W - 40} y2={cellY + cellH + 30} stroke="var(--text-muted)" stroke-width="1.5" />
    <line x1={cellX + cellW/2} y1={cellY + cellH} x2={cellX + cellW/2} y2={cellY + cellH + 30} stroke="var(--text-muted)" stroke-width="1.5" />
    <line x1={cellX + cellW/2} y1={cellY + cellH + 30} x2={VB_W - 40} y2={cellY + cellH + 30} stroke="var(--text-muted)" stroke-width="1.5" />

    <!-- Стрелка тока -->
    <path
      d={`M ${res1X + resW/2} ${resY - 25} L ${res1X + resW/2 + 8} ${resY - 35} L ${res1X + resW/2 - 8} ${resY - 35} Z`}
      fill="var(--accent)"
      filter="url(#glow)"
    />
    <text x={res1X + resW/2} y={resY - 42} text-anchor="middle" fill="var(--accent)" font-size="11" font-weight="bold">
      I_CB = {iCb} мА
    </text>
  </svg>
</div>

<style>
  .balancing-visual {
    width: 100%;
    max-width: 500px;
    margin: 0 auto;
    display: flex;
    justify-content: center;
    padding: var(--space-3);
  }
  svg {
    width: 100%;
    height: auto;
    display: block;
  }
</style>