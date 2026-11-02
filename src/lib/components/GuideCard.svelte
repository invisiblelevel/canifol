<script>
  import { Play, FileText } from '@lucide/svelte';
  import { getStrings } from '../strings.svelte.js';

  let { guide, onOpen } = $props();
  const strings = $derived(getStrings());

  const isVideo = $derived(guide.type === 'video');
  let imgError = $state(false);

  function handleClick() {
    onOpen?.(guide);
  }

  function handleKeydown(e) {
    if (e.key === 'Enter' || e.key === ' ') {
      e.preventDefault();
      handleClick();
    }
  }
</script>

<article
  class="guide-card"
  role="button"
  tabindex="0"
  onclick={handleClick}
  onkeydown={handleKeydown}
  aria-label={guide.title}
>
  <div class="guide-card__thumb">
    {#if guide.image && !imgError}
      <img
        src={guide.image}
        alt=""
        loading="lazy"
        onerror={() => (imgError = true)}
      />
    {:else}
      <div class="guide-card__placeholder">
        {#if isVideo}
          <Play size={32} strokeWidth={1.5} />
        {:else}
          <FileText size={32} strokeWidth={1.5} />
        {/if}
      </div>
    {/if}
    <span class="guide-card__badge">
      {#if isVideo}
        <Play size={12} strokeWidth={2.5} />
        {strings.guides.filters.video}
      {:else}
        <FileText size={12} strokeWidth={2.5} />
        {strings.guides.filters.article}
      {/if}
    </span>
  </div>

  <div class="guide-card__body">
    <h3 class="guide-card__title">{guide.title}</h3>
    {#if guide.description}
      <p class="guide-card__desc">{guide.description}</p>
    {/if}
    <div class="guide-card__meta">
      <span class="guide-card__source">{guide.source}</span>
    </div>
  </div>
</article>

<style>
  .guide-card {
    display: flex;
    flex-direction: column;
    background: var(--bg-elevated);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-lg);
    overflow: hidden;
    cursor: pointer;
    transition: border-color var(--transition-fast), transform var(--transition-fast), box-shadow var(--transition-fast);
  }

  .guide-card:hover {
    border-color: var(--accent);
    transform: translateY(-2px);
    box-shadow: var(--shadow-md);
  }

  .guide-card:focus-visible {
    outline: 2px solid var(--accent);
    outline-offset: 2px;
  }

  .guide-card__thumb {
    position: relative;
    aspect-ratio: 16 / 9;
    background: var(--bg-elevated-2);
    overflow: hidden;
  }

  .guide-card__thumb img {
    width: 100%;
    height: 100%;
    object-fit: cover;
    display: block;
  }

  .guide-card__placeholder {
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--accent);
    opacity: 0.6;
  }

  .guide-card__badge {
    position: absolute;
    top: 8px;
    left: 8px;
    display: inline-flex;
    align-items: center;
    gap: 4px;
    padding: 3px 8px;
    border-radius: var(--radius-sm);
    font-size: 11px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.03em;
    background: rgba(0, 0, 0, 0.7);
    color: #fff;
    backdrop-filter: blur(4px);
  }

  .guide-card__body {
    display: flex;
    flex-direction: column;
    gap: 6px;
    padding: 12px 14px 14px;
    flex: 1;
  }

  .guide-card__title {
    margin: 0;
    font-size: var(--font-size-base);
    font-weight: 600;
    color: var(--text-primary);
    line-height: 1.35;
    display: -webkit-box;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
  }

  .guide-card__desc {
    margin: 0;
    font-size: var(--font-size-sm);
    color: var(--text-secondary);
    line-height: 1.4;
    display: -webkit-box;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
  }

  .guide-card__meta {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    margin-top: auto;
    padding-top: 4px;
  }

  .guide-card__source {
    font-size: 11px;
    font-weight: 600;
    color: var(--accent);
    text-transform: uppercase;
    letter-spacing: 0.04em;
  }
</style>