<script>
  let { rShunt = 10, iMax = 100, powerW = 10 } = $props();

  // Цвет индикатора нагрева
  const heatColor = $derived(
    powerW > 5 ? '#e85a5a' : powerW > 2 ? '#f7b24a' : '#8bc47a'
  );
  const heatLabel = $derived(
    powerW > 5 ? 'Сильный нагрев' : powerW > 2 ? 'Умеренный нагрев' : 'Норма'
  );

  // Геометрия
  const VB_W = 500;
  const VB_H = 160;

  const shuntX = 200;
  const shuntY = 60;
  const shuntW = 100;
  const shuntH = 40;
</script>

<div class="shunt-visual">
  <svg viewBox={`0 0 ${VB_W} ${VB_H}`} preserveAspectRatio="xMidYMid meet">
    <defs>
      <linearGradient id="shuntGrad" x1="0" y1="0" x2="0" y2="1">
        <stop offset="0%" stop-color="#3a3a3a" />
        <stop offset="100%" stop-color="#1a1a1a" />
      </linearGradient>
      <linearGradient id="heatGrad" x1="0" y1="0" x2="1" y2="0">
        <stop offset="0%" stop-color="#1a1a1a" />
        <stop offset="100%" stop-color={heatColor} />
      </linearGradient>
    </defs>

    <!-- Провод слева -->
    <line x1="40" y1={shuntY + shuntH/2} x2={shuntX} y2={shuntY + shuntH/2} stroke="var(--text-muted)" stroke-width="3" stroke-linecap="round" />

    <!-- Шунт -->
    <rect x={shuntX} y={shuntY} width={shuntW} height={shuntH} rx="6" fill="url(#shuntGrad)" stroke="var(--border-strong)" stroke-width="1.5" />
    
    <!-- Полоска нагрева -->
    <rect x={shuntX + 4} y={shuntY + shuntH - 6} width={shuntW - 8} height="3" rx="1.5" fill="url(#heatGrad)" />

    <!-- Подпись шунта -->
    <text x={shuntX + shuntW/2} y={shuntY + shuntH/2 + 5} text-anchor="middle" fill="var(--accent)" font-size="14" font-weight="bold">
      {rShunt.toFixed(1)} мОм
    </text>

    <!-- Провод справа -->
    <line x1={shuntX + shuntW} y1={shuntY + shuntH/2} x2={VB_W - 40} y2={shuntY + shuntH/2} stroke="var(--text-muted)" stroke-width="3" stroke-linecap="round" />

    <!-- Стрелка тока (I max) -->
    <path d={`M 90 ${shuntY + shuntH/2 - 25} L 110 ${shuntY + shuntH/2 - 35} L 110 ${shuntY + shuntH/2 - 15} Z`} fill="var(--accent)" />
    <text x="115" y={shuntY + shuntH/2 - 25} fill="var(--accent)" font-size="12" font-weight="bold">I = {iMax} А</text>

    <!-- Падение напряжения (V sense) -->
    <line x1={shuntX} y1={shuntY - 15} x2={shuntX + shuntW} y2={shuntY - 15} stroke="var(--warning)" stroke-width="2" stroke-dasharray="4,2" />
    <text x={shuntX + shuntW/2} y={shuntY - 25} text-anchor="middle" fill="var(--warning)" font-size="12" font-weight="bold">
      V_sense
    </text>

    <!-- Индикатор нагрева (сдвинут вправо, чтобы не наезжать на провод) -->
    <text x={VB_W - 20} y={shuntY + shuntH/2 - 5} text-anchor="end" fill={heatColor} font-size="11" font-weight="600">
      {heatLabel}
    </text>
    <text x={VB_W - 20} y={shuntY + shuntH/2 + 12} text-anchor="end" fill="var(--text-muted)" font-size="10">
      {powerW.toFixed(1)} Вт
    </text>
  </svg>
</div>

<style>
  .shunt-visual {
    width: 100%;
    max-width: 500px;
    margin: 0 auto;
    display: flex;
    justify-content: center;
    padding: var(--space-3);
  }
  svg { width: 100%; height: auto; display: block; }
</style>