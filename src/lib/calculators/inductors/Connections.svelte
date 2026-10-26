<script>
  import Input from '../../components/Input.svelte';
  import Select from '../../components/Select.svelte';
  import Button from '../../components/Button.svelte';
  import { getStrings } from '../../strings.svelte.js';
  import { sumSeriesL, sumParallelL } from '../../utils/formulas.js';
  import { formatHenry, formatHenryCompact } from '../../utils/inductorCodes.js';

  const strings = $derived(getStrings());

  let mode = $state('series');
  let nextId = 1;

  let rows = $state([
    { id: nextId++, value: '', unit: 'uh' },
    { id: nextId++, value: '', unit: 'uh' }
  ]);

  const unitOptions = $derived([
    { value: 'nh', label: strings.units.nh },
    { value: 'uh', label: strings.units.uh },
    { value: 'mh', label: strings.units.mh },
    { value: 'h', label: strings.units.h }
  ]);
  const unitFactor = { nh: 1e-9, uh: 1e-6, mh: 1e-3, h: 1 };

  function parseValue(str) {
    if (str == null) return null;
    const t = String(str).trim().replace(',', '.');
    if (!t) return null;
    const n = parseFloat(t);
    return isNaN(n) ? null : n;
  }

  const henries = $derived(
    rows.map((r) => {
      const v = parseValue(r.value);
      return v !== null ? v * unitFactor[r.unit] : null;
    })
  );

  const validL = $derived(henries.filter((v) => v !== null && v > 0));

  const result = $derived.by(() => {
    if (validL.length === 0) return null;
    return mode === 'series' ? sumSeriesL(validL) : sumParallelL(validL);
  });

  const formulaText = $derived(
    mode === 'series' ? 'L = L₁ + L₂ + …' : '1/L = 1/L₁ + 1/L₂ + …'
  );

  const modeLabel = $derived(mode === 'series' ? strings.inductors.labels.series : strings.inductors.labels.parallel);

  function addRow() {
    rows = [...rows, { id: nextId++, value: '', unit: 'uh' }];
  }
  function removeRow(id) {
    if (rows.length <= 1) return;
    rows = rows.filter((r) => r.id !== id);
  }
  function reset() {
    nextId = 1;
    rows = [
      { id: nextId++, value: '', unit: 'uh' },
      { id: nextId++, value: '', unit: 'uh' }
    ];
  }

  const statusText = $derived(
    validL.length === 0 ? strings.inductors.labels.enterOneInductance
    : result != null ? strings.inductors.labels.ready : strings.inductors.labels.calcFailed
  );
  const statusKind = $derived(
    validL.length === 0 ? 'hint' : result != null ? 'ok' : 'error'
  );
</script>

<div class="conn">
  <div class="mode-switch no-select" role="group" aria-label="Тип соединения">
    <button type="button" class="mode-switch__item" class:active={mode === 'series'} onclick={() => (mode = 'series')}>
      {strings.inductors.labels.series}
    </button>
    <button type="button" class="mode-switch__item" class:active={mode === 'parallel'} onclick={() => (mode = 'parallel')}>
      {strings.inductors.labels.parallel}
    </button>
  </div>

  <div class="rows">
    {#each rows as row, idx (row.id)}
      <div class="row">
        <div class="row__index">L{idx + 1}</div>
        <div class="row__value">
          <Input bind:value={row.value} placeholder="0" inputmode="decimal" aria-label="L{idx + 1}" />
        </div>
        <Select bind:value={row.unit} options={unitOptions} compact aria-label="L{idx + 1}" />
        <button type="button" class="row__remove" aria-label={strings.buttons.remove} onclick={() => removeRow(row.id)} disabled={rows.length <= 1}>×</button>
      </div>
    {/each}
  </div>

  <div class="controls">
    <Button variant="secondary" size="sm" onclick={addRow}>{strings.inductors.labels.addInductor}</Button>
    <Button variant="ghost" size="sm" onclick={reset}>{strings.buttons.reset}</Button>
  </div>

  <div class="formula">
    <div class="formula__label">{modeLabel}</div>
    <div class="formula__expr">{formulaText}</div>
  </div>

  <div class="status status--{statusKind}" aria-live="polite">{statusText}</div>

  {#if result != null}
    <div class="result" aria-live="polite">
      <div class="result__main">
        <div class="result__value">
          {formatHenry(result).value} {strings.units[formatHenry(result).unit]}
        </div>
        <div class="result__compact">{formatHenryCompact(result)}</div>
      </div>
      {#if validL.length > 1}
        <div class="result__breakdown">
          {#each validL as v, i (i)}
            <span class="chip">{formatHenryCompact(v)}</span>
            {#if i < validL.length - 1}<span class="chip-op">{mode === 'series' ? '+' : '∥'}</span>{/if}
          {/each}
          <span class="chip-op">=</span>
          <span class="chip chip--total">{formatHenryCompact(result)}</span>
        </div>
      {/if}
    </div>
  {/if}
</div>

<style>
  .conn { display: flex; flex-direction: column; gap: var(--space-4); }
  .mode-switch { display: flex; gap: var(--space-1); padding: 4px; background: var(--bg-elevated-2); border: 1px solid var(--border-subtle); border-radius: var(--radius-md); align-self: flex-start; flex-wrap: wrap; }
  .mode-switch__item { min-height: 36px; padding: 0 var(--space-3); border-radius: var(--radius-sm); font-size: var(--font-size-sm); font-weight: 500; color: var(--text-secondary); background: transparent; white-space: nowrap; }
  .mode-switch__item.active { background: var(--accent-dim); color: var(--accent); }
  .rows { display: flex; flex-direction: column; gap: var(--space-2); }
  .row { display: flex; align-items: center; gap: var(--space-2); }
  .row__index { flex-shrink: 0; width: 32px; font-family: var(--font-mono); font-size: var(--font-size-sm); color: var(--text-muted); text-align: center; }
  .row__value { flex: 1; }
  .row__remove { flex-shrink: 0; width: var(--touch-target); height: var(--touch-target); display: flex; align-items: center; justify-content: center; border-radius: var(--radius-md); color: var(--text-muted); font-size: 1.25rem; }
  .row__remove:hover:not(:disabled) { background: rgba(232, 90, 90, 0.12); color: var(--danger); }
  .row__remove:disabled { opacity: 0.3; cursor: not-allowed; }
  .controls { display: flex; gap: var(--space-2); flex-wrap: wrap; }
  .formula { padding: var(--space-3); background: var(--bg-elevated-2); border: 1px solid var(--border-subtle); border-radius: var(--radius-md); }
  .formula__label { font-size: var(--font-size-sm); text-transform: uppercase; color: var(--text-muted); margin-bottom: var(--space-1); }
  .formula__expr { font-family: var(--font-mono); font-size: var(--font-size-base); color: var(--text-primary); }
  .status { font-size: var(--font-size-sm); padding: var(--space-2) var(--space-3); border-radius: var(--radius-md); border: 1px solid transparent; }
  .status--hint { color: var(--text-muted); background: var(--bg-elevated-2); border-color: var(--border-subtle); }
  .status--ok { color: var(--success); background: rgba(139, 196, 122, 0.08); border-color: rgba(139, 196, 122, 0.25); }
  .status--error { color: var(--danger); background: rgba(232, 90, 90, 0.08); border-color: rgba(232, 90, 90, 0.25); }
  .result { display: flex; flex-direction: column; gap: var(--space-3); padding: var(--space-4); background: var(--bg-elevated-2); border: 1px solid var(--border-subtle); border-radius: var(--radius-lg); }
  .result__main { display: flex; align-items: baseline; gap: var(--space-3); flex-wrap: wrap; }
  .result__value { font-size: clamp(1.5rem, 5vw, 2.25rem); font-weight: 700; color: var(--accent); line-height: 1.1; }
  .result__compact { font-family: var(--font-mono); font-size: var(--font-size-base); color: var(--text-muted); }
  .result__breakdown { display: flex; flex-wrap: wrap; align-items: center; gap: var(--space-1); font-family: var(--font-mono); font-size: var(--font-size-sm); color: var(--text-secondary); }
  .chip { padding: 2px 8px; background: var(--bg-elevated); border: 1px solid var(--border-subtle); border-radius: var(--radius-sm); }
  .chip--total { color: var(--accent); border-color: var(--accent-glow); }
  .chip-op { color: var(--text-muted); padding: 0 2px; }
  @media (max-width: 768px) { .row__index { width: 28px; } .result { padding: var(--space-3); } }
</style>