<script>
  import Select from '../../components/Select.svelte';
  import Button from '../../components/Button.svelte';
  import ResistorVisual from '../../components/ResistorVisual.svelte';
  import { RESISTOR_COLORS, parseColorBands, formatOhm, formatOhmCompact } from '../../utils/resistorCodes.js';

  const DIGIT_COLORS = RESISTOR_COLORS.filter((c) => c.digit != null);
  const MULTIPLIER_COLORS = RESISTOR_COLORS.filter((c) => c.multiplier != null);
  const TOLERANCE_COLORS = RESISTOR_COLORS.filter((c) => c.tolerance != null);
  const TCR_COLORS = RESISTOR_COLORS.filter((c) => c.tcr != null);

  let bandCount = $state(4);
  let b1 = $state('brown'), b2 = $state('black'), b3 = $state('black'), mult = $state('red'), tol = $state('gold'), tcr = $state('brown');

  const activeColors = $derived.by(() => {
    const list = [b1, b2];
    if (bandCount >= 5) list.push(b3);
    list.push(mult);
    if (bandCount >= 4) list.push(tol);
    if (bandCount >= 6) list.push(tcr);
    return list;
  });

  const parsed = $derived(parseColorBands(activeColors));
  const copyText = $derived(parsed ? formatOhm(parsed.value) + (parsed.tolerance != null ? ` ±${parsed.tolerance}%` : '') + (parsed.tcr != null ? ` ${parsed.tcr} ppm/°C` : '') : '');
  let copied = $state(false);

  async function copyResult() {
    if (!copyText) return;
    try { await navigator.clipboard.writeText(copyText); copied = true; setTimeout(() => (copied = false), 1200); } catch {}
  }

  function reset() { bandCount = 4; b1 = 'brown'; b2 = 'black'; b3 = 'black'; mult = 'red'; tol = 'gold'; tcr = 'brown'; }

  function colorOptions(list) { return list.map((c) => ({ value: c.id, label: c.name })); }

  const standardLabel = $derived(({ 3: '3 полосы', 4: '4 полосы', 5: '5 полос', 6: '6 полос' })[bandCount] ?? '');
</script>

<div class="cc">
  <div class="bands-switch no-select">
    {#each [3, 4, 5, 6] as n}
      <button type="button" class="bands-switch__item" class:active={bandCount === n} onclick={() => (bandCount = n)}>{n}</button>
    {/each}
  </div>

  <div class="preview"><ResistorVisual bands={activeColors} width={420} height={90} /></div>

  <div class="selects">
    <Select bind:value={b1} label="Полоса 1" options={colorOptions(DIGIT_COLORS)} />
    <Select bind:value={b2} label="Полоса 2" options={colorOptions(DIGIT_COLORS)} />
    {#if bandCount >= 5}<Select bind:value={b3} label="Цифра 3" options={colorOptions(DIGIT_COLORS)} />{/if}
    <Select bind:value={mult} label="Множитель" options={colorOptions(MULTIPLIER_COLORS)} />
    {#if bandCount >= 4}<Select bind:value={tol} label="Допуск" options={colorOptions(TOLERANCE_COLORS)} />{/if}
    {#if bandCount >= 6}<Select bind:value={tcr} label="ТКС" options={colorOptions(TCR_COLORS)} />{/if}
  </div>

  {#if parsed}
    <div class="result">
      <div class="result__main">
        <div class="result__value">{formatOhm(parsed.value)}</div>
        <div class="result__compact">{formatOhmCompact(parsed.value)}</div>
      </div>
      <div class="result__meta">
        <span class="badge">{standardLabel}</span>
        {#if parsed.tolerance != null}<span class="badge">±{parsed.tolerance}%</span>{/if}
        {#if parsed.tcr != null}<span class="badge">{parsed.tcr} ppm/°C</span>{/if}
      </div>
      <div class="result__actions">
        <Button variant="secondary" size="sm" onclick={copyResult}>{copied ? 'Скопировано' : 'Копировать'}</Button>
        <Button variant="ghost" size="sm" onclick={reset}>Сброс</Button>
      </div>
    </div>
  {:else}
    <div class="error-note">Выбранная комбинация цветов некорректна</div>
  {/if}
</div>

<style>
  .cc { display: flex; flex-direction: column; gap: var(--space-4); }
  .bands-switch { display: flex; gap: var(--space-1); padding: 4px; background: var(--bg-elevated-2); border: 1px solid var(--border-subtle); border-radius: var(--radius-md); align-self: flex-start; }
  .bands-switch__item { min-width: 44px; min-height: 36px; padding: 0 var(--space-3); border-radius: var(--radius-sm); color: var(--text-secondary); background: transparent; }
  .bands-switch__item.active { background: var(--accent-dim); color: var(--accent); }
  .preview { display: flex; justify-content: center; padding: var(--space-3); background: var(--bg-elevated-2); border-radius: var(--radius-lg); border: 1px solid var(--border-subtle); }
  .selects { display: grid; grid-template-columns: repeat(auto-fit, minmax(140px, 1fr)); gap: var(--space-3); }
  .result { display: flex; flex-direction: column; gap: var(--space-3); padding: var(--space-4); background: var(--bg-elevated-2); border: 1px solid var(--border-subtle); border-radius: var(--radius-lg); }
  .result__main { display: flex; align-items: baseline; gap: var(--space-3); flex-wrap: wrap; }
  .result__value { font-size: 2rem; font-weight: 700; color: var(--accent); }
  .result__compact { font-family: var(--font-mono); color: var(--text-muted); }
  .result__meta { display: flex; gap: var(--space-2); flex-wrap: wrap; }
  .badge { padding: 4px 10px; font-size: var(--font-size-sm); color: var(--text-secondary); background: var(--bg-elevated); border: 1px solid var(--border-subtle); border-radius: 999px; }
  .result__actions { display: flex; gap: var(--space-2); flex-wrap: wrap; }
  .error-note { padding: var(--space-3); color: var(--danger); background: rgba(239, 83, 80, 0.08); border: 1px solid rgba(239, 83, 80, 0.25); border-radius: var(--radius-md); font-size: var(--font-size-sm); }
</style>