<script>
  import { getPdfViewerContext } from 'svelte-pdf-view';
  import { ChevronLeft, ChevronRight, ZoomIn, ZoomOut, Maximize, Minimize } from '@lucide/svelte';
  import { getStrings } from '../strings.svelte.js';

  const strings = $derived(getStrings());
  const pdfContext = getPdfViewerContext();

  let pageInput = $state(1);
  let isMaximized = $state(false);

  $effect(() => {
    if (pdfContext.state.currentPage) {
      pageInput = pdfContext.state.currentPage;
    }
  });

  // Следим за изменением нативного полноэкранного режима (чтобы иконка менялась при Esc)
  $effect(() => {
    function onFullscreenChange() {
      if (!document.fullscreenElement) {
        isMaximized = false;
        const container = document.querySelector('.pdf-main-container');
        if (container) container.classList.remove('pdf-maximized');
      }
    }
    document.addEventListener('fullscreenchange', onFullscreenChange);
    return () => document.removeEventListener('fullscreenchange', onFullscreenChange);
  });

  function goToPage() {
    const p = parseInt(pageInput, 10);
    if (!isNaN(p) && p >= 1 && p <= pdfContext.state.totalPages) {
      pdfContext.actions.goToPage(p);
    } else {
      pageInput = pdfContext.state.currentPage;
    }
  }

  function onKeydown(e) {
    if (e.key === 'Enter') {
      e.preventDefault();
      goToPage();
    }
  }

  async function toggleFullscreen() {
    const container = document.querySelector('.pdf-main-container');
    if (!container) return;

    isMaximized = !isMaximized;

    if (isMaximized) {
      container.classList.add('pdf-maximized');
      try {
        await container.requestFullscreen();
      } catch (e) {
        console.warn('Native fullscreen failed, using CSS fallback', e);
      }
    } else {
      container.classList.remove('pdf-maximized');
      if (document.fullscreenElement) {
        await document.exitFullscreen();
      }
    }
  }
</script>

<div class="custom-toolbar">
  <button 
    class="tool-btn" 
    onclick={() => pdfContext.actions.goToPage(pdfContext.state.currentPage - 1)} 
    disabled={pdfContext.state.currentPage <= 1}
  >
    <ChevronLeft size={16} />
  </button>
  
  <input
    type="text"
    bind:value={pageInput}
    onkeydown={onKeydown}
    onblur={goToPage}
    class="page-input"
  />
  
  <span class="total-pages">/ {pdfContext.state.totalPages}</span>
  
  <button 
    class="tool-btn" 
    onclick={() => pdfContext.actions.goToPage(pdfContext.state.currentPage + 1)} 
    disabled={pdfContext.state.currentPage >= pdfContext.state.totalPages}
  >
    <ChevronRight size={16} />
  </button>

  <div class="spacer"></div>

  <button 
    class="tool-btn" 
    type="button"
    onclick={toggleFullscreen}
    title={strings.datasheets.labels.fullscreen}
  >
    {#if isMaximized}
      <Minimize size={16} />
    {:else}
      <Maximize size={16} />
    {/if}
  </button>

  <button 
    class="tool-btn" 
    onclick={() => pdfContext.actions.zoomOut()} 
    disabled={pdfContext.state.scale <= 0.2}
  >
    <ZoomOut size={16} />
  </button>
  
  <span class="zoom-text">{Math.round(pdfContext.state.scale * 100)}%</span>
  
  <button 
    class="tool-btn" 
    onclick={() => pdfContext.actions.zoomIn()} 
    disabled={pdfContext.state.scale >= 3.0}
  >
    <ZoomIn size={16} />
  </button>
</div>

<style>
  .custom-toolbar { display: flex; align-items: center; gap: var(--space-2); padding: var(--space-2) var(--space-3); background: var(--bg-elevated-2); border-bottom: 1px solid var(--border-subtle); color: var(--text-primary); position: relative; z-index: 100; }
  .tool-btn { display: flex; align-items: center; justify-content: center; width: 32px; height: 32px; border-radius: var(--radius-sm); color: var(--text-secondary); background: var(--bg-elevated); border: 1px solid var(--border-strong); transition: background var(--transition-fast), color var(--transition-fast); }
  .tool-btn:hover:not(:disabled) { background: var(--bg-elevated-3); color: var(--text-primary); }
  .tool-btn:disabled { opacity: 0.3; cursor: not-allowed; }
  .page-input { width: 48px; height: 32px; text-align: center; background: var(--bg-base); border: 1px solid var(--border-strong); border-radius: var(--radius-sm); color: var(--text-primary); font-size: var(--font-size-sm); outline: none; }
  .page-input:focus { border-color: var(--accent); }
  .total-pages { font-size: var(--font-size-sm); color: var(--text-secondary); margin-right: var(--space-2); }
  .spacer { flex: 1; }
  .zoom-text { font-size: var(--font-size-sm); color: var(--text-secondary); min-width: 48px; text-align: center; }
</style>