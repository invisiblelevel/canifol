<script>
  import { invoke } from '@tauri-apps/api/core';
  import Input from '../../components/Input.svelte';
  import Button from '../../components/Button.svelte';
  import { getStrings } from '../../strings.svelte.js';

  const strings = $derived(getStrings());

  let query = $state('BC547');
  let trType = $state('bjt'); // 'mosfet' | 'bjt'
  let loading = $state(false);
  let error = $state('');
  let results = $state({ digikey: [], lcsc: [], soviet: [] });

  async function search() {
    if (!query.trim()) return;
    loading = true; error = ''; results = { digikey: [], lcsc: [], soviet: [] };

    try {
      const [digiRes, lcscRes, sovietRes] = await Promise.all([
        invoke('find_analogs_digikey', { query, trType }).catch(() => []),
        invoke('find_analogs_lcsc', { query, trType }).catch(() => []),
        invoke('search_soviet_analog', { query }).catch(() => [])
      ]);

      results = { digikey: digiRes, lcsc: lcscRes, soviet: sovietRes };
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  function reset() {
    query = trType === 'mosfet' ? 'IRFZ44N' : 'BC547';
    results = { digikey: [], lcsc: [], soviet: [] };
    error = '';
  }
</script>

<div class="analog-search">
  <div class="type-switch no-select">
    <button type="button" class="type-btn" class:active={trType === 'mosfet'} onclick={() => { trType = 'mosfet'; query = 'IRFZ44N'; results = { digikey: [], lcsc: [], soviet: [] }; }}>
      {strings.transistorTypes.mosfet}
    </button>
    <button type="button" class="type-btn" class:active={trType === 'bjt'} onclick={() => { trType = 'bjt'; query = 'BC547'; results = { digikey: [], lcsc: [], soviet: [] }; }}>
      {strings.transistorTypes.bjt}
    </button>
  </div>

  <div class="search-bar">
    <div class="search-input">
      <Input bind:value={query} label={strings.transistors.labels.model} placeholder={trType === 'mosfet' ? 'IRFZ44N' : 'BC547 или КТ315'} />
    </div>
    <Button variant="primary" onclick={search} loading={loading}>{strings.transistors.labels.searchAnalog}</Button>
  </div>

  {#if error}
    <div class="error-msg">{error}</div>
  {/if}

  {#if results.soviet.length > 0}
    <div class="source-block">
      <div class="source-header">{strings.transistors.labels.sovietAnalogs} ({results.soviet.length})</div>
      <div class="results-list">
        {#each results.soviet as item (item.name)}
          <div class="result-card">
            <div class="result-card__main">
              <div class="result-card__name">{item.name}</div>
            </div>
            <div class="result-card__specs">
              {#each item.analogs as a (a)}
                <span class="tag">{a}</span>
              {/each}
            </div>
          </div>
        {/each}
      </div>
    </div>
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
              {#if item.tr_type}<span class="tag">{item.tr_type}</span>{/if}
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
              {#if item.tr_type}<span class="tag">{item.tr_type}</span>{/if}
            </div>
          </div>
        {/each}
      </div>
    </div>
  {/if}

  {#if !loading && !error && query && results.digikey.length === 0 && results.lcsc.length === 0 && results.soviet.length === 0}
    <div class="empty">{strings.transistors.labels.nothingFound}</div>
  {:else if !loading && !error}
    <div class="empty">{strings.transistors.labels.enterModel}</div>
  {/if}

  <div class="actions">
    <Button variant="ghost" size="sm" onclick={reset}>{strings.buttons.reset}</Button>
  </div>
</div>

<style>
  .analog-search { display: flex; flex-direction: column; gap: var(--space-4); }
  .type-switch { display: flex; gap: var(--space-1); padding: 4px; background: var(--bg-elevated-2); border: 1px solid var(--border-subtle); border-radius: var(--radius-md); align-self: flex-start; }
  .type-btn { min-height: 36px; padding: 0 var(--space-4); border-radius: var(--radius-sm); font-size: var(--font-size-sm); font-weight: 600; color: var(--text-secondary); background: transparent; transition: background var(--transition-fast), color var(--transition-fast); }
  .type-btn:hover { color: var(--text-primary); }
  .type-btn.active { background: var(--accent-dim); color: var(--accent); }
  .search-bar { display: flex; gap: var(--space-3); align-items: flex-end; }
  .search-input { flex: 1; }
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
  .tag { padding: 1px 8px; background: var(--bg-elevated); border: 1px solid var(--border-subtle); border-radius: 999px; color: var(--accent); }
  .actions { display: flex; justify-content: flex-start; }
</style>