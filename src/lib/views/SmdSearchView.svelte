<script>
  import Card from '../components/Card.svelte';
  import Input from '../components/Input.svelte';
  import { getStrings } from '../strings.svelte.js';

  const strings = $derived(getStrings());

  let query = $state('');
  let index = $state(null);
  let data = $state(null);
  let loadingIndex = $state(false);
  let loadingData = $state(false);
  let indexError = $state('');
  let dataError = $state('');
  let results = $state([]);
  let searched = $state(false);

  async function loadIndex() {
    loadingIndex = true;
    indexError = '';
    try {
      const res = await fetch('/smd/smdIndex.json');
      if (!res.ok) throw new Error(`HTTP ${res.status}`);
      index = await res.json();
    } catch (e) {
      indexError = String(e);
    } finally {
      loadingIndex = false;
    }
  }

  async function loadData() {
    if (data) return;
    loadingData = true;
    dataError = '';
    try {
      const res = await fetch('/smd/smdData.json');
      if (!res.ok) throw new Error(`HTTP ${res.status}`);
      data = await res.json();
    } catch (e) {
      dataError = String(e);
    } finally {
      loadingData = false;
    }
  }

  async function search() {
    const q = query.trim().toUpperCase();
    if (!q) {
      results = [];
      searched = false;
      return;
    }
    if (!index) {
      await loadIndex();
    }
    if (!index) return;

    const idxs = index[q];
    if (!idxs || idxs.length === 0) {
      results = [];
      searched = true;
      return;
    }

    if (!data) {
      await loadData();
    }
    if (!data) return;

    results = idxs.map(i => data[i]).filter(Boolean);
    searched = true;
  }

  $effect(() => {
    loadIndex();
  });
</script>

<Card title={strings.smd.title}>
  <div class="smd-search">
    {#if loadingIndex}
      <div class="loading-bar">
        <div class="loading-bar__text">{strings.smd.loadingIndex}</div>
        <div class="loading-bar__track"><div class="loading-bar__indeterminate"></div></div>
      </div>
    {:else if indexError}
      <div class="error-note">{strings.smd.indexError}: {indexError}</div>
    {:else if index}
      <div class="search-row">
        <div class="search-input">
          <Input
            bind:value={query}
            label={strings.smd.searchLabel}
            placeholder={strings.smd.searchPlaceholder}
            onkeydown={(e) => { if (e.key === 'Enter') search(); }}
          />
        </div>
        <button class="search-btn" onclick={search} disabled={loadingData}>
          {loadingData ? strings.smd.loadingData : strings.smd.searchBtn}
        </button>
      </div>

      {#if loadingData}
        <div class="loading-bar">
          <div class="loading-bar__text">{strings.smd.loadingData}</div>
          <div class="loading-bar__track"><div class="loading-bar__indeterminate"></div></div>
        </div>
      {/if}

      {#if dataError}
        <div class="error-note">{strings.smd.dataError}: {dataError}</div>
      {/if}

      {#if searched && results.length === 0}
        <div class="empty">{strings.smd.nothingFound}</div>
      {:else if results.length > 0}
        <div class="results-header">{strings.smd.resultsFound}: {results.length}</div>
        <div class="results-table-wrap">
          <table class="results-table">
            <thead>
              <tr>
                <th>{strings.smd.code}</th>
                <th>{strings.smd.model}</th>
                <th>{strings.smd.type}</th>
                <th>{strings.smd.package}</th>
                <th>{strings.smd.params}</th>
              </tr>
            </thead>
            <tbody>
              {#each results as r, i (i)}
                <tr>
                  <td class="td-code">{r.code}</td>
                  <td class="td-model">{r.model}</td>
                  <td>{r.type}</td>
                  <td>{r.package}</td>
                  <td class="td-params">{r.params}</td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      {/if}
    {/if}
  </div>
</Card>

<style>
  .smd-search { display: flex; flex-direction: column; gap: var(--space-3); }

  .search-row { display: flex; gap: var(--space-2); align-items: flex-end; }
  .search-input { flex: 1; }
  .search-btn {
    padding: 0 var(--space-4);
    height: 44px;
    border-radius: var(--radius-md);
    background: var(--accent);
    color: var(--bg-base);
    font-weight: 700;
    font-size: var(--font-size-base);
    transition: filter var(--transition-fast);
  }
  .search-btn:hover:not(:disabled) { filter: brightness(1.1); }
  .search-btn:disabled { opacity: 0.6; cursor: not-allowed; }

  .loading-bar { display: flex; flex-direction: column; gap: 6px; padding: var(--space-3); background: var(--bg-elevated-2); border: 1px solid var(--border-subtle); border-radius: var(--radius-md); }
  .loading-bar__text { font-size: var(--font-size-sm); color: var(--text-secondary); }
  .loading-bar__track { position: relative; height: 6px; background: var(--bg-base); border-radius: 3px; overflow: hidden; }
  .loading-bar__indeterminate {
    position: absolute;
    top: 0; left: 0;
    width: 40%;
    height: 100%;
    background: var(--accent);
    border-radius: 3px;
    animation: loadSlide 1.2s ease-in-out infinite;
  }
  @keyframes loadSlide {
    0% { left: -40%; }
    100% { left: 100%; }
  }

  .error-note { padding: var(--space-3); color: var(--danger); background: rgba(232, 90, 90, 0.08); border: 1px solid rgba(232, 90, 90, 0.25); border-radius: var(--radius-md); font-size: var(--font-size-sm); }
  .empty { color: var(--text-muted); text-align: center; padding: var(--space-5); }

  .results-header { font-size: var(--font-size-sm); font-weight: 600; color: var(--accent); }
  .results-table-wrap { overflow-x: auto; max-height: 60vh; overflow-y: auto; border: 1px solid var(--border-subtle); border-radius: var(--radius-md); }
  .results-table { width: 100%; border-collapse: collapse; font-size: var(--font-size-sm); }
  .results-table th { text-align: left; padding: var(--space-2) var(--space-3); color: var(--text-muted); font-weight: 600; border-bottom: 2px solid var(--border-strong); position: sticky; top: 0; background: var(--bg-elevated); z-index: 1; }
  .results-table td { padding: var(--space-2) var(--space-3); border-bottom: 1px solid var(--border-subtle); color: var(--text-secondary); vertical-align: top; }
  .td-code { font-family: var(--font-mono); color: var(--text-primary) !important; font-weight: 600; }
  .td-model { color: var(--text-primary) !important; }
  .td-params { font-family: var(--font-mono); font-size: 12px; color: var(--text-muted); }
  @media (max-width: 768px) {
    .search-row { flex-direction: column; align-items: stretch; }
    .search-btn { height: 44px; }
  }
</style>