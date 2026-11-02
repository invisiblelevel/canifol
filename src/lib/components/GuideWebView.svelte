<script>
  import { ArrowLeft, RotateCw, X, Maximize } from '@lucide/svelte';
  import { getStrings } from '../strings.svelte.js';

  let { guide, onClose } = $props();
  const strings = $derived(getStrings());

  let loading = $state(true);
  let failed = $state(false);
  let iframeEl = $state(null);
  let reloadKey = $state(0);
  let containerEl = $state(null);

  // Проверка: Android или нет
  const isAndroid = $derived(
    typeof navigator !== 'undefined' && /android/i.test(navigator.userAgent)
  );

  function handleIframeLoad() {
    loading = false;
  }

  function handleIframeError() {
    loading = false;
    failed = true;
  }

  function goBack() {
    try {
      iframeEl?.contentWindow?.history.back();
    } catch (e) {
      console.error('Back failed:', e);
    }
  }

  function reload() {
    loading = true;
    failed = false;
    reloadKey++;
  }

  async function toggleFullscreen() {
    if (!containerEl || isAndroid) return;
    try {
      if (document.fullscreenElement) {
        await document.exitFullscreen();
      } else {
        await containerEl.requestFullscreen();
      }
    } catch (e) {
      console.error('Fullscreen error:', e);
    }
  }
</script>

<div class="guide-webview">
  <header class="guide-webview__toolbar">
    <button class="guide-webview__btn" onclick={goBack} title={strings.guides.back} aria-label={strings.guides.back}>
      <ArrowLeft size={18} />
    </button>
    <button class="guide-webview__btn" onclick={reload} title={strings.guides.reload} aria-label={strings.guides.reload}>
      <RotateCw size={18} />
    </button>
    <div class="guide-webview__title" title={guide.title}>{guide.title}</div>
    
    <!-- Кнопка фуллскрина ТОЛЬКО на десктопе -->
    {#if !isAndroid}
      <button class="guide-webview__btn" onclick={toggleFullscreen} title="Fullscreen" aria-label="Fullscreen">
        <Maximize size={18} />
      </button>
    {/if}

    <button class="guide-webview__btn guide-webview__btn--close" onclick={onClose} title={strings.guides.close} aria-label={strings.guides.close}>
      <X size={18} />
    </button>
  </header>

  <div class="guide-webview__content">
    {#if loading}
      <div class="guide-webview__loading">
        <div class="guide-webview__spinner"></div>
        <span>{strings.guides.loading}</span>
      </div>
    {/if}

    {#if failed}
      <div class="guide-webview__failed">
        <p class="guide-webview__failed-title">{strings.guides.openFailed}</p>
        <p class="guide-webview__failed-hint">{strings.guides.openFailedHint}</p>
      </div>
    {:else}
      {#key reloadKey}
        <div bind:this={containerEl} class="guide-webview__iframe-container">
          <iframe
            bind:this={iframeEl}
            src={guide.link}
            title={guide.title}
            class="guide-webview__iframe"
            onload={handleIframeLoad}
            onerror={handleIframeError}
            sandbox="allow-same-origin allow-scripts allow-popups allow-forms allow-presentation"
            allowfullscreen
            allow="fullscreen; picture-in-picture"
          ></iframe>
        </div>
      {/key}
    {/if}
  </div>
</div>

<style>
  .guide-webview { display: flex; flex-direction: column; height: calc(100vh - 120px); background: var(--bg-elevated); border: 1px solid var(--border-subtle); border-radius: var(--radius-lg); overflow: hidden; }
  .guide-webview__toolbar { display: flex; align-items: center; gap: var(--space-2); padding: 8px 12px; background: var(--bg-elevated); border-bottom: 1px solid var(--border-subtle); flex-shrink: 0; }
  .guide-webview__btn { display: inline-flex; align-items: center; justify-content: center; width: 36px; height: 36px; border-radius: var(--radius-md); background: transparent; color: var(--text-secondary); border: 1px solid transparent; cursor: pointer; transition: background var(--transition-fast), color var(--transition-fast); flex-shrink: 0; }
  .guide-webview__btn:hover { background: var(--bg-elevated-2); color: var(--text-primary); }
  .guide-webview__btn--close { margin-left: auto; color: var(--danger); }
  .guide-webview__btn--close:hover { background: var(--danger); color: #fff; }
  .guide-webview__title { flex: 1; font-size: var(--font-size-sm); font-weight: 600; color: var(--text-primary); white-space: nowrap; overflow: hidden; text-overflow: ellipsis; padding: 0 var(--space-2); }
  .guide-webview__content { flex: 1; position: relative; overflow: hidden; background: #000; }
  .guide-webview__iframe-container { width: 100%; height: 100%; background: #000; }
  .guide-webview__iframe { width: 100%; height: 100%; border: none; display: block; }
  .guide-webview__iframe-container:fullscreen { width: 100vw; height: 100vh; border-radius: 0; }
  .guide-webview__loading { position: absolute; inset: 0; display: flex; flex-direction: column; align-items: center; justify-content: center; gap: var(--space-3); background: var(--bg-base); color: var(--text-secondary); font-size: var(--font-size-sm); z-index: 1; }
  .guide-webview__spinner { width: 32px; height: 32px; border: 3px solid var(--border-strong); border-top-color: var(--accent); border-radius: 50%; animation: spin 0.8s linear infinite; }
  @keyframes spin { to { transform: rotate(360deg); } }
  .guide-webview__failed { position: absolute; inset: 0; display: flex; flex-direction: column; align-items: center; justify-content: center; gap: var(--space-2); padding: var(--space-5); background: var(--bg-base); text-align: center; }
  .guide-webview__failed-title { font-size: var(--font-size-lg); font-weight: 600; color: var(--text-primary); margin: 0; }
  .guide-webview__failed-hint { font-size: var(--font-size-sm); color: var(--text-muted); margin: 0; }
</style>