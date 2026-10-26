<script>
  let { s = 3, p = 4 } = $props();

  const sNum = $derived(Number(s) || 0);
  const pNum = $derived(Number(p) || 0);

  const displayS = $derived(Math.min(sNum, 6));
  const displayP = $derived(Math.min(pNum, 6));
  const hasMoreS = $derived(sNum > 6);
  const hasMoreP = $derived(pNum > 6);

  const cellW = 24;
  const cellH = 36;
  const gapX = 10;
  const gapY = 14;
  const padX = 60; // Увеличили отступ слева, чтобы B- не обрезался
  const padY = 30;

  const VB_W = $derived(padX * 2 + displayP * cellW + (displayP - 1) * gapX + (hasMoreP ? 20 : 0));
  const VB_H = $derived(padY * 2 + displayS * cellH + (displayS - 1) * gapY + (hasMoreS ? 30 : 0));
</script>

{#key `${sNum}-${pNum}`}
  <div class="pack-visual">
    <svg viewBox={`0 0 ${VB_W} ${VB_H}`} preserveAspectRatio="xMidYMid meet">
      <defs>
        <linearGradient id="cellGradPack" x1="0" y1="0" x2="0" y2="1">
          <stop offset="0%" stop-color="#3a3a3a" />
          <stop offset="100%" stop-color="#1a1a1a" />
        </linearGradient>
        <linearGradient id="terminalGrad" x1="0" y1="0" x2="0" y2="1">
          <stop offset="0%" stop-color="#f5b83d" />
          <stop offset="100%" stop-color="#b8860b" />
        </linearGradient>
      </defs>

      <!-- Параллельные соединения (внутри групп) -->
      {#each Array(displayS) as _, row}
        {@const yTop = padY + row * (cellH + gapY) - 6}
        {@const yBot = padY + row * (cellH + gapY) + cellH + 6}
        <line x1={padX} y1={yTop} x2={padX + displayP * cellW + (displayP - 1) * gapX} y2={yTop} stroke="var(--accent)" stroke-width="2.5" stroke-linecap="round" />
        <line x1={padX} y1={yBot} x2={padX + displayP * cellW + (displayP - 1) * gapX} y2={yBot} stroke="var(--text-secondary)" stroke-width="2.5" stroke-linecap="round" />
      {/each}

      <!-- Последовательные соединения (между группами) -->
      {#each Array(displayS - 1) as _, row}
        {@const x = padX + (displayP * cellW + (displayP - 1) * gapX) / 2}
        {@const y1 = padY + row * (cellH + gapY) + cellH + 6}
        {@const y2 = padY + (row + 1) * (cellH + gapY) - 6}
        <line x1={x} y1={y1} x2={x} y2={y2} stroke="var(--warning)" stroke-width="3" stroke-linecap="round" />
      {/each}

      <!-- Балансировочные провода -->
      {#each Array(displayS) as _, row}
        {@const y = padY + row * (cellH + gapY) + cellH / 2}
        <line x1={padX - 10} y1={y} x2={padX} y2={y} stroke="var(--warning)" stroke-width="1.5" stroke-dasharray="3,3" />
        <text x={padX - 14} y={y + 3} text-anchor="end" fill="var(--warning)" font-size="10" font-weight="600">B{row + 1}</text>
      {/each}

      <!-- Банки -->
      {#each Array(displayS) as _, row}
        {#each Array(displayP) as _, col}
          {@const x = padX + col * (cellW + gapX)}
          {@const y = padY + row * (cellH + gapY)}
          <rect x={x} y={y} width={cellW} height={cellH} rx="3" fill="url(#cellGradPack)" stroke="var(--border-strong)" stroke-width="1" />
          <rect x={x + cellW/2 - 3} y={y - 3} width="6" height="3" fill="url(#terminalGrad)" rx="1" />
          <rect x={x + cellW/2 - 3} y={y + cellH} width="6" height="3" fill="var(--text-muted)" rx="1" />
        {/each}
        {#if hasMoreP}
          {@const x = padX + displayP * (cellW + gapX)}
          {@const y = padY + row * (cellH + gapY) + cellH / 2}
          <text x={x} y={y + 4} fill="var(--text-muted)" font-size="14" font-weight="bold">...</text>
        {/if}
      {/each}

      {#if hasMoreS}
        {@const y = padY + displayS * (cellH + gapY)}
        <text x={VB_W / 2} y={y + 4} text-anchor="middle" fill="var(--text-muted)" font-size="14" font-weight="bold">...</text>
      {/if}

      <!-- Клеммы B+ и B- (с увеличенным отступом) -->
      <line x1={padX} y1={padY - 6} x2={padX - 40} y2={padY - 6} stroke="var(--accent)" stroke-width="3" stroke-linecap="round" />
      <text x={padX - 42} y={padY - 2} text-anchor="end" fill="var(--accent)" font-size="14" font-weight="700">B+</text>

      <line
        x1={padX}
        y1={padY + (displayS - 1) * (cellH + gapY) + cellH + 6}
        x2={padX - 40}
        y2={padY + (displayS - 1) * (cellH + gapY) + cellH + 6}
        stroke="var(--text-secondary)"
        stroke-width="3"
        stroke-linecap="round"
      />
      <text
        x={padX - 42}
        y={padY + (displayS - 1) * (cellH + gapY) + cellH + 10}
        text-anchor="end"
        fill="var(--text-secondary)"
        font-size="14"
        font-weight="700"
      >B-</text>
    </svg>
  </div>
{/key}

<style>
  .pack-visual {
    width: 100%;
    max-width: 550px;
    margin: 0 auto;
    display: flex;
    justify-content: center;
    padding: var(--space-3);
  }
  svg { width: 100%; height: auto; display: block; }
</style>