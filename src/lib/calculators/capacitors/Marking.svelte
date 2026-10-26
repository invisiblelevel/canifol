<script>
  import Input from '../../components/Input.svelte';
  import Button from '../../components/Button.svelte';
  import CapacitorVisual from '../../components/CapacitorVisual.svelte';
  import { getStrings } from '../../strings.svelte.js';
  import { parseCapacitorMarking, parseSmdCapacitor, parseCapacitorColors, CAPACITOR_COLORS, formatFarad, formatFaradCompact } from '../../utils/capacitorCodes.js';

  const strings = $derived(getStrings());

  let mode = $state('code');

  let copied = $state(false);
  let copyText = $state('');

  function makeCopyText(parsed) {
    if (!parsed) return '';
    const f = formatFarad(parsed.value);
    const parts = [`${f.value} ${strings.units[f.unit]}`];
    if (parsed.tolerance != null) parts.push(`±${parsed.tolerance}%`);
    if (parsed.voltage != null) parts.push(`${parsed.voltage} В`);
    return parts.join(' ');
  }

  async function copyResult() {
    if (!copyText) return;
    try {
      await navigator.clipboard.writeText(copyText);
      copied = true;
      setTimeout(() => (copied = false), 1200);
    } catch {}
  }

  let codeStr = $state('');
  let codeEl = $state(null);

  $effect(() => {
    if (mode === 'code') codeEl?.focus?.();
  });

  const codeNormalized = $derived(codeStr.trim());
  const codeParsed = $derived(codeNormalized ? parseCapacitorMarking(codeNormalized) : null);
  const codeError = $derived(!!codeNormalized && !codeParsed);

  $effect(() => {
    if (mode === 'code') copyText = makeCopyText(codeParsed);
  });

  let smdStr = $state('');
  let smdEl = $state(null);

  $effect(() => {
    if (mode === 'smd') smdEl?.focus?.();
  });

  const smdNormalized = $derived(smdStr.trim().toUpperCase());
  const smdParsed = $derived(smdNormalized ? parseSmdCapacitor(smdNormalized) : null);
  const smdError = $derived(!!smdNormalized && !smdParsed);

  $effect(() => {
    if (mode === 'smd') copyText = makeCopyText(smdParsed);
  });

  const COLOR_OPTIONS = $derived(
    CAPACITOR_COLORS.map((c) => ({ value: c.id, label: strings.colors[c.id] ?? c.name }))
  );

  let bandCount = $state(4);
  let c1 = $state('brown');
  let c2 = $state('black');
  let c3 = $state('orange');
  let tol = $state('white');
  let volt = $state('red');

  const activeColors = $derived.by(() => {
    const list = [c1, c2, c3];
    if (bandCount >= 4) list.push(tol);
    if (bandCount >= 5) list.push(volt);
    return list;
  });

  const colorParsed = $derived(parseCapacitorColors(activeColors));

  $effect(() => {
    if (mode === 'color') copyText = makeCopyText(colorParsed);
  });

  function reset() {
    if (mode === 'code') codeStr = '';
    else if (mode === 'smd') smdStr = '';
    else {
      bandCount = 4;
      c1 = 'brown';
      c2 = 'black';
      c3 = 'orange';
      tol = 'white';
      volt = 'red';
    }
  }
</script>

<div class="marking">
  <div class="modes no-select" role="group" aria-label="Тип маркировки">
    <button type="button" class="modes__item" class:active={mode === 'code'} onclick={() => (mode = 'code')}>
      {strings.markingTabs.code}
    </button>
    <button type="button" class="modes__item" class:active={mode === 'smd'} onclick={() => (mode = 'smd')}>
      {strings.markingTabs.smd}
    </button>
    <button type="button" class="modes__item" class:active={mode === 'color'} onclick={() => (mode = 'color')}>
      {strings.markingTabs.color}
    </button>
  </div>

  {#if mode === 'code'}
    <Input
      bind:value={codeStr}
      bind:this={codeEl}
      label={strings.capacitors.labels.codeLabel}
      placeholder={strings.capacitors.labels.codePlaceholder}
      hint={strings.capacitors.labels.codeHint}
      error={codeError ? strings.resistors.labels.smdError : ''}
      autocapitalize="characters"
      autocorrect="off"
      spellcheck={false}
      autocomplete="off"
    />

    {#if codeParsed}
      <div class="result" aria-live="polite">
        <div class="result__main">
          <div class="result__value">
            {formatFarad(codeParsed.value).value} {strings.units[formatFarad(codeParsed.value).unit]}
          </div>
          <div class="result__compact">{formatFaradCompact(codeParsed.value)}</div>
        </div>
        <div class="result__meta">
          {#if codeParsed.tolerance != null}
            <span class="badge">±{codeParsed.tolerance}%</span>
          {/if}
          {#if codeParsed.voltage != null}
            <span class="badge">{codeParsed.voltage} В</span>
          {/if}
        </div>
        <div class="result__actions">
          <Button variant="secondary" size="sm" onclick={copyResult}>
            {copied ? strings.buttons.copied : strings.buttons.copy}
          </Button>
          <Button variant="ghost" size="sm" onclick={reset}>{strings.buttons.clear}</Button>
        </div>
      </div>
    {/if}
  {:else if mode === 'smd'}
    <Input
      bind:value={smdStr}
      bind:this={smdEl}
      label={strings.resistors.labels.smd}
      placeholder={strings.capacitors.labels.smdCapPlaceholder}
      hint={strings.capacitors.labels.smdCapHint}
      error={smdError ? strings.resistors.labels.smdError : ''}
      autocapitalize="characters"
      autocorrect="off"
      spellcheck={false}
      autocomplete="off"
    />

    {#if smdParsed}
      <div class="result" aria-live="polite">
        <div class="result__main">
          <div class="result__value">
            {formatFarad(smdParsed.value).value} {strings.units[formatFarad(smdParsed.value).unit]}
          </div>
          <div class="result__compact">{formatFaradCompact(smdParsed.value)}</div>
        </div>
        <div class="result__meta">
          <span class="badge">{smdParsed.type === 'tant' ? strings.capacitors.labels.tant : strings.capacitors.labels.ceramic}</span>
        </div>
        {#if smdParsed.warning}
          <div class="warning-note">{smdParsed.warning}</div>
        {/if}
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
      {#each [3, 4, 5] as n}
        <button type="button" class="bands-switch__item" class:active={bandCount === n} onclick={() => (bandCount = n)}>
          {n}
        </button>
      {/each}
    </div>

    <div class="preview">
      <CapacitorVisual bands={activeColors} width={420} />
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
      {#if bandCount >= 5}
        <label class="sel">
          <span class="sel__label">{strings.capacitors.labels.voltageLabel}</span>
          <select bind:value={volt}>
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
            {formatFarad(colorParsed.value).value} {strings.units[formatFarad(colorParsed.value).unit]}
          </div>
          <div class="result__compact">{formatFaradCompact(colorParsed.value)}</div>
        </div>
        <div class="result__meta">
          {#if colorParsed.tolerance != null}
            <span class="badge">±{colorParsed.tolerance}%</span>
          {/if}
          {#if colorParsed.voltage != null}
            <span class="badge">{colorParsed.voltage} В</span>
          {/if}
        </div>
        {#if colorParsed.warning}
          <div class="warning-note">
            {strings.capacitors.labels[colorParsed.warning] || colorParsed.warning}
          </div>
        {/if}
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
  .warning-note { padding: var(--space-2) var(--space-3); font-size: var(--font-size-sm); color: var(--warning); background: rgba(247, 178, 74, 0.08); border: 1px solid rgba(247, 178, 74, 0.25); border-radius: var(--radius-md); }
  .error-note { padding: var(--space-3); font-size: var(--font-size-sm); color: var(--danger); background: rgba(242, 106, 106, 0.08); border: 1px solid rgba(242, 106, 106, 0.25); border-radius: var(--radius-md); }
  .result__actions { display: flex; gap: var(--space-2); flex-wrap: wrap; }
  @media (max-width: 768px) { .result { padding: var(--space-3); } }
</style>