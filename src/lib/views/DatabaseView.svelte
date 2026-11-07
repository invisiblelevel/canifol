<script>
  import { invoke } from '@tauri-apps/api/core';
  import Card from '../components/Card.svelte';
  import Button from '../components/Button.svelte';
  import { getStrings } from '../strings.svelte.js';
  import BoardViewer from '../components/BoardViewer.svelte';
  import { PdfViewer, PdfRenderer } from 'svelte-pdf-view';
  import PdfCustomToolbar from '../components/PdfCustomToolbar.svelte';
  import { ArrowLeft, Database as DatabaseIcon, Search } from '@lucide/svelte';
  import EmptyState from '../components/EmptyState.svelte';

  const strings = $derived(getStrings());
  const dbStrings = $derived(strings.database);
  
  const CATALOG_FILE_ID = "10--ZxmNatSAOxNrQaidWYp_zyJubApaY";
  const CATALOG_MEGA_URL = "https://mega.nz/file/cyN1DIrJ#RuAXgTNRZlqrvM0I_UinIfcEAHlFl8DBOPWAHCE62EE";

  let catalog = $state(null);
  let loading = $state(false);
  let error = $state('');
  
  let activeCategory = $state('');
  let activeModel = $state(null);
  let searchQuery = $state('');
  
  let currentBoard = $state(null);
  let currentPdf = $state(null);
  let viewerKey = $state(0);
  
  let loadingFileName = $state('');
  let fileLoading = $state(false);

  const filteredCatalog = $derived.by(() => {
    if (!catalog) return {};
    
    const query = searchQuery.trim().toLowerCase();
    if (!query) {
      if (activeCategory && catalog[activeCategory]) {
        return { [activeCategory]: catalog[activeCategory] };
      }
      return catalog;
    }

    const result = {};
    for (const [cat, models] of Object.entries(catalog)) {
      const matchedModels = {};
      for (const [model, files] of Object.entries(models)) {
        const modelLower = model.toLowerCase();
        const catLower = cat.toLowerCase();

        let matches = modelLower.includes(query) || catLower.includes(query);

        if (!matches) {
          matches = files.some(f => f.name.toLowerCase().includes(query));
        }

        if (matches) {
          matchedModels[model] = files;
        }
      }
      if (Object.keys(matchedModels).length > 0) {
        result[cat] = matchedModels;
      }
    }
    return result;
  });

  async function loadCatalog() {
    loading = true;
    error = '';
    try {
      await invoke('delete_cached_file', { filename: 'catalog.json' }).catch(() => {});
      
      let localPath;
      try {
        localPath = await invoke('download_gdrive_file', { 
          fileId: CATALOG_FILE_ID, 
          filename: 'catalog.json' 
        });
      } catch (gdriveError) {
        console.warn('GDrive failed, trying MEGA...', gdriveError);
        localPath = await invoke('download_mega_file', {
          url: CATALOG_MEGA_URL,
          filename: 'catalog.json'
        });
      }

      const content = await invoke('read_text_file', { path: localPath });
      catalog = JSON.parse(content);
      
      const cats = Object.keys(catalog);
      if (cats.length > 0) {
        activeCategory = cats[0];
      }
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  async function openFile(file) {
    fileLoading = true;
    loadingFileName = file.name;
    error = '';
    currentBoard = null;
    currentPdf = null;
    
    try {
      const localPath = await invoke('download_gdrive_file', { 
        fileId: file.id, 
        filename: file.name 
      });
      
      if (file.type === 'pdf') {
        const bytesArray = await invoke('read_binary_file', { path: localPath });
        const blob = new Blob([new Uint8Array(bytesArray)], { type: 'application/pdf' });
        currentPdf = { ...file, blobUrl: URL.createObjectURL(blob) };
        viewerKey++;
      } else {
        currentBoard = await invoke('read_boardview', { path: localPath });
      }
    } catch (e) {
      error = String(e);
    } finally {
      fileLoading = false;
      loadingFileName = '';
    }
  }

  function closeViewer() {
    currentBoard = null;
    currentPdf = null;
  }

  function closeModal() {
    activeModel = null;
    closeViewer();
  }

  function highlight(text, query) {
    if (!query.trim()) return text;
    const regex = new RegExp(`(${query.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')})`, 'gi');
    return text.replace(regex, '<mark>$1</mark>');
  }
</script>

{#if currentPdf}
  <Card title={currentPdf.name}>
    <div class="pdf-header">
      <Button variant="ghost" size="sm" onclick={closeViewer}>
        <ArrowLeft size={16} /> {dbStrings.backToList}
      </Button>
    </div>
    <div class="pdf-main-container">
      <div class="pdf-toolbar-wrapper">
        <PdfViewer src={currentPdf.blobUrl} scale={1.0}>
          <PdfCustomToolbar />
          {#key viewerKey}
            <PdfRenderer
              backgroundColor="#181a1f"
              scrollbarThumbColor="#f5b83d"
              scrollbarTrackColor="#2a2f39"
              scrollbarThumbHoverColor="#e6a420"
              scrollbarWidth="12px"
              pageShadow="0 4px 12px rgba(0,0,0,0.5)"
            />
          {/key}
        </PdfViewer>
      </div>
    </div>
  </Card>
{:else if currentBoard}
  <Card title={activeModel}>
    <div class="pdf-header">
      <Button variant="ghost" size="sm" onclick={closeViewer}>
        <ArrowLeft size={16} /> {dbStrings.backToList}
      </Button>
    </div>
    <BoardViewer data={currentBoard} />
  </Card>
{:else}
  <Card title={dbStrings.title}>
    {#if !catalog && !loading}
      <EmptyState 
        icon={DatabaseIcon}
        title={dbStrings.emptyTitle}
        description={dbStrings.emptyDesc}
      >
        <Button onclick={loadCatalog}>
          {dbStrings.loadBtn}
        </Button>
      </EmptyState>
    {:else if loading}
      <div class="loading-center">
        <div class="spinner"></div>
        <span>{dbStrings.loading}</span>
      </div>
    {:else if error}
      <div class="error-box">{error}</div>
    {:else if catalog}
      <div class="controls">
        <div class="search-wrapper">
          <Search size={18} class="search-icon" />
          <input 
            type="text" 
            bind:value={searchQuery}
            placeholder={dbStrings.searchPlaceholder}
            class="search-input"
          />
        </div>
        <Button variant="ghost" size="sm" onclick={loadCatalog} loading={loading}>
          {dbStrings.refresh}
        </Button>
      </div>

      {#if !searchQuery.trim() && activeCategory && catalog[activeCategory]}
        <div class="categories-tabs">
          {#each Object.keys(catalog) as cat}
            <button 
              class="tab-btn" 
              class:active={activeCategory === cat}
              onclick={() => { activeCategory = cat; activeModel = null; }}
            >
              {cat}
            </button>
          {/each}
        </div>
      {/if}

      {#if Object.keys(filteredCatalog).length > 0}
        {#each Object.entries(filteredCatalog) as [catName, models]}
          {#if searchQuery.trim()}
            <h4 class="category-header">{catName}</h4>
          {/if}
          <div class="model-list">
            {#each Object.entries(models) as [modelName, files]}
              <button class="model-item" onclick={() => { activeModel = modelName; activeCategory = catName; }}>
                <span>{@html highlight(modelName, searchQuery)}</span>
                <span class="badge">{files.length}</span>
              </button>
            {/each}
          </div>
        {/each}
      {:else}
        <div class="placeholder">{dbStrings.nothingFound}</div>
      {/if}
    {/if}
  </Card>
{/if}

{#if activeModel && catalog && activeCategory && !currentBoard && !currentPdf}
  <div class="modal-overlay" onclick={closeModal}>
    <div class="modal-content" onclick={(e) => e.stopPropagation()}>
      <h3>{activeModel}</h3>
      
      {#if fileLoading}
        <div class="loading-indicator">
          <div class="spinner"></div>
          <span>{dbStrings.downloading}: {loadingFileName}...</span>
        </div>
      {/if}

      <div class="file-list">
        {#each catalog[activeCategory][activeModel] as file}
          <button 
            class="file-btn" 
            class:file-highlighted={searchQuery.trim() && file.name.toLowerCase().includes(searchQuery.trim().toLowerCase())}
            onclick={() => openFile(file)} 
            disabled={fileLoading}
          >
            {file.type === 'pdf' ? '📄' : '🗺️'} {@html highlight(file.name, searchQuery)}
          </button>
        {/each}
      </div>
      
      <Button variant="ghost" onclick={closeModal}>{dbStrings.close}</Button>
    </div>
  </div>
{/if}

<style>
  /* ... стили без изменений ... */
  .controls { margin-bottom: 16px; display: flex; gap: 12px; align-items: center; justify-content: space-between; }
  .search-wrapper { position: relative; flex: 1; max-width: 400px; }
  .search-icon { position: absolute; left: 12px; top: 50%; transform: translateY(-50%); color: var(--text-muted); pointer-events: none; }
  .search-input {
    width: 100%;
    padding: 10px 12px 10px 40px;
    background: var(--bg-base);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-md);
    color: var(--text-primary);
    font-size: var(--font-size-sm);
    transition: border-color var(--transition-fast);
  }
  .search-input:focus { outline: none; border-color: var(--accent); }

  .categories-tabs { display: flex; gap: 8px; margin-bottom: 16px; border-bottom: 1px solid var(--border-subtle); padding-bottom: 8px; overflow-x: auto; }
  .tab-btn { padding: 6px 12px; border-radius: var(--radius-sm); color: var(--text-secondary); white-space: nowrap; }
  .tab-btn.active { background: var(--accent-dim); color: var(--accent); }
  
  .category-header { font-size: 14px; font-weight: 600; color: var(--accent); margin: 16px 0 8px 0; text-transform: uppercase; letter-spacing: 0.05em; }
  
  .model-list { display: flex; flex-direction: column; gap: 4px; margin-bottom: 16px; }
  .model-item { display: flex; justify-content: space-between; padding: 12px; background: var(--bg-elevated-2); border-radius: var(--radius-md); color: var(--text-primary); text-align: left; }
  .model-item:hover { background: var(--bg-elevated-3); }
  .badge { font-size: 12px; color: var(--text-muted); }
  
  .placeholder { text-align: center; color: var(--text-muted); padding: 40px; }
  .error-box { background: rgba(255,0,0,0.1); color: red; padding: 12px; border-radius: var(--radius-md); margin-bottom: 16px; }
  
  .loading-center { display: flex; flex-direction: column; align-items: center; gap: 12px; padding: 40px; color: var(--text-muted); }

  .modal-overlay { position: fixed; inset: 0; background: rgba(0,0,0,0.7); display: flex; align-items: center; justify-content: center; z-index: 1000; }
  .modal-content { background: var(--bg-elevated); padding: 24px; border-radius: var(--radius-lg); width: 90%; max-width: 600px; max-height: 80vh; overflow-y: auto; display: flex; flex-direction: column; gap: 16px; }
  .file-list { display: flex; flex-direction: column; gap: 8px; }
  .file-btn { text-align: left; padding: 10px; background: var(--bg-base); border: 1px solid var(--border-subtle); border-radius: var(--radius-sm); color: var(--text-primary); }
  .file-btn:hover { border-color: var(--accent); }
  .file-btn.file-highlighted { border-color: var(--accent); background: var(--accent-dim); }

  .loading-indicator { display: flex; align-items: center; gap: 12px; padding: 12px; background: var(--bg-base); border: 1px solid var(--border-subtle); border-radius: var(--radius-md); color: var(--accent); font-size: 14px; }
  
  .spinner { width: 16px; height: 16px; border: 2px solid rgba(245, 184, 61, 0.3); border-top-color: var(--accent); border-radius: 50%; animation: spin 0.8s linear infinite; flex-shrink: 0; }
  @keyframes spin { to { transform: rotate(360deg); } }

  .pdf-header { display: flex; justify-content: space-between; margin-bottom: var(--space-3); }
  .pdf-main-container { display: flex; flex-direction: column; height: 70vh; border: 1px solid var(--border-subtle); border-radius: var(--radius-md); overflow: hidden; }
  .pdf-toolbar-wrapper { display: flex; flex-direction: column; height: 100%; width: 100%; }
  :global(.pdf-toolbar) { flex-shrink: 0; background: var(--bg-elevated-2) !important; border-bottom: 1px solid var(--border-subtle) !important; }
  :global(.pdf-toolbar button) { background: var(--bg-elevated) !important; color: var(--text-primary) !important; border: 1px solid var(--border-strong) !important; border-radius: var(--radius-sm) !important; }
  :global(.pdf-toolbar button:hover) { background: var(--bg-elevated-3) !important; }
  :global(.pdf-toolbar input) { background: var(--bg-base) !important; color: var(--text-primary) !important; border: 1px solid var(--border-strong) !important; border-radius: var(--radius-sm) !important; }
  :global(.pdf-toolbar button[title="Download"]),
  :global(.pdf-toolbar button[aria-label="Download"]),
  :global(.pdf-toolbar .pdf-toolbar__download) { display: none !important; }

  :global(mark) {
    background: var(--accent);
    color: #000;
    padding: 0 2px;
    border-radius: 2px;
  }
</style>