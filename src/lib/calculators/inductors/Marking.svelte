<script>
  import Input from '../../components/Input.svelte';
  import Button from '../../components/Button.svelte';
  import InductorVisual from '../../components/InductorVisual.svelte';
  import { getStrings } from '../../strings.svelte.js';
  import { parseSmdInductor, parseInductorColors, INDUCTOR_COLORS, formatHenry, formatHenryCompact } from '../../utils/inductorCodes.js';

  const strings = $derived(getStrings());

  let mode = $state('smd');
  let copied = $state(false);
  let copyText = $state('');

  async function copyResult() {
    if (!copyText) return;
    try {
      await navigator.clipboard.writeText(copyText);
      copied = true;
      setTimeout(() => (copied = false), 1200);
    } catch {}
  }

  let smdStr = $state('');
  let smdEl = $state(null);

  $effect(() => {
    if (mode === 'smd') smdEl?.focus?.();
  });

  const smdNormalized = $derived(smdStr.trim().toUpperCase());
  const smdParsed = $derived(smdNormalized ? parseSmdInductor(smdNormalized) : null);
  const smdError = $derived(!!smdNormalized && !smdParsed);

  $effect(() => {
    if (mode === 'smd') {
      if (smdParsed) {
        const f = formatHenry(smdParsed.value);
        copyText = `${f.value} ${strings.units[f.unit]}`;
      } else {
        copyText = '';
      }
    }
  });

  const COLOR_OPTIONS = $derived(
    INDUCTOR_COLORS.map((c) => ({ value: c.id, label: strings.colors[c.id] ?? c.name }))
  );

  let bandCount = $state(4);
  let c1 = $state('brown');
  let c2 = $state('black');
  let c3 = $state('red');
  let tol = $state('gold');

  const activeColors = $derived.by(() => {
    const list = [c1, c2, c3];
    if (bandCount >= 4) list.push(tol);
    return list;
  });

  const colorParsed = $derived(parseInductorColors(activeColors));

  $effect(() => {
    if (mode === 'color') {
      if (colorParsed) {
        const f = formatHenry(colorParsed.value);
        copyText = `${f.value} ${strings.units[f.unit]}`;
      } else {
        copyText = '';
      }
    }
  });

  function reset() {
    if (mode === 'smd') smdStr = '';
    else {
      bandCount = 4;
      c1 = 'brown';
      c2 = 'black';
      c3 = 'red';
      tol = 'gold';
    }
  }
</script>

<div class="marking">
  <div class="modes no-select" role="group" aria-label="Тип маркировки">
    <button type="button" class="modes__item" class:active={mode === 'smd'} onclick={() => (mode = 'smd')}>
      {strings.markingTabs.smd}
    </button>
    <button type="button" class="modes__item" class:active={mode === 'color'} onclick={() => (mode = 'color')}>
      {strings.markingTabs.color}
    </button>
  </div>

  {#if mode === 'smd'}
    <Input
      bind:value={smdStr}
      bind:this={smdEl}
      label={strings.resistors.labels.smd}
      placeholder={strings.inductors.labels.smdIndPlaceholder}
      hint={strings.inductors.labels.smdIndHint}
      error={smdError ? strings.resistors.labels.smdError : ''}
      autocapitalize="characters"
      autocorrect="off"
      spellcheck={false}
      autocomplete="off"
      maxlength={4}
    />

    {#if smdParsed}
      <div class="result" aria-live="polite">
        <div class="result__main">
          <div class="result__value">
            {formatHenry(smdParsed.value).value} {strings.units[formatHenry(smdParsed.value).unit]}
          </div>
          <div class="result__compact">{formatHenryCompact(smdParsed.value)}</div>
        </div>
        <div class="result__meta">
          <span class="badge">{smdParsed.standard}</span>
        </div>
        <div class="result__actions">
          <Button variant="secondary" size="sm" onclick={copyResult}>
            {copied ? strings.buttons.copied : strings.buttons.copy}
          </Button>
          <Button variant="ghost" size="sm" onclick={reset}>{strings.buttons.clear}</Button>
        </div>
      </div>
    {/if}
  {:else}
    <div class="bands-switch no-select" role="group" aria-label={strings.capacitors.labels.bandCount}>
      {#each [3, 4] as n}
        <button
          type="button"
          class="bands-switch__item"
          class:active={bandCount === n}
          onclick={() => (bandCount = n)}
        >
          {n}
        </button>
      {/each}
    </div>

    <div class="preview">
      <InductorVisual bands={activeColors} width={420} />
    </div>

    <div class="selects">
      <label class="sel">
        <span class="sel__label">{strings.capacitors.labels.digit1}</span>
        <select bind:value={c1}>
          {#each COLOR_OPTIONS as opt (opt.value)}
            <option value={opt.value}>{opt.label}</option>
          {/each}
        </select>
      </label>
      <label class="sel">
        <span class="sel__label">{strings.capacitors.labels.digit2}</span>
        <select bind:value={c2}>
          {#each COLOR_OPTIONS as opt (opt.value)}
            <option value={opt.value}>{opt.label}</option>
          {/each}
        </select>
      </label>
      <label class="sel">
        <span class="sel__label">{strings.capacitors.labels.multiplier}</span>
        <select bind:value={c3}>
          {#each COLOR_OPTIONS as opt (opt.value)}
            <option value={opt.value}>{opt.label}</option>
          {/each}
        </select>
      </label>
      {#if bandCount >= 4}
        <label class="sel">
          <span class="sel__label">{strings.capacitors.labels.tolerance}</span>
          <select bind:value={tol}>
            {#each COLOR_OPTIONS as opt (opt.value)}
              <option value={opt.value}>{opt.label}</option>
            {/each}
          </select>
        </label>
      {/if}
    </div>

    {#if colorParsed}
      <div class="result" aria-live="polite">
        <div class="result__main">
          <div class="result__value">
            {formatHenry(colorParsed.value).value} {strings.units[formatHenry(colorParsed.value).unit]}
          </div>
          <div class="result__compact">{formatHenryCompact(colorParsed.value)}</div>
        </div>
        <div class="result__meta">
          {#if colorParsed.tolerance != null}
            <span class="badge">±{colorParsed.tolerance}%</span>
          {/if}
        </div>
        <div class="result__actions">
          <Button variant="secondary" size="sm" onclick={copyResult}>
            {copied ? strings.buttons.copied : strings.buttons.copy}
          </Button>
          <Button variant="ghost" size="sm" onclick={reset}>{strings.buttons.reset}</Button>
        </div>
      </div>
    {:else}
      <div class="error-note">{strings.capacitors.labels.invalidColorsCap}</div>
    {/if}
  {/if}
</div>

<style>
  .marking { display: flex; flex-direction: column; gap: var(--space-4); }
  .modes { display: flex; gap: var(--space-1); padding: 4px; background: var(--bg-elevated-2); border: 1px solid var(--border-subtle); border-radius: var(--radius-md); align-self: flex-start; }
  .modes__item { min-height: 36px; padding: 0 var(--space-4); border-radius: var(--radius-sm); font-size: var(--font-size-sm); font-weight: 600; color: var(--text-secondary); background: transparent; transition: background var(--transition-fast), color var(--transition-fast); }
  .modes__item:hover { color: var(--text-primary); }
  .modes__item.active { background: var(--accent-dim); color: var(--accent); }
  .bands-switch { display: flex; gap: var(--space-1); padding: 4px; background: var(--bg-elevated-2); border: 1px solid var(--border-subtle); border-radius: var(--radius-md); align-self: flex-start; }
  .bands-switch__item { min-width: 44px; min-height: 36px; padding: 0 var(--space-3); border-radius: var(--radius-sm); font-size: var(--font-size-sm); font-weight: 600; color: var(--text-secondary); background: transparent; transition: background var(--transition-fast), color var(--transition-fast); }
  .bands-switch__item:hover { color: var(--text-primary); }
  .bands-switch__item.active { background: var(--accent-dim); color: var(--accent); }
  .preview { display: flex; justify-content: center; padding: var(--space-3); background: linear-gradient(180deg, var(--bg-elevated-2) 0%, var(--bg-elevated) 100%); border-radius: var(--radius-lg); border: 1px solid var(--border-subtle); }
  .selects { display: grid; grid-template-columns: repeat(auto-fit, minmax(140px, 1fr)); gap: var(--space-3); }
  .sel { display: flex; flex-direction: column; gap: var(--space-2); }
  .sel__label { font-size: var(--font-size-sm); font-weight: 500; color: var(--text-secondary); }
  .sel select { min-height: var(--touch-target); padding: 0 var(--space-3); background: var(--bg-elevated-2); border: 1px solid var(--border-strong); border-radius: var(--radius-md); color: var(--text-primary); font-size: var(--font-size-base); cursor: pointer; }
  .sel select:focus { outline: none; border-color: var(--accent); box-shadow: 0 0 0 3px var(--accent-dim); }
  .result { display: flex; flex-direction: column; gap: var(--space-3); padding: var(--space-4); background: var(--bg-elevated-2); border: 1px solid var(--border-subtle); border-radius: var(--radius-lg); }
  .result__main { display: flex; align-items: baseline; gap: var(--space-3); flex-wrap: wrap; }
  .result__value { font-size: clamp(1.5rem, 5vw, 2.25rem); font-weight: 700; color: var(--accent); line-height: 1.1; }
  .result__compact { font-family: var(--font-mono); font-size: var(--font-size-base); color: var(--text-muted); }
  .result__meta { display: flex; gap: var(--space-2); flex-wrap: wrap; }
  .badge { display: inline-flex; align-items: center; padding: 4px 10px; font-size: var(--font-size-sm); color: var(--text-secondary); background: var(--bg-elevated); border: 1px solid var(--border-subtle); border-radius: 999px; }
  .error-note { padding: var(--space-3); font-size: var(--font-size-sm); color: var(--danger); background: rgba(232, 90, 90, 0.08); border: 1px solid rgba(232, 90, 90, 0.25); border-radius: var(--radius-md); }
  .result__actions { display: flex; gap: var(--space-2); flex-wrap: wrap; }
  @media (max-width: 768px) { .result { padding: var(--space-3); } }
</style>