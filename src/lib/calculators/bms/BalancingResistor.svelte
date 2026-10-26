<script>
  import Input from '../../components/Input.svelte';
  import Button from '../../components/Button.svelte';
  import BalancingVisual from '../../components/BalancingVisual.svelte';
  import { getStrings } from '../../strings.svelte.js';
  import { calcBalancingResistor } from '../../utils/bmsFormulas.js';

  const strings = $derived(getStrings());

  let vCell = $state('4.2');
  let iCb = $state('100');
  let rDsOn = $state('0');

  const vCellNum = $derived(parseFloat(vCell.replace(',', '.')) || 0);
  const iCbNum = $derived(parseFloat(iCb.replace(',', '.')) || 0);
  const rDsOnNum = $derived(parseFloat(rDsOn.replace(',', '.')) || 0);

  const result = $derived(
    vCellNum > 0 && iCbNum > 0 ? calcBalancingResistor(vCellNum, iCbNum, rDsOnNum) : null
  );

  function reset() {
    vCell = '4.2';
    iCb = '100';
    rDsOn = '0';
  }
</script>

<div class="calc">
  <div class="fields">
    <Input bind:value={vCell} label={strings.bms.labels.cellVoltage} placeholder="4.2" suffix={strings.units.v} inputmode="decimal" />
    <Input bind:value={iCb} label={strings.bms.labels.balancingCurrent} placeholder="100" suffix="мА" inputmode="decimal" />
    <Input bind:value={rDsOn} label={strings.bms.labels.rdsOn} placeholder="0" suffix={strings.units.ohm} inputmode="decimal" hint={strings.bms.labels.rdsOnHint} />
  </div>

  {#if result}
    <div class="visual">
      <BalancingVisual rCb={result.rCb} vCell={vCellNum} iCb={iCbNum} />
    </div>

    <div class="result" aria-live="polite">
      <div class="result__main">
        <div class="result__value">{result.rCb.toFixed(1)} {strings.units.ohm}</div>
        <div class="result__label">{strings.bms.labels.nominalRcb}</div>
      </div>
      <div class="result__meta">
        <span class="badge">{strings.bms.labels.powerAtLeast} {result.powerW.toFixed(2)} {strings.units.w}</span>
        <span class="badge">{strings.bms.labels.worstCurrent}: {Math.round(result.iCbWorst * 1000)} мА</span>
      </div>
      <div class="result__actions">
        <Button variant="ghost" size="sm" onclick={reset}>{strings.buttons.reset}</Button>
      </div>
    </div>
  {:else if vCellNum > 0 && iCbNum > 0}
    <div class="error-note">{strings.bms.labels.calcFailed}</div>
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
  .error-note { padding: var(--space-3); color: var(--danger); background: rgba(232, 90, 90, 0.08); border: 1px solid rgba(232, 90, 90, 0.25); border-radius: var(--radius-md); font-size: var(--font-size-sm); }
  .result__actions { display: flex; gap: var(--space-2); }
  @media (max-width: 768px) { .result { padding: var(--space-3); } }
</style>