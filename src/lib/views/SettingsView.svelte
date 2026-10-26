<script>
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import Card from '../components/Card.svelte';
  import Button from '../components/Button.svelte';
  import { getStrings } from '../strings.svelte.js';
  import { uiScaleStore } from '../stores/uiScale.svelte.js';

  const strings = $derived(getStrings());

  let cacheSize = $state(0);
  let clearing = $state(false);
  let cleared = $state(false);

  function formatBytes(bytes) {
    if (bytes === 0) return `0 ${strings.units.b}`;
    if (bytes < 1024) return `${bytes} ${strings.units.b}`;
    if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} ${strings.units.kb}`;
    if (bytes < 1024 * 1024 * 1024) return `${(bytes / (1024 * 1024)).toFixed(1)} ${strings.units.mb}`;
    return `${(bytes / (1024 * 1024 * 1024)).toFixed(2)} ${strings.units.gb}`;
  }

  async function loadCacheSize() {
    try {
      const size = await invoke('get_cache_size');
      console.log('[DEBUG] Cache size from Rust:', size);
      cacheSize = size;
    } catch (e) {
      console.error('[DEBUG] Failed to load cache size', e);
    }
  }

  async function clearCache() {
    if (!confirm(strings.settings.confirmClear)) return;
    clearing = true;
    try {
      await invoke('clear_cache');
      await loadCacheSize();
      cleared = true;
      setTimeout(() => (cleared = false), 2000);
    } catch (e) {
      console.error('Clear cache error:', e);
      alert('Error: ' + e);
    } finally {
      clearing = false;
    }
  }

  function setScale(value) {
    uiScaleStore.set(value);
  }

  onMount(() => {
    loadCacheSize();
  });
</script>

<Card title={strings.settings.title}>
  <div class="settings">
    <!-- Масштаб -->
    <div class="section">
      <div class="section__title">{strings.settings.scaleTitle}</div>
      <div class="section__hint">{strings.settings.scaleHint}</div>
      <div class="scale-control">
        <input
          type="range"
          min="70"
          max="150"
          step="5"
          value={uiScaleStore.current}
          oninput={(e) => setScale(parseInt(e.currentTarget.value, 10))}
          class="scale-slider"
        />
        <div class="scale-value">{uiScaleStore.current}%</div>
      </div>
      <div class="scale-presets">
        {#each [80, 90, 100, 110, 125, 150] as preset}
          <button
            class="preset-btn"
            class:active={uiScaleStore.current === preset}
            onclick={() => setScale(preset)}
          >
            {preset}%
          </button>
        {/each}
      </div>
    </div>

    <!-- Кэш -->
    <div class="section">
      <div class="section__title">{strings.settings.cacheTitle}</div>
      <div class="section__hint">{strings.settings.cacheHint}</div>
      <div class="cache-row">
        <div class="cache-info">
          <span class="cache-info__label">{strings.settings.cacheSize}:</span>
          <span class="cache-info__value">
            {cacheSize > 0 ? formatBytes(cacheSize) : strings.settings.cacheEmpty}
          </span>
        </div>
        <Button
          variant={cacheSize > 0 ? 'secondary' : 'ghost'}
          size="sm"
          onclick={clearCache}
          loading={clearing}
          disabled={cacheSize === 0}
        >
          {cleared ? strings.settings.cacheCleared : strings.buttons.clearCache}
        </Button>
      </div>
    </div>
  </div>
</Card>

<style>
  .settings { display: flex; flex-direction: column; gap: var(--space-5); }
  .section { display: flex; flex-direction: column; gap: var(--space-3); padding-bottom: var(--space-4); border-bottom: 1px solid var(--border-subtle); }
  .section:last-of-type { border-bottom: none; padding-bottom: 0; }
  .section__title { font-size: var(--font-size-base); font-weight: 600; color: var(--text-primary); }
  .section__hint { font-size: var(--font-size-sm); color: var(--text-muted); line-height: 1.5; }

  .scale-control { display: flex; align-items: center; gap: var(--space-3); }
  .scale-slider { flex: 1; accent-color: var(--accent); }
  .scale-value { min-width: 60px; text-align: right; font-family: var(--font-mono); font-weight: 600; color: var(--accent); }

  .scale-presets { display: flex; gap: var(--space-2); flex-wrap: wrap; }
  .preset-btn {
    min-height: 36px;
    padding: 0 var(--space-3);
    border-radius: var(--radius-md);
    font-size: var(--font-size-sm);
    color: var(--text-secondary);
    background: var(--bg-elevated-2);
    border: 1px solid var(--border-subtle);
  }
  .preset-btn:hover { color: var(--text-primary); border-color: var(--border-strong); }
  .preset-btn.active { background: var(--accent-dim); color: var(--accent); border-color: var(--accent); }

  .cache-row { display: flex; align-items: center; justify-content: space-between; gap: var(--space-3); flex-wrap: wrap; }
  .cache-info { display: flex; gap: var(--space-2); align-items: baseline; }
  .cache-info__label { font-size: var(--font-size-sm); color: var(--text-muted); }
  .cache-info__value { font-family: var(--font-mono); font-size: var(--font-size-base); color: var(--text-primary); font-weight: 600; }
</style>