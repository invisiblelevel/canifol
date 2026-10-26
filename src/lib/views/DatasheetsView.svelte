<script>
  import { invoke } from '@tauri-apps/api/core';
  import { save } from '@tauri-apps/plugin-dialog';
  import { writeFile } from '@tauri-apps/plugin-fs';
  import Input from '../components/Input.svelte';
  import Button from '../components/Button.svelte';
  import Card from '../components/Card.svelte';
  import HelpButton from '../components/HelpButton.svelte';
  import SectionHelpModal from '../components/SectionHelpModal.svelte';
  import { ArrowLeft } from '@lucide/svelte';
  import { PdfViewer, PdfRenderer } from 'svelte-pdf-view';
  import PdfCustomToolbar from '../components/PdfCustomToolbar.svelte';
  import { getStrings } from '../strings.svelte.js';

  const strings = $derived(getStrings());

  let query = $state('LM7805');
  let source = $state('digikey');
  let loading = $state(false);
  let results = $state([]);
  let error = $state('');

  let currentPdf = $state(null);
  let pdfLoading = $state(false);
  let pdfError = $state('');
  let viewerKey = $state(0);
  let showHelp = $state(false);

  $effect(() => {
    function handleResize() {
      clearTimeout(window._resizeTimer);
      window._resizeTimer = setTimeout(() => {
        if (currentPdf) viewerKey++;
      }, 200);
    }
    window.addEventListener('resize', handleResize);
    return () => window.removeEventListener('resize', handleResize);
  });

  async function search() {
    if (!query.trim()) return;
    loading = true; error = ''; results = [];
    try {
      const command = source === 'digikey' ? 'search_datasheets' : 'search_lcsc';
      const res = await invoke(command, { query });
      results = res;
    } catch (e) { 
      console.error('Search error:', e);
      error = String(e); 
    }
    finally { loading = false; }
  }

  async function openPdf(item) {
    pdfLoading = true; pdfError = ''; currentPdf = null;
    try {
      await invoke('download_pdf', { url: item.datasheet_url, filename: item.name });
      const bytes = await invoke('read_pdf', { filename: item.name });
      const blob = new Blob([new Uint8Array(bytes)], { type: 'application/pdf' });
      currentPdf = { ...item, blobUrl: URL.createObjectURL(blob) };
    } catch (e) {
      pdfError = String(e);
    } finally {
      pdfLoading = false;
    }
  }

  async function savePdf() {
    if (!currentPdf) return;
    try {
      const path = await save({
        filters: [{ name: 'PDF', extensions: ['pdf'] }],
        defaultPath: `${currentPdf.name}.pdf`
      });
      if (path) {
        const bytes = await invoke('read_pdf', { filename: currentPdf.name });
        await writeFile(path, new Uint8Array(bytes));
      }
    } catch (e) {
      alert(strings.datasheets.labels.saveError + e);
    }
  }
</script>

{#if currentPdf}
  <Card title={currentPdf.name}>
    <div class="pdf-header">
      <Button variant="ghost" size="sm" onclick={() => currentPdf = null}>
        <ArrowLeft size={16} /> {strings.datasheets.back}
      </Button>
      <Button variant="primary" size="sm" onclick={savePdf}>{strings.datasheets.download}</Button>
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
{:else}
  <Card title={strings.datasheets.title}>
    {#snippet actions()}
      <HelpButton onclick={() => showHelp = true} />
    {/snippet}

    <div class="source-switch no-select">
      <button type="button" class="source-btn" class:active={source === 'digikey'} onclick={() => { source = 'digikey'; results = []; error = ''; }}>
        {strings.datasheets.digikey}
      </button>
      <button type="button" class="source-btn" class:active={source === 'lcsc'} onclick={() => { source = 'lcsc'; results = []; error = ''; }}>
        {strings.datasheets.lcsc}
      </button>
    </div>

    <div class="search-bar">
      <div class="search-input">
        <Input bind:value={query} label={strings.datasheets.labels.searchLabel} placeholder={strings.datasheets.labels.searchPlaceholder} />
      </div>
      <Button variant="primary" onclick={search} loading={loading}>{strings.datasheets.searchBtn}</Button>
    </div>

    {#if error}<div class="error-msg">{error}</div>{/if}

    {#if pdfLoading}
      <div class="loading-msg">
        <span class="spinner"></span>
        {strings.datasheets.labels.downloading}
      </div>
    {/if}

    {#if pdfError}<div class="error-msg">{pdfError}</div>{/if}

    {#if results.length > 0}
      <ul class="results-list">
        {#each results as item (`${item.name}_${item.manufacturer}_${item.datasheet_url}`)}
          <li class="result-item">
            <div class="result-info">
              <div class="result-name">{item.name}</div>
              <div class="result-mfr">{item.manufacturer}</div>
            </div>
            <Button variant="secondary" size="sm" onclick={() => openPdf(item)}>
              {strings.datasheets.openPdf}
            </Button>
          </li>
        {/each}
      </ul>
    {:else if !loading && !error}
      <div class="empty">{strings.datasheets.labels.enterQuery}</div>
    {/if}
  </Card>

  <SectionHelpModal open={showHelp} onClose={() => showHelp = false} title={strings.help.datasheets.title}>
    <h3>{strings.help.datasheets.sourcesTitle}</h3>
    <p>{strings.help.datasheets.sourcesText}</p>
    <h3>{strings.help.datasheets.cacheTitle}</h3>
    <p>{strings.help.datasheets.cacheText}</p>
    <h3>{strings.help.datasheets.viewerTitle}</h3>
    <p>{strings.help.datasheets.viewerText}</p>
    <h3>{strings.help.datasheets.tipsTitle}</h3>
    <ul>
      <li>{strings.help.datasheets.tip1}</li>
      <li>{strings.help.datasheets.tip2}</li>
      <li>{strings.help.datasheets.tip3}</li>
    </ul>
  </SectionHelpModal>
{/if}

<style>
  .source-switch { display: flex; gap: var(--space-1); padding: 4px; background: var(--bg-elevated-2); border: 1px solid var(--border-subtle); border-radius: var(--radius-md); align-self: flex-start; margin-bottom: var(--space-4); }
  .source-btn { min-height: 36px; padding: 0 var(--space-4); border-radius: var(--radius-sm); font-size: var(--font-size-sm); font-weight: 600; color: var(--text-secondary); background: transparent; transition: background var(--transition-fast), color var(--transition-fast); }
  .source-btn:hover { color: var(--text-primary); }
  .source-btn.active { background: var(--accent-dim); color: var(--accent); }
  .search-bar { display: flex; gap: var(--space-3); align-items: flex-end; margin-bottom: var(--space-4); }
  .search-input { flex: 1; }
  .error-msg { color: var(--danger); background: rgba(232, 90, 90, 0.1); padding: var(--space-3); border-radius: var(--radius-md); margin-bottom: var(--space-4); }
  
  /* Новые стили для спиннера и текста загрузки */
  .loading-msg { 
    display: flex; 
    align-items: center; 
    gap: var(--space-2); 
    color: var(--accent); 
    margin-bottom: var(--space-4); 
    font-size: var(--font-size-sm);
  }
  .spinner {
    width: 16px;
    height: 16px;
    border: 2px solid rgba(245, 184, 61, 0.3); /* Полупрозрачный акцент */
    border-top-color: var(--accent); /* Яркий акцент */
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
    flex-shrink: 0;
  }
  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  .results-list { list-style: none; padding: 0; display: flex; flex-direction: column; gap: var(--space-2); }
  .result-item { display: flex; justify-content: space-between; align-items: center; gap: var(--space-3); background: var(--bg-elevated-2); padding: var(--space-3); border-radius: var(--radius-md); border: 1px solid var(--border-subtle); }
  .result-info { flex: 1; overflow: hidden; }
  .result-name { font-weight: 600; color: var(--accent); }
  .result-mfr { font-size: var(--font-size-sm); color: var(--text-secondary); }
  .empty { color: var(--text-muted); text-align: center; padding: var(--space-5); }
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
</style>