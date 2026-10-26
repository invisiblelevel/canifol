<script>
  import Input from '../../components/Input.svelte';
  import Select from '../../components/Select.svelte';
  import Button from '../../components/Button.svelte';
  import { getStrings } from '../../strings.svelte.js';
  import { wheelerSingleLayer, wheelerMultiLayer } from '../../utils/formulas.js';
  import { formatHenry, formatHenryCompact } from '../../utils/inductorCodes.js';

  const strings = $derived(getStrings());

  let mode = $state('single');
  let diameter = $state('');
  let length = $state('');
  let turns = $state('');
  let layers = $state('2');

  function parseNum(s) {
    if (s == null) return null;
    const t = String(s).trim().replace(',', '.');
    if (!t) return null;
    const n = parseFloat(t);
    return isNaN(n) || n <= 0 ? null : n;
  }

  const d = $derived(parseNum(diameter));
  const l = $derived(parseNum(length));
  const n = $derived(parseNum(turns));
  const lay = $derived(parseNum(layers));

  const result = $derived.by(() => {
    if (mode === 'single') {
      if (d === null || l === null || n === null) return null;
      return wheelerSingleLayer(d, l, n);
    } else {
      if (d === null || l === null || n === null || lay === null) return null;
      return wheelerMultiLayer(d, l, lay, n);
    }
  });

  function reset() {
    diameter = '';
    length = '';
    turns = '';
    layers = '2';
  }

  const statusText = $derived(
    result != null ? strings.inductors.labels.ready : strings.inductors.labels.fillAllFields
  );
  const statusKind = $derived(result != null ? 'ok' : 'hint');
</script>

<div class="geom">
  <div class="mode-switch no-select" role="group" aria-label="Тип катушки">
    <button type="button" class="mode-switch__item" class:active={mode === 'single'} onclick={() => (mode = 'single')}>
      {strings.inductors.labels.single}
    </button>
    <button type="button" class="mode-switch__item" class:active={mode === 'multi'} onclick={() => (mode = 'multi')}>
      {strings.inductors.labels.multi}
    </button>
  </div>

  <div class="fields">
    <Input bind:value={diameter} label={strings.inductors.labels.diameter} placeholder="10" inputmode="decimal" suffix={strings.units.mm} />
    <Input bind:value={length} label={strings.inductors.labels.length} placeholder="20" inputmode="decimal" suffix={strings.units.mm} />
    <Input bind:value={turns} label={strings.inductors.labels.turns} placeholder="20" inputmode="numeric" />
    {#if mode === 'multi'}
      <Input bind:value={layers} label={strings.inductors.labels.layers} placeholder="2" inputmode="numeric" />
    {/if}
  </div>

  <div class="formula">
    <div class="formula__label">{strings.inductors.labels.wheelerFormula}</div>
    <div class="formula__expr">
      {mode === 'single' 
        ? 'L = (r² · N²) / (9r + 10l)' 
        : 'L = (0.8 · r² · N²) / (6r + 9l + 10c)'}
    </div>
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
      <div class="result__actions">
        <Button variant="ghost" size="sm" onclick={reset}>{strings.buttons.reset}</Button>
      </div>
    </div>
  {/if}
</div>

<style>
  .geom { display: flex; flex-direction: column; gap: var(--space-4); }
  .mode-switch { display: flex; gap: var(--space-1); padding: 4px; background: var(--bg-elevated-2); border: 1px solid var(--border-subtle); border-radius: var(--radius-md); align-self: flex-start; flex-wrap: wrap; }
  .mode-switch__item { min-height: 36px; padding: 0 var(--space-3); border-radius: var(--radius-sm); font-size: var(--font-size-sm); font-weight: 500; color: var(--text-secondary); background: transparent; white-space: nowrap; }
  .mode-switch__item.active { background: var(--accent-dim); color: var(--accent); }
  .fields { display: grid; grid-template-columns: repeat(auto-fit, minmax(180px, 1fr)); gap: var(--space-3); }
  .formula { padding: var(--space-3); background: var(--bg-elevated-2); border: 1px solid var(--border-subtle); border-radius: var(--radius-md); }
  .formula__label { font-size: var(--font-size-sm); text-transform: uppercase; color: var(--text-muted); margin-bottom: var(--space-1); }
  .formula__expr { font-family: var(--font-mono); font-size: var(--font-size-base); color: var(--text-primary); }
  .status { font-size: var(--font-size-sm); padding: var(--space-2) var(--space-3); border-radius: var(--radius-md); border: 1px solid transparent; }
  .status--hint { color: var(--text-muted); background: var(--bg-elevated-2); border-color: var(--border-subtle); }
  .status--ok { color: var(--success); background: rgba(139, 196, 122, 0.08); border-color: rgba(139, 196, 122, 0.25); }
  .result { display: flex; flex-direction: column; gap: var(--space-3); padding: var(--space-4); background: var(--bg-elevated-2); border: 1px solid var(--border-subtle); border-radius: var(--radius-lg); }
  .result__main { display: flex; align-items: baseline; gap: var(--space-3); flex-wrap: wrap; }
  .result__value { font-size: clamp(1.5rem, 5vw, 2.25rem); font-weight: 700; color: var(--accent); line-height: 1.1; }
  .result__compact { font-family: var(--font-mono); font-size: var(--font-size-base); color: var(--text-muted); }
  .result__actions { display: flex; gap: var(--space-2); }
  @media (max-width: 480px) { .fields { grid-template-columns: 1fr; } .result { padding: var(--space-3); } }
</style>