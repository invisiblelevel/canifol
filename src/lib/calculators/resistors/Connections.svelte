<script>
  import Input from '../../components/Input.svelte';
  import Select from '../../components/Select.svelte';
  import Button from '../../components/Button.svelte';
  import { getStrings } from '../../strings.svelte.js';
  import { formatOhm, formatOhmCompact } from '../../utils/resistorCodes.js';
  import { sumSeries, sumParallel } from '../../utils/formulas.js';

  const strings = $derived(getStrings());

  let mode = $state('series');
  let nextId = 1;
  let rows = $state([{ id: nextId++, value: '', unit: 'ohm' }, { id: nextId++, value: '', unit: 'ohm' }]);

  const unitOptions = $derived([
    { value: 'ohm', label: strings.units.ohm },
    { value: 'kohm', label: strings.units.kohm },
    { value: 'mohm', label: strings.units.mohm }
  ]);
  const unitFactor = { ohm: 1, kohm: 1e3, mohm: 1e6 };

  function parseValue(str) { if (str == null) return null; const t = String(str).trim().replace(',', '.'); if (!t) return null; const n = parseFloat(t); return isNaN(n) ? null : n; }

  const ohms = $derived(rows.map((r) => { const v = parseValue(r.value); return v !== null ? v * unitFactor[r.unit] : null; }));
  const validOhms = $derived(ohms.filter((v) => v !== null && v >= 0));
  const hasShort = $derived(mode === 'parallel' && validOhms.some((v) => v === 0));

  const result = $derived.by(() => {
    if (validOhms.length === 0) return null;
    return mode === 'series' ? sumSeries(validOhms) : sumParallel(validOhms);
  });

  const formulaText = $derived(mode === 'series' ? 'R = R₁ + R₂ + …' : '1/R = 1/R₁ + 1/R₂ + …');
  const modeLabel = $derived(mode === 'series' ? strings.resistors.labels.series : strings.resistors.labels.parallel);

  function addRow() { rows = [...rows, { id: nextId++, value: '', unit: 'ohm' }]; }
  function removeRow(id) { if (rows.length <= 1) return; rows = rows.filter((r) => r.id !== id); }
  function reset() { nextId = 1; rows = [{ id: nextId++, value: '', unit: 'ohm' }, { id: nextId++, value: '', unit: 'ohm' }]; }

  const copyText = $derived(result != null ? `${formatOhm(result)} (${formatOhmCompact(result)})` : '');
  let copied = $state(false);
  async function copyResult() { if (!copyText) return; try { await navigator.clipboard.writeText(copyText); copied = true; setTimeout(() => (copied = false), 1200); } catch {} }

  const statusText = $derived(
    validOhms.length === 0 ? strings.resistors.labels.enterTwo
    : hasShort ? strings.resistors.labels.invalidColors
    : result != null ? strings.resistors.labels.ready
    : strings.resistors.labels.calcFailed
  );
  const statusKind = $derived(validOhms.length === 0 ? 'hint' : hasShort ? 'warning' : result != null ? 'ok' : 'error');
</script>

<div class="conn">
  <div class="mode-switch no-select">
    <button type="button" class="mode-switch__item" class:active={mode === 'series'} onclick={() => (mode = 'series')}>{strings.resistors.labels.series}</button>
    <button type="button" class="mode-switch__item" class:active={mode === 'parallel'} onclick={() => (mode = 'parallel')}>{strings.resistors.labels.parallel}</button>
  </div>

  <div class="rows">
    {#each rows as row, idx (row.id)}
      <div class="row">
        <div class="row__index">R{idx + 1}</div>
        <div class="row__value"><Input bind:value={row.value} placeholder="0" inputmode="decimal" /></div>
        <Select bind:value={row.unit} options={unitOptions} compact />
        <button type="button" class="row__remove" onclick={() => removeRow(row.id)} disabled={rows.length <= 1}>×</button>
      </div>
    {/each}
  </div>

  <div class="controls">
    <Button variant="secondary" size="sm" onclick={addRow}>{strings.resistors.labels.addResistor}</Button>
    <Button variant="ghost" size="sm" onclick={reset}>{strings.buttons.reset}</Button>
  </div>

  <div class="formula">
    <div class="formula__label">{modeLabel} {strings.resistors.labels.formula}</div>
    <div class="formula__expr">{formulaText}</div>
  </div>

  <div class="status status--{statusKind}">{statusText}</div>

  {#if result != null}
    <div class="result">
      <div class="result__main">
        <div class="result__value">{formatOhm(result)}</div>
        <div class="result__compact">{formatOhmCompact(result)}</div>
      </div>
      <div class="result__actions">
        <Button variant="secondary" size="sm" onclick={copyResult}>{copied ? strings.buttons.copied : strings.buttons.copy}</Button>
      </div>
    </div>
  {/if}
</div>

<style>
  .conn { display: flex; flex-direction: column; gap: var(--space-4); }
  .mode-switch { display: flex; gap: var(--space-1); padding: 4px; background: var(--bg-elevated-2); border: 1px solid var(--border-subtle); border-radius: var(--radius-md); align-self: flex-start; flex-wrap: wrap; }
  .mode-switch__item { min-height: 36px; padding: 0 var(--space-3); border-radius: var(--radius-sm); color: var(--text-secondary); background: transparent; }
  .mode-switch__item.active { background: var(--accent-dim); color: var(--accent); }
  .rows { display: flex; flex-direction: column; gap: var(--space-2); }
  .row { display: flex; align-items: center; gap: var(--space-2); }
  .row__index { width: 32px; text-align: center; color: var(--text-muted); }
  .row__value { flex: 1; }
  .row__remove { width: var(--touch-target); height: var(--touch-target); color: var(--text-muted); }
  .row__remove:disabled { opacity: 0.3; }
  .controls { display: flex; gap: var(--space-2); flex-wrap: wrap; }
  .formula { padding: var(--space-3); background: var(--bg-elevated-2); border: 1px solid var(--border-subtle); border-radius: var(--radius-md); }
  .formula__label { font-size: var(--font-size-sm); text-transform: uppercase; color: var(--text-muted); }
  .formula__expr { font-family: var(--font-mono); color: var(--text-primary); }
  .status { font-size: var(--font-size-sm); padding: var(--space-2) var(--space-3); border-radius: var(--radius-md); border: 1px solid transparent; }
  .status--hint { color: var(--text-muted); background: var(--bg-elevated-2); }
  .status--ok { color: var(--success); background: rgba(102, 187, 106, 0.08); }
  .status--warning { color: var(--warning); background: rgba(255, 167, 38, 0.08); }
  .status--error { color: var(--danger); background: rgba(239, 83, 80, 0.08); }
  .result { display: flex; flex-direction: column; gap: var(--space-3); padding: var(--space-4); background: var(--bg-elevated-2); border: 1px solid var(--border-subtle); border-radius: var(--radius-lg); }
  .result__main { display: flex; align-items: baseline; gap: var(--space-3); }
  .result__value { font-size: 2rem; font-weight: 700; color: var(--accent); }
  .result__compact { font-family: var(--font-mono); color: var(--text-muted); }
  .result__actions { display: flex; gap: var(--space-2); }
</style>