<script>
  import Input from '../../components/Input.svelte';
  import Button from '../../components/Button.svelte';
  import { getStrings } from '../../strings.svelte.js';
  import { MOSFETS } from './mosfetDatabase.js';

  const strings = $derived(getStrings());

  let iMax = $state('10');
  let vPack = $state('12');
  
  const iMaxNum = $derived(parseFloat(iMax.replace(',', '.')) || 0);
  const vPackNum = $derived(parseFloat(vPack.replace(',', '.')) || 0);
  
  const requiredCurrent = $derived(iMaxNum > 0 ? iMaxNum * 1.2 : 0);
  const requiredVoltage = $derived(vPackNum > 0 ? vPackNum * 1.2 : 0);
  
  const filteredMosfets = $derived(
    iMaxNum > 0 && vPackNum > 0
      ? MOSFETS
          .filter((m) => m.idCont >= requiredCurrent && m.vds >= requiredVoltage)
          .sort((a, b) => a.idCont - b.idCont)
      : []
  );

  function calcCount(mosfet) {
    if (iMaxNum <= 0) return 0;
    const safeCurrent = mosfet.idCont * 0.5;
    return Math.ceil(iMaxNum / safeCurrent);
  }

  function reset() {
    iMax = '10';
    vPack = '12';
  }
</script>

<div class="calc">
  <div class="fields">
    <Input bind:value={iMax} label={strings.bms.labels.maxCurrentField} placeholder="10" inputmode="decimal" />
    <Input bind:value={vPack} label={strings.bms.labels.packVoltage} placeholder="12" inputmode="decimal" />
  </div>

  {#if iMaxNum > 0 && vPackNum > 0}
    {#if filteredMosfets.length > 0}
      <div class="results-list">
        <div class="results-header">
          {strings.bms.labels.suitableFor} {Math.round(requiredCurrent)} {strings.bms.labels.suitableFor2} {Math.round(requiredVoltage)} {strings.bms.labels.suitableFor3}
        </div>
        {#each filteredMosfets as m (m.id)}
          {@const count = calcCount(m)}
          <div class="fet-card">
            <div class="fet-card__info">
              <div class="fet-card__name">{m.name}</div>
              <div class="fet-card__specs">
                <span>Vds: {m.vds}{strings.units.v}</span>
                <span>Id: {m.idCont}{strings.units.a}</span>
                <span>Rds: {m.rdsOn} {strings.units.mohm_short}</span>
              </div>
            </div>
            <div class="fet-card__result">
              <div class="fet-card__count">{count} {strings.common.pcs}</div>
              <div class="fet-card__hint">{strings.bms.labels.needPieces}</div>
            </div>
          </div>
        {/each}
      </div>
    {:else}
      <div class="error-note">
        {strings.bms.labels.noMosfets} {Math.round(requiredCurrent)} {strings.units.a} и {Math.round(requiredVoltage)} {strings.units.v}.
        {strings.bms.labels.noMosfets2}
      </div>
    {/if}
  {:else}
    <div class="empty-note">{strings.bms.labels.enterCurrentVoltage}</div>
  {/if}

  {#if iMaxNum > 0 && vPackNum > 0 && filteredMosfets.length > 0}
    <div class="result__actions">
      <Button variant="ghost" size="sm" onclick={reset}>{strings.buttons.reset}</Button>
    </div>
  {/if}
</div>

<style>
  .calc { display: flex; flex-direction: column; gap: var(--space-4); }
  .fields { display: flex; flex-direction: column; gap: var(--space-3); }
  
  .results-list { display: flex; flex-direction: column; gap: var(--space-2); }
  .results-header { font-size: var(--font-size-sm); color: var(--text-muted); margin-bottom: var(--space-1); }
  
  .fet-card {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: var(--space-3);
    background: var(--bg-elevated-2);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-md);
    transition: border-color var(--transition-fast);
  }
  .fet-card:hover { border-color: var(--border-strong); }
  
  .fet-card__info { flex: 1; }
  .fet-card__name { font-weight: 600; color: var(--accent); font-size: var(--font-size-base); }
  .fet-card__specs { display: flex; gap: var(--space-3); font-size: var(--font-size-sm); color: var(--text-secondary); margin-top: 4px; }
  
  .fet-card__result { text-align: right; }
  .fet-card__count { font-size: var(--font-size-xl); font-weight: 700; color: var(--accent); line-height: 1.1; }
  .fet-card__hint { font-size: var(--font-size-sm); color: var(--text-muted); }

  .error-note { padding: var(--space-3); color: var(--danger); background: rgba(232, 90, 90, 0.08); border: 1px solid rgba(232, 90, 90, 0.25); border-radius: var(--radius-md); font-size: var(--font-size-sm); }
  .empty-note { padding: var(--space-3); color: var(--text-muted); text-align: center; font-size: var(--font-size-sm); }
  .result__actions { display: flex; gap: var(--space-2); }
</style>