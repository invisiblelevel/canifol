<script>
  import Input from '../../components/Input.svelte';
  import Button from '../../components/Button.svelte';
  import ShuntVisual from '../../components/ShuntVisual.svelte';
  import { getStrings } from '../../strings.svelte.js';
  import { calcCurrentShunt } from '../../utils/bmsFormulas.js';

  const strings = $derived(getStrings());

  let iMax = $state('100');
  let iMin = $state('1');
  let vOs = $state('100');

  const iMaxNum = $derived(parseFloat(iMax.replace(',', '.')) || 0);
  const iMinNum = $derived(parseFloat(iMin.replace(',', '.')) || 0);
  const vOsNum = $derived(parseFloat(vOs.replace(',', '.')) || 0);

  const result = $derived(
    iMaxNum > 0 && iMinNum > 0 ? calcCurrentShunt(iMaxNum, iMinNum, vOsNum) : null
  );

  function reset() {
    iMax = '100'; iMin = '1'; vOs = '100';
  }
</script>

<div class="calc">
  <div class="fields">
    <Input bind:value={iMax} label={strings.bms.labels.maxCurrentLabel} placeholder="100" suffix={strings.units.a} inputmode="decimal" />
    <Input bind:value={iMin} label={strings.bms.labels.minCurrentLabel} placeholder="1" suffix={strings.units.a} inputmode="decimal" />
    <Input bind:value={vOs} label={strings.bms.labels.offsetVoltage} placeholder="100" suffix={strings.units.uv} inputmode="decimal" hint={strings.bms.labels.offsetHint} />
  </div>

  {#if result}
    <div class="visual">
      <ShuntVisual rShunt={result.rShunt} iMax={iMaxNum} powerW={result.powerW} />
    </div>

    <div class="result" aria-live="polite">
      <div class="result__main">
        <div class="result__value">{result.rShunt.toFixed(2)} {strings.units.mohm_short}</div>
        <div class="result__label">{strings.bms.labels.recommendedNominal}</div>
      </div>
      
      <div class="result__meta">
        <span class="badge">{strings.bms.labels.powerLabel}: {result.powerW.toFixed(2)} {strings.units.w}</span>
        <span class="badge">{strings.bms.labels.vSense}: {result.vSenseMaxMv.toFixed(1)} {strings.units.mv || 'мВ'}</span>
        <span class="badge" class:badge--warn={!result.isSensitive}>
          {strings.bms.labels.minCurrentBadge}: {result.minDetectableMa.toFixed(1)} {strings.units.ma}
        </span>
      </div>

      {#if !result.isSensitive}
        <div class="warning-note">
          {strings.bms.labels.notSensitive} {iMinNum} {strings.units.a}. {strings.bms.labels.notSensitiveHint}
        </div>
      {/if}

      <div class="result__actions">
        <Button variant="ghost" size="sm" onclick={reset}>{strings.buttons.reset}</Button>
      </div>
    </div>
  {:else}
    <div class="error-note">{strings.bms.labels.enterCorrectCurrent}</div>
  {/if}
</div>

<style>
  .calc { display: flex; flex-direction: column; gap: var(--space-4); }
  .fields { display: grid; grid-template-columns: repeat(auto-fit, minmax(160px, 1fr)); gap: var(--space-3); }
  .visual { display: flex; justify-content: center; background: var(--bg-elevated-2); border-radius: var(--radius-lg); border: 1px solid var(--border-subtle); padding: var(--space-2); }
  .result { display: flex; flex-direction: column; gap: var(--space-3); padding: var(--space-4); background: var(--bg-elevated-2); border: 1px solid var(--border-subtle); border-radius: var(--radius-lg); }
  .result__main { display: flex; align-items: baseline; gap: var(--space-3); }
  .result__value { font-size: clamp(1.5rem, 5vw, 2.25rem); font-weight: 700; color: var(--accent); line-height: 1.1; }
  .result__label { font-size: var(--font-size-sm); color: var(--text-muted); }
  .result__meta { display: flex; gap: var(--space-2); flex-wrap: wrap; }
  .badge { display: inline-flex; align-items: center; padding: 4px 10px; font-size: var(--font-size-sm); color: var(--text-secondary); background: var(--bg-elevated); border: 1px solid var(--border-subtle); border-radius: 999px; }
  .badge--warn { color: var(--warning); border-color: rgba(247, 178, 74, 0.35); }
  .warning-note { padding: var(--space-2) var(--space-3); font-size: var(--font-size-sm); color: var(--warning); background: rgba(247, 178, 74, 0.08); border: 1px solid rgba(247, 178, 74, 0.25); border-radius: var(--radius-md); }
  .error-note { padding: var(--space-3); color: var(--danger); background: rgba(232, 90, 90, 0.08); border: 1px solid rgba(232, 90, 90, 0.25); border-radius: var(--radius-md); font-size: var(--font-size-sm); }
  .result__actions { display: flex; gap: var(--space-2); }
  @media (max-width: 768px) { .result { padding: var(--space-3); } }
</style>