<script>
  import Input from '../../components/Input.svelte';
  import Button from '../../components/Button.svelte';
  import BatteryPackVisual from '../../components/BatteryPackVisual.svelte';
  import { getStrings } from '../../strings.svelte.js';
  import { calcBatteryConfig } from '../../utils/bmsFormulas.js';

  const strings = $derived(getStrings());

  let s = $state('3');
  let p = $state('4');
  let vCell = $state('3.7');
  let capCell = $state('3000');
  let cRate = $state('1');

  const sNum = $derived(parseInt(s) || 0);
  const pNum = $derived(parseInt(p) || 0);
  const vCellNum = $derived(parseFloat(vCell.replace(',', '.')) || 0);
  const capCellNum = $derived(parseFloat(capCell.replace(',', '.')) || 0);
  const cRateNum = $derived(parseFloat(cRate.replace(',', '.')) || 0);

  const result = $derived(
    sNum > 0 && pNum > 0 && vCellNum > 0 && capCellNum > 0
      ? calcBatteryConfig(sNum, pNum, vCellNum, capCellNum, cRateNum)
      : null
  );

  function reset() {
    s = '3'; p = '4'; vCell = '3.7'; capCell = '3000'; cRate = '1';
  }
</script>

<div class="calc">
  <div class="fields">
    <Input bind:value={s} label={strings.bms.labels.seriesBms} placeholder="3" inputmode="numeric" />
    <Input bind:value={p} label={strings.bms.labels.parallelCount} placeholder="4" inputmode="numeric" />
    <Input bind:value={vCell} label={strings.bms.labels.cellVoltage} placeholder="3.7" suffix={strings.units.v} inputmode="decimal" />
    <Input bind:value={capCell} label={strings.bms.labels.cellCapacity} placeholder="3000" suffix={strings.units.ma + '·' + strings.units.h} inputmode="decimal" />
    <Input bind:value={cRate} label={strings.bms.labels.cRate} placeholder="1" inputmode="decimal" hint={strings.bms.labels.cRateHint} />
  </div>

  {#if result}
    <div class="visual">
      <BatteryPackVisual s={sNum} p={pNum} vCell={vCellNum} capacityAh={result.capacityAh} />
    </div>

    <div class="result" aria-live="polite">
      <div class="result__row">
        <div class="result__label">{strings.bms.labels.voltageBms}</div>
        <div class="result__value">{result.voltage.toFixed(1)} {strings.units.v}</div>
      </div>
      <div class="result__row">
        <div class="result__label">{strings.bms.labels.capacity}</div>
        <div class="result__value">{result.capacityAh.toFixed(1)} {strings.units.ah}</div>
      </div>
      <div class="result__row">
        <div class="result__label">{strings.bms.labels.energy}</div>
        <div class="result__value">{result.energyWh.toFixed(0)} {strings.units.wh}</div>
      </div>
      <div class="result__row">
        <div class="result__label">{strings.bms.labels.maxCurrent} ({cRateNum}C)</div>
        <div class="result__value">{result.maxCurrent.toFixed(1)} {strings.units.a}</div>
      </div>

      <div class="result__actions">
        <Button variant="ghost" size="sm" onclick={reset}>{strings.buttons.reset}</Button>
      </div>
    </div>
  {:else}
    <div class="error-note">{strings.bms.labels.enterCorrectValues}</div>
  {/if}
</div>

<style>
  .calc { display: flex; flex-direction: column; gap: var(--space-4); }
  .fields { display: grid; grid-template-columns: repeat(auto-fit, minmax(150px, 1fr)); gap: var(--space-3); }
  .visual { display: flex; justify-content: center; background: var(--bg-elevated-2); border-radius: var(--radius-lg); border: 1px solid var(--border-subtle); padding: var(--space-2); }
  .result { display: flex; flex-direction: column; gap: var(--space-2); padding: var(--space-4); background: var(--bg-elevated-2); border: 1px solid var(--border-subtle); border-radius: var(--radius-lg); }
  .result__row { display: flex; justify-content: space-between; align-items: center; padding-bottom: var(--space-1); border-bottom: 1px solid var(--border-subtle); }
  .result__row:last-of-type { border-bottom: none; }
  .result__label { font-size: var(--font-size-base); color: var(--text-secondary); }
  .result__value { font-size: var(--font-size-lg); font-weight: 700; color: var(--accent); font-family: var(--font-mono); }
  .result__actions { display: flex; gap: var(--space-2); margin-top: var(--space-2); }
  .error-note { padding: var(--space-3); color: var(--danger); background: rgba(232, 90, 90, 0.08); border: 1px solid rgba(232, 90, 90, 0.25); border-radius: var(--radius-md); font-size: var(--font-size-sm); }
</style>