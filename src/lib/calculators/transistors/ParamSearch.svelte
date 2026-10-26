<script>
  import { invoke } from '@tauri-apps/api/core';
  import Input from '../../components/Input.svelte';
  import Button from '../../components/Button.svelte';
  import { getStrings } from '../../strings.svelte.js';

  const strings = $derived(getStrings());

  let trType = $state('mosfet'); // 'mosfet' | 'bjt'
  let loading = $state(false);
  let error = $state('');
  let results = $state({ digikey: [], lcsc: [] });

  let vdsMin = $state('');
  let idMin = $state('');
  let rdsOnMax = $state('');

  let vceoMin = $state('');
  let icMin = $state('');
  let hfeMin = $state('');

  async function search() {
    loading = true; error = ''; results = { digikey: [], lcsc: [] };

    try {
      const params = trType === 'mosfet'
        ? {
            type: 'mosfet',
            vds: parseFloat(vdsMin.replace(',', '.')) || null,
            id: parseFloat(idMin.replace(',', '.')) || null,
            rdsOn: parseFloat(rdsOnMax.replace(',', '.')) || null
          }
        : {
            type: 'bjt',
            vceo: parseFloat(vceoMin.replace(',', '.')) || null,
            ic: parseFloat(icMin.replace(',', '.')) || null,
            hfe: parseFloat(hfeMin.replace(',', '.')) || null
          };

      // Убрали .catch(() => []), чтобы видеть реальные ошибки
      const digiRes = await invoke('find_by_params_digikey', { params }).catch(e => {
        console.error('DigiKey search failed:', e);
        throw e; // Пробрасываем ошибку дальше
      });

      const lcscRes = await invoke('find_by_params_lcsc', { params }).catch(e => {
        console.error('LCSC search failed:', e);
        throw e;
      });

      results = { digikey: digiRes, lcsc: lcscRes };
    } catch (e) {
      console.error('Search error:', e);
      error = String(e);
    } finally {
      loading = false;
    }
  }

  function reset() {
    vdsMin = ''; idMin = ''; rdsOnMax = '';
    vceoMin = ''; icMin = ''; hfeMin = '';
    results = { digikey: [], lcsc: [] };
    error = '';
  }
</script>

<div class="param-search">
  <div class="type-switch no-select">
    <button type="button" class="type-btn" class:active={trType === 'mosfet'} onclick={() => { trType = 'mosfet'; results = { digikey: [], lcsc: [] }; }}>
      {strings.transistorTypes.mosfet}
    </button>
    <button type="button" class="type-btn" class:active={trType === 'bjt'} onclick={() => { trType = 'bjt'; results = { digikey: [], lcsc: [] }; }}>
      {strings.transistorTypes.bjt}
    </button>
  </div>

  <div class="fields">
    {#if trType === 'mosfet'}
      <Input bind:value={vdsMin} label={strings.transistors.labels.vdsMin} placeholder="55" suffix={strings.units.v} inputmode="decimal" />
      <Input bind:value={idMin} label={strings.transistors.labels.idMin} placeholder="49" suffix={strings.units.a} inputmode="decimal" />
      <Input bind:value={rdsOnMax} label={strings.transistors.labels.rdsOnMax} placeholder="17.5" suffix={strings.units.mohm_short} inputmode="decimal" />
    {:else}
      <Input bind:value={vceoMin} label={strings.transistors.labels.vceoMin} placeholder="45" suffix={strings.units.v} inputmode="decimal" />
      <Input bind:value={icMin} label={strings.transistors.labels.icMin} placeholder="0.1" suffix={strings.units.a} inputmode="decimal" />
      <Input bind:value={hfeMin} label={strings.transistors.labels.hfeMin} placeholder="100" inputmode="decimal" />
    {/if}
  </div>

  <div class="actions-bar">
    <Button variant="primary" onclick={search} loading={loading}>{strings.transistors.labels.searchParam}</Button>
    <Button variant="ghost" size="sm" onclick={reset}>{strings.buttons.reset}</Button>
  </div>

  {#if error}
    <div class="error-msg">{error}</div>
  {/if}

  {#if results.digikey.length > 0}
    <div class="source-block">
      <div class="source-header">{strings.transistors.labels.digikeyAnalogs} ({results.digikey.length})</div>
      <div class="results-list">
        {#each results.digikey as item (item.name)}
          <div class="result-card">
            <div class="result-card__main">
              <div class="result-card__name">{item.name}</div>
              <div class="result-card__mfr">{item.manufacturer}</div>
            </div>
            <div class="result-card__specs">
              {#if trType === 'mosfet'}
                {#if item.vds != null}<span>Vds: {item.vds}{strings.units.v}</span>{/if}
                {#if item.id != null}<span>Id: {item.id}{strings.units.a}</span>{/if}
                {#if item.rds_on != null}<span>Rds: {item.rds_on} {strings.units.mohm_short}</span>{/if}
              {:else}
                {#if item.vceo != null}<span>Vceo: {item.vceo}{strings.units.v}</span>{/if}
                {#if item.ic != null}<span>Ic: {item.ic}{strings.units.a}</span>{/if}
                {#if item.hfe != null}<span>hFE: {item.hfe}</span>{/if}
              {/if}
            </div>
          </div>
        {/each}
      </div>
    </div>
  {/if}

  {#if results.lcsc.length > 0}
    <div class="source-block">
      <div class="source-header">{strings.transistors.labels.lcscAnalogs} ({results.lcsc.length})</div>
      <div class="results-list">
        {#each results.lcsc as item (item.name + item.source)}
          <div class="result-card">
            <div class="result-card__main">
              <div class="result-card__name">{item.name}</div>
              <div class="result-card__mfr">{item.manufacturer}</div>
            </div>
            <div class="result-card__specs">
              {#if trType === 'mosfet'}
                {#if item.vds != null}<span>Vds: {item.vds}{strings.units.v}</span>{/if}
                {#if item.id != null}<span>Id: {item.id}{strings.units.a}</span>{/if}
                {#if item.rds_on != null}<span>Rds: {item.rds_on} {strings.units.mohm_short}</span>{/if}
              {:else}
                {#if item.vceo != null}<span>Vceo: {item.vceo}{strings.units.v}</span>{/if}
                {#if item.ic != null}<span>Ic: {item.ic}{strings.units.a}</span>{/if}
                {#if item.hfe != null}<span>hFE: {item.hfe}</span>{/if}
              {/if}
            </div>
          </div>
        {/each}
      </div>
    </div>
  {/if}

  {#if !loading && !error && results.digikey.length === 0 && results.lcsc.length === 0}
    <div class="empty">{strings.transistors.labels.enterParams}</div>
  {/if}
</div>

<style>
  .param-search { display: flex; flex-direction: column; gap: var(--space-4); }
  .type-switch { display: flex; gap: var(--space-1); padding: 4px; background: var(--bg-elevated-2); border: 1px solid var(--border-subtle); border-radius: var(--radius-md); align-self: flex-start; }
  .type-btn { min-height: 36px; padding: 0 var(--space-4); border-radius: var(--radius-sm); font-size: var(--font-size-sm); font-weight: 600; color: var(--text-secondary); background: transparent; }
  .type-btn.active { background: var(--accent-dim); color: var(--accent); }
  .fields { display: grid; grid-template-columns: repeat(auto-fit, minmax(150px, 1fr)); gap: var(--space-3); }
  .actions-bar { display: flex; gap: var(--space-2); flex-wrap: wrap; }
  .error-msg { color: var(--danger); background: rgba(232, 90, 90, 0.1); padding: var(--space-3); border-radius: var(--radius-md); }
  .empty { color: var(--text-muted); text-align: center; padding: var(--space-5); }
  .source-block { display: flex; flex-direction: column; gap: var(--space-2); }
  .source-header { font-size: var(--font-size-sm); font-weight: 600; color: var(--accent); text-transform: uppercase; letter-spacing: 0.5px; padding-bottom: var(--space-1); border-bottom: 1px solid var(--border-subtle); }
  .results-list { display: flex; flex-direction: column; gap: var(--space-2); }
  .result-card { display: flex; flex-direction: column; gap: var(--space-1); background: var(--bg-elevated-2); padding: var(--space-3); border: 1px solid var(--border-subtle); border-radius: var(--radius-md); }
  .result-card__main { display: flex; justify-content: space-between; align-items: baseline; gap: var(--space-3); }
  .result-card__name { font-weight: 600; color: var(--accent); }
  .result-card__mfr { font-size: var(--font-size-sm); color: var(--text-secondary); }
  .result-card__specs { display: flex; flex-wrap: wrap; gap: var(--space-2); font-size: var(--font-size-sm); color: var(--text-secondary); }
</style>