<script>
  import Input from '../../components/Input.svelte';
  import Select from '../../components/Select.svelte';
  import Button from '../../components/Button.svelte';
  import { getStrings } from '../../strings.svelte.js';
  import { formatFarad } from '../../utils/capacitorCodes.js';
  import { findEsr, judgeEsr, formatEsr, ESR_TYPE_KEYS } from '../../utils/esrFormulas.js';

  const strings = $derived(getStrings());
  const L = $derived(strings.capacitors.labels);
  const U = $derived(strings.units);

  // --- Состояние ---
  let capInput = $state('220');      // мкФ
  let voltInput = $state('25');      // В
  let typeInput = $state('general');
  let measuredInput = $state('');    // Ом

  // --- Пресеты ---
  const capPresets = [1, 10, 47, 100, 220, 470, 1000, 2200, 4700];
  const voltPresets = [6.3, 10, 16, 25, 35, 50, 63, 100];

  const typeOptions = $derived([
    { value: 'general',  label: L.esrTypeGeneral },
    { value: 'lowEsr',   label: L.esrTypeLowEsr },
    { value: 'audio',    label: L.esrTypeAudio },
    { value: 'highTemp', label: L.esrTypeHighTemp }
  ]);

  // --- Расчёт ---
  const capVal = $derived(parseFloat(String(capInput).replace(',', '.')) || 0);
  const voltVal = $derived(parseFloat(String(voltInput).replace(',', '.')) || 0);
  const measuredVal = $derived(parseFloat(String(measuredInput).replace(',', '.')) || 0);

  const range = $derived(findEsr(capVal, voltVal, typeInput));
  const verdict = $derived(judgeEsr(measuredVal || null, range));
  const typeLabel = $derived(L[ESR_TYPE_KEYS[typeInput]] ?? '');

  // --- Формат ёмкости ---
  function formatCapDisplay(microfarad) {
    const f = microfarad * 1e-6;
    const { value, unit } = formatFarad(f);
    return `${value} ${U[unit] || unit}`;
  }

  function pickCap(v) { capInput = String(v); }
  function pickVolt(v) { voltInput = String(v); }

  const verdictClass = $derived(
    verdict.level === 'ok'    ? 'verdict--ok'    :
    verdict.level === 'low'   ? 'verdict--warn'  :
    verdict.level === 'high'  ? 'verdict--warn'  :
    verdict.level === 'bad'   ? 'verdict--bad'   : ''
  );
</script>

<div class="esr">
  <!-- Блок ввода -->
  <div class="esr__inputs">
    <Input
      bind:value={capInput}
      label={L.esrCapacitance}
      inputmode="decimal"
      suffix={U.uf}
    />
    <Input
      bind:value={voltInput}
      label={L.esrVoltage}
      inputmode="decimal"
      suffix={U.v}
    />
    <Select
      bind:value={typeInput}
      label={L.esrType}
      options={typeOptions}
    />
  </div>

  <!-- Пресеты ёмкости -->
  <div class="esr__presets">
    <span class="esr__presets-label">{L.esrPresetsCap}:</span>
    {#each capPresets as p (p)}
      <button
        type="button"
        class="preset-btn"
        class:active={capVal === p}
        onclick={() => pickCap(p)}
      >{formatCapDisplay(p)}</button>
    {/each}
  </div>

  <!-- Пресеты напряжения -->
  <div class="esr__presets">
    <span class="esr__presets-label">{L.esrPresetsVolt}:</span>
    {#each voltPresets as p (p)}
      <button
        type="button"
        class="preset-btn"
        class:active={voltVal === p}
        onclick={() => pickVolt(p)}
      >{p} {U.v}</button>
    {/each}
  </div>

  <!-- Результат: норма -->
  {#if range}
    <div class="esr__result">
      <div class="esr__result-header">
        {formatCapDisplay(capVal)} / {voltVal} {U.v} · {typeLabel}
      </div>
      <div class="esr__row">
        <span class="esr__row-label">{L.esrRange}:</span>
        <span class="esr__row-value">{formatEsr(range.min, U)} — {formatEsr(range.max, U)}</span>
      </div>
      <div class="esr__row">
        <span class="esr__row-label">{L.esrBadThreshold}:</span>
        <span class="esr__row-value">&gt; {formatEsr(range.bad, U)}</span>
      </div>
    </div>
  {:else}
    <p class="esr__hint">{L.esrEnterCap}</p>
  {/if}

  <!-- Проверка замера -->
  <div class="esr__check">
    <Input
      bind:value={measuredInput}
      label={L.esrMeasured}
      placeholder={L.esrMeasuredPlaceholder}
      inputmode="decimal"
      suffix={U.ohm}
    />
    {#if measuredVal > 0 && range}
      <div class="verdict {verdictClass}">
        <span class="verdict__icon">
          {#if verdict.level === 'ok'}✓
          {:else if verdict.level === 'bad'}✕
          {:else if verdict.level === 'low' || verdict.level === 'high'}!
          {:else}?
          {/if}
        </span>
        <span class="verdict__text">{L[verdict.key]}</span>
      </div>
    {/if}
  </div>

  <p class="esr__disclaimer">{L.esrDisclaimer}</p>
</div>

<style>
  .esr { display: flex; flex-direction: column; gap: var(--space-4); }

  .esr__inputs {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: var(--space-3);
  }
  @media (max-width: 768px) {
    .esr__inputs { grid-template-columns: 1fr; }
  }

  .esr__presets {
    display: flex;
    flex-wrap: wrap;
    gap: var(--space-2);
    align-items: center;
  }
  .esr__presets-label {
    font-size: var(--font-size-sm);
    color: var(--text-muted);
    margin-right: var(--space-2);
  }
  .preset-btn {
    min-height: 32px;
    padding: 0 var(--space-3);
    border-radius: var(--radius-md);
    font-size: var(--font-size-sm);
    color: var(--text-secondary);
    background: var(--bg-elevated-2);
    border: 1px solid var(--border-subtle);
    font-family: var(--font-mono);
    transition: background var(--transition-fast), color var(--transition-fast), border-color var(--transition-fast);
  }
  .preset-btn:hover { color: var(--text-primary); border-color: var(--border-strong); }
  .preset-btn.active { background: var(--accent-dim); color: var(--accent); border-color: var(--accent); }

  .esr__result {
    background: var(--bg-elevated-2);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-md);
    padding: var(--space-3) var(--space-4);
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }
  .esr__result-header {
    font-size: var(--font-size-sm);
    font-weight: 600;
    color: var(--accent);
    margin-bottom: var(--space-1);
  }
  .esr__row {
    display: flex;
    justify-content: space-between;
    align-items: baseline;
    gap: var(--space-2);
  }
  .esr__row-label {
    font-size: var(--font-size-sm);
    color: var(--text-muted);
  }
  .esr__row-value {
    font-family: var(--font-mono);
    font-size: var(--font-size-base);
    font-weight: 600;
    color: var(--text-primary);
  }

  .esr__hint {
    color: var(--text-muted);
    font-size: var(--font-size-sm);
    font-style: italic;
  }

  .esr__check { display: flex; flex-direction: column; gap: var(--space-3); }

  .verdict {
    display: flex;
    align-items: center;
    gap: var(--space-3);
    padding: var(--space-3) var(--space-4);
    border-radius: var(--radius-md);
    border: 1px solid transparent;
    font-size: var(--font-size-base);
    font-weight: 600;
  }
  .verdict__icon {
    width: 28px; height: 28px;
    border-radius: 50%;
    display: flex; align-items: center; justify-content: center;
    font-size: 16px;
    font-weight: 700;
    flex-shrink: 0;
  }
  .verdict--ok {
    background: rgba(76, 175, 80, 0.12);
    border-color: rgba(76, 175, 80, 0.4);
    color: #8fd89a;
  }
  .verdict--ok .verdict__icon { background: rgba(76, 175, 80, 0.25); color: #a8e6b1; }
  .verdict--warn {
    background: rgba(255, 193, 7, 0.12);
    border-color: rgba(255, 193, 7, 0.4);
    color: #f5c542;
  }
  .verdict--warn .verdict__icon { background: rgba(255, 193, 7, 0.25); color: #ffd966; }
  .verdict--bad {
    background: rgba(242, 106, 106, 0.12);
    border-color: rgba(242, 106, 106, 0.45);
    color: #f58a8a;
  }
  .verdict--bad .verdict__icon { background: rgba(242, 106, 106, 0.25); color: #ffb3b3; }

  .esr__disclaimer {
    font-size: 12px;
    color: var(--text-muted);
    line-height: 1.5;
    padding: var(--space-2) var(--space-3);
    background: var(--bg-base);
    border-left: 2px solid var(--border-strong);
    border-radius: var(--radius-sm);
  }
</style>