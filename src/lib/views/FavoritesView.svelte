<script>
  import Card from '../components/Card.svelte';
  import Button from '../components/Button.svelte';
  import { getStrings } from '../strings.svelte.js';
  import { favoritesStore } from '../stores/favorites.svelte.js';
  import { navStore } from '../stores/navigation.svelte.js';

  const strings = $derived(getStrings());
  const F = $derived(strings.favorites);

  let activeTab = $state('all');

  const tabs = $derived([
    { id: 'all', label: F.tabs.all },
    { id: 'smd', label: F.tabs.smd },
    { id: 'pinouts', label: F.tabs.pinouts },
    { id: 'datasheets', label: F.tabs.datasheets }
  ]);

  function openPinout(item) {
    navStore.setPayload({ pinoutTab: item.tab, pinoutId: item.id });
    navStore.forcePinoutReload();
    navStore.current = 'pinouts';
  }

  function openDatasheet(item) {
    navStore.setPayload({ datasheetItem: item });
    navStore.current = 'datasheets';
  }
</script>

<Card title={F.title}>
  <nav class="fav-tabs no-select">
    {#each tabs as tab (tab.id)}
      <button type="button" class="fav-tab" class:active={activeTab === tab.id} onclick={() => activeTab = tab.id}>
        {tab.label}
      </button>
    {/each}
  </nav>

  <div class="fav-content">
    {#if activeTab === 'all' || activeTab === 'smd'}
      {#if favoritesStore.smd.length > 0}
        <section class="fav-section">
          <h3 class="fav-section__title">{F.sections.smd} ({favoritesStore.smd.length})</h3>
          <ul class="fav-list smd-list">
            {#each favoritesStore.smd as item (`${item.code}_${item.model}_${item.package}`)}
              <li class="fav-item smd-item">
                <div class="smd-grid">
                  <div class="smd-cell">
                    <span class="smd-label">{strings.smd.code}</span>
                    <span class="smd-value code-val">{item.code}</span>
                  </div>
                  <div class="smd-cell">
                    <span class="smd-label">{strings.smd.model}</span>
                    <span class="smd-value">{item.model}</span>
                  </div>
                  <div class="smd-cell">
                    <span class="smd-label">{strings.smd.type}</span>
                    <span class="smd-value">{item.type}</span>
                  </div>
                  <div class="smd-cell">
                    <span class="smd-label">{strings.smd.package}</span>
                    <span class="smd-value">{item.package}</span>
                  </div>
                  <div class="smd-cell full-width">
                    <span class="smd-label">{strings.smd.params}</span>
                    <span class="smd-value params-val">{item.params || '—'}</span>
                  </div>
                </div>
                <div class="fav-item__actions">
                  <Button variant="ghost" size="sm" onclick={() => favoritesStore.removeSmd(item)}>✕</Button>
                </div>
              </li>
            {/each}
          </ul>
        </section>
      {:else if activeTab === 'smd'}
        <div class="fav-empty">{F.empty}</div>
      {/if}
    {/if}

    {#if activeTab === 'all' || activeTab === 'pinouts'}
      {#if favoritesStore.pinouts.length > 0}
        <section class="fav-section">
          <h3 class="fav-section__title">{F.sections.pinouts} ({favoritesStore.pinouts.length})</h3>
          <ul class="fav-list">
            {#each favoritesStore.pinouts as item (item.id)}
              <li class="fav-item">
                <div class="fav-item__main">
                  <span class="fav-item__code">{item.name || item.id}</span>
                  <span class="fav-item__meta">{item.tab}</span>
                </div>
                <div class="fav-item__actions">
                  <Button variant="secondary" size="sm" onclick={() => openPinout(item)}>{F.open}</Button>
                  <Button variant="ghost" size="sm" onclick={() => favoritesStore.removePinout(item.id)}>✕</Button>
                </div>
              </li>
            {/each}
          </ul>
        </section>
      {:else if activeTab === 'pinouts'}
        <div class="fav-empty">{F.empty}</div>
      {/if}
    {/if}

    {#if activeTab === 'all' || activeTab === 'datasheets'}
      {#if favoritesStore.datasheets.length > 0}
        <section class="fav-section">
          <h3 class="fav-section__title">{F.sections.datasheets} ({favoritesStore.datasheets.length})</h3>
          <ul class="fav-list">
            {#each favoritesStore.datasheets as item (`${item.name}_${item.manufacturer}_${item.datasheet_url}`)}
              <li class="fav-item">
                <div class="fav-item__main">
                  <span class="fav-item__code">{item.name}</span>
                  <span class="fav-item__meta">
                    {item.manufacturer} · {item.source}
                    {#if item.offline}
                      <span class="offline-badge">{F.offlineBadge}</span>
                    {/if}
                  </span>
                </div>
                <div class="fav-item__actions">
                  <Button variant="secondary" size="sm" onclick={() => openDatasheet(item)}>{F.open}</Button>
                  <Button variant="ghost" size="sm" onclick={() => favoritesStore.removeDatasheet(item)}>✕</Button>
                </div>
              </li>
            {/each}
          </ul>
        </section>
      {:else if activeTab === 'datasheets'}
        <div class="fav-empty">{F.empty}</div>
      {/if}
    {/if}

    {#if activeTab === 'all' && favoritesStore.smd.length === 0 && favoritesStore.pinouts.length === 0 && favoritesStore.datasheets.length === 0}
      <div class="fav-empty">{F.empty}</div>
    {/if}
  </div>
</Card>

<style>
  .fav-tabs { display: flex; gap: var(--space-1); padding-bottom: var(--space-3); margin-bottom: var(--space-4); border-bottom: 1px solid var(--border-subtle); overflow-x: auto; }
  .fav-tab { min-height: 36px; padding: 0 var(--space-3); border-radius: var(--radius-md); font-size: var(--font-size-sm); color: var(--text-secondary); background: transparent; white-space: nowrap; transition: background var(--transition-fast), color var(--transition-fast); }
  .fav-tab:hover { background: var(--bg-elevated-2); color: var(--text-primary); }
  .fav-tab.active { background: var(--accent-dim); color: var(--accent); }
  
  .fav-content { display: flex; flex-direction: column; gap: var(--space-4); }
  .fav-section { display: flex; flex-direction: column; gap: var(--space-2); }
  .fav-section__title { font-size: var(--font-size-base); font-weight: 600; color: var(--accent); margin-bottom: var(--space-1); }
  
  .fav-list { list-style: none; padding: 0; display: flex; flex-direction: column; gap: var(--space-2); }
  
  /* Стили для обычных карточек (Распиновки, Даташиты) */
  .fav-item { display: flex; justify-content: space-between; align-items: center; gap: var(--space-3); background: var(--bg-elevated-2); padding: var(--space-3); border-radius: var(--radius-md); border: 1px solid var(--border-subtle); }
  .fav-item__main { flex: 1; display: flex; flex-direction: column; gap: 2px; overflow: hidden; }
  .fav-item__code { font-weight: 600; color: var(--text-primary); font-family: var(--font-mono); }
  .fav-item__meta { font-size: var(--font-size-sm); color: var(--text-muted); display: flex; align-items: center; gap: var(--space-2); flex-wrap: wrap; }
  .fav-item__actions { display: flex; gap: var(--space-1); flex-shrink: 0; }

  /* Стили для SMD-карточек (сетка) */
  .smd-item { align-items: flex-start; }
  .smd-grid { flex: 1; display: grid; grid-template-columns: repeat(auto-fit, minmax(100px, 1fr)); gap: var(--space-2); width: 100%; }
  .smd-cell { display: flex; flex-direction: column; gap: 2px; }
  .smd-cell.full-width { grid-column: 1 / -1; margin-top: var(--space-1); border-top: 1px dashed var(--border-subtle); padding-top: var(--space-2); }
  .smd-label { font-size: 10px; text-transform: uppercase; color: var(--text-muted); letter-spacing: 0.05em; font-weight: 600; }
  .smd-value { font-size: var(--font-size-sm); color: var(--text-primary); font-family: var(--font-mono); }
  .code-val { font-weight: 700; color: var(--accent); }
  .params-val { font-size: 12px; color: var(--text-secondary); word-break: break-word; }

  .offline-badge {
    display: inline-block;
    padding: 1px 6px;
    font-size: 10px;
    font-weight: 700;
    letter-spacing: 0.04em;
    text-transform: uppercase;
    color: var(--accent);
    background: var(--accent-dim);
    border-radius: var(--radius-sm);
  }
  
  .fav-empty { color: var(--text-muted); text-align: center; padding: var(--space-5); font-style: italic; }

  @media (max-width: 768px) {
    .smd-grid { grid-template-columns: 1fr 1fr; }
  }
</style>