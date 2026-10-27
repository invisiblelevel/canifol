<script>
  import Input from '../../components/Input.svelte';
  import Button from '../../components/Button.svelte';
  import { getStrings } from '../../strings.svelte.js';
  import { parseSmd, formatOhm, formatOhmCompact } from '../../utils/resistorCodes.js';

  const strings = $derived(getStrings());

  let raw = $state('');
  let copied = $state(false);
  let inputEl = $state(null);

  $effect(() => { inputEl?.focus?.(); });

  const normalized = $derived(raw.trim().toUpperCase().replace(/\s+/g, ''));
  const parsed = $derived(normalized ? parseSmd(normalized) : null);
  const hasError = $derived(!!normalized && !parsed);

  const standardLabel = $derived(parsed ? {
    '3digit': strings.resistors.labels.standard3,
    '4digit': strings.resistors.labels.standard4,
    eia96: strings.resistors.labels.eia96
  }[parsed.standard] : '');

  const copyText = $derived(parsed ? formatOhm(parsed.value) : '');

  async function copyResult() {
    if (!copyText) return;
    try { await navigator.clipboard.writeText(copyText); copied = true; setTimeout(() => (copied = false), 1200); } catch {}
  }

  function clearAll() { raw = ''; inputEl?.focus?.(); }
</script>

<div class="smd">
  <Input
    bind:value={raw}
    bind:this={inputEl}
    label={strings.resistors.labels.smd}
    placeholder={strings.resistors.labels.smdPlaceholder}
    hint={strings.resistors.labels.smdHint}
    error={hasError ? strings.resistors.labels.smdError : ''}
    maxlength={4}
  />
  {#if parsed}
    <div class="result">
      <div class="result__main">
        <div class="result__value">{formatOhm(parsed.value)}</div>
        <div class="result__compact">{formatOhmCompact(parsed.value)}</div>
      </div>
      <div class="result__meta">
        <span class="badge">{standardLabel}</span>
        {#if parsed.tolerance != null}<span class="badge">±{parsed.tolerance}%</span>{/if}
      </div>
      <div class="result__actions">
        <Button variant="secondary" size="sm" onclick={copyResult}>{copied ? strings.buttons.copied : strings.buttons.copy}</Button>
        <Button variant="ghost" size="sm" onclick={clearAll}>{strings.buttons.clear}</Button>
      </div>
    </div>
  {/if}
</div>

<style>
  .smd { display: flex; flex-direction: column; gap: var(--space-4); }
  .result { display: flex; flex-direction: column; gap: var(--space-3); padding: var(--space-4); background: var(--bg-elevated-2); border: 1px solid var(--border-subtle); border-radius: var(--radius-lg); }
  .result__main { display: flex; align-items: baseline; gap: var(--space-3); flex-wrap: wrap; }
  .result__value { font-size: 2rem; font-weight: 700; color: var(--accent); }
  .result__compact { font-family: var(--font-mono); color: var(--text-muted); }
  .result__meta { display: flex; gap: var(--space-2); flex-wrap: wrap; }
  .badge { padding: 4px 10px; font-size: var(--font-size-sm); color: var(--text-secondary); background: var(--bg-elevated); border: 1px solid var(--border-subtle); border-radius: 999px; }
  .result__actions { display: flex; gap: var(--space-2); flex-wrap: wrap; }
</style>