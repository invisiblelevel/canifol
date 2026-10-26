<script>
  let { count = 1, tj = 50, tJmax = 100 } = $props();

  const displayCount = Math.min(count, 6);
  const hasMore = count > 6;

  // Цвет MOSFET в зависимости от температуры
  function getColor(temp, maxTemp) {
    const ratio = temp / maxTemp;
    if (ratio > 0.9) return '#e85a5a'; // красный
    if (ratio > 0.75) return '#f7b24a'; // жёлтый
    return '#8bc47a'; // зелёный
  }

  const color = $derived(getColor(tj, tJmax));

  // Геометрия
  const fetW = 40;
  const fetH = 50;
  const gap = 12;
  const padX = 30;
  const padY = 40;
  const VB_W = padX * 2 + displayCount * fetW + (displayCount - 1) * gap + (hasMore ? 30 : 0);
  const VB_H = padY * 2 + fetH + 30;
</script>

{#key `${count}-${tj.toFixed(1)}`}
  <div class="mosfet-visual">
    <svg viewBox={`0 0 ${VB_W} ${VB_H}`} preserveAspectRatio="xMidYMid meet">
      <defs>
        <linearGradient id="fetGrad" x1="0" y1="0" x2="0" y2="1">
          <stop offset="0%" stop-color="#3a3a3a" />
          <stop offset="100%" stop-color="#1a1a1a" />
        </linearGradient>
      </defs>

      {#each Array(displayCount) as _, i}
        {@const x = padX + i * (fetW + gap)}
        {@const y = padY}
        
        <!-- Корпус MOSFET -->
        <rect x={x} y={y} width={fetW} height={fetH} rx="4" fill="url(#fetGrad)" stroke={color} stroke-width="2" />
        
        <!-- Ножки -->
        <line x1={x + 8} y1={y - 10} x2={x + 8} y2={y} stroke="var(--text-muted)" stroke-width="2" />
        <line x1={x + fetW - 8} y1={y - 10} x2={x + fetW - 8} y2={y} stroke="var(--text-muted)" stroke-width="2" />
        <line x1={x + 8} y1={y + fetH} x2={x + 8} y2={y + fetH + 10} stroke="var(--text-muted)" stroke-width="2" />
        <line x1={x + fetW - 8} y1={y + fetH} x2={x + fetW - 8} y2={y + fetH + 10} stroke="var(--text-muted)" stroke-width="2" />
        
        <!-- Иконка радиатора внутри -->
        <path d={`M ${x + 10} ${y + fetH/2 - 8} h ${fetW - 20} v 4 h -${fetW - 20} z`} fill={color} opacity="0.8" />
        <path d={`M ${x + 10} ${y + fetH/2 + 4} h ${fetW - 20} v 4 h -${fetW - 20} z`} fill={color} opacity="0.6" />
        
        <!-- Температура -->
        <text x={x + fetW/2} y={y + fetH + 24} text-anchor="middle" fill={color} font-size="10" font-weight="600">
          {tj.toFixed(0)}°C
        </text>
      {/each}

      {#if hasMore}
        {@const x = padX + displayCount * (fetW + gap)}
        <text x={x} y={padY + fetH/2 + 4} fill="var(--text-muted)" font-size="16" font-weight="bold">...</text>
      {/if}
    </svg>
  </div>
{/key}

<style>
  .mosfet-visual {
    width: 100%;
    max-width: 450px;
    margin: 0 auto;
    display: flex;
    justify-content: center;
    padding: var(--space-3);
  }
  svg { width: 100%; height: auto; display: block; }
</style>