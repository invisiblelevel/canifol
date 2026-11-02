<script>
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { BookOpen, RefreshCw } from '@lucide/svelte';
  import { getStrings } from '../strings.svelte.js';
  import { localeStore } from '../stores/locale.svelte.js';
  import Button from '../components/Button.svelte';
  import Input from '../components/Input.svelte';
  import EmptyState from '../components/EmptyState.svelte';
  import GuideCard from '../components/GuideCard.svelte';
  import GuideWebView from '../components/GuideWebView.svelte';

  const strings = $derived(getStrings());
  const locale = $derived(localeStore.current);

  let guides = $state([]);
  let loading = $state(true);
  let error = $state(null);
  let search = $state('');
  let activeGuide = $state(null);
  let refreshing = $state(false);

  const filtered = $derived(
    (guides || []).filter((g) => {
      if (!g || !g.title || !g.link) return false;
      if (search && !g.title.toLowerCase().includes(search.toLowerCase())) return false;
      return true;
    })
  );

  async function load(force = false) {
    if (force) refreshing = true;
    else loading = true;
    error = null;
    try {
      const raw = await invoke('fetch_guides', { locale, force });
      // Жёсткая фильтрация: оставляем только объекты с link и title
      guides = Array.isArray(raw) ? raw.filter((g) => g && g.link && g.title) : [];
    } catch (e) {
      console.error('Guides load error:', e);
      error = String(e);
      guides = [];
    } finally {
      loading = false;
      refreshing = false;
    }
  }

  function openGuide(guide) {
    if (guide && guide.link) {
      activeGuide = guide;
    }
  }

  function closeGuide() {
    activeGuide = null;
  }

  onMount(() => {
    load();
  });

  $effect(() => {
    locale;
    load();
  });
</script>

{#if activeGuide && activeGuide.link}
  <GuideWebView guide={activeGuide} onClose={closeGuide} />
{:else}
  <div class="guides">
    <header class="guides__header">
      <div class="guides__title-wrap">
        <h1 class="guides__title">{strings.guides.title}</h1>
        <p class="guides__subtitle">{strings.guides.subtitle}</p>
      </div>
      <Button
        variant="secondary"
        size="sm"
        onclick={() => load(true)}
        loading={refreshing}
      >
        <RefreshCw size={16} />
        {strings.guides.refresh}
      </Button>
    </header>

    <div class="guides__filters">
      <div class="guides__search">
        <Input
          type="text"
          placeholder={strings.guides.searchPlaceholder}
          bind:value={search}
        />
      </div>
    </div>

    {#if loading}
      <div class="guides__loading">
        <div class="guides__spinner"></div>
        <p class="guides__loading-title">{strings.guides.loading}</p>
        <p class="guides__loading-hint">{strings.guides.loadingHint}</p>
      </div>
    {:else if error}
      <EmptyState
        icon={BookOpen}
        title={strings.guides.error}
        description={error}
      >
        <Button variant="primary" onclick={() => load(true)}>
          {strings.guides.retry}
        </Button>
      </EmptyState>
    {:else if guides.length === 0}
      <EmptyState
        icon={BookOpen}
        title={strings.guides.empty}
        description={strings.guides.emptyHint}
      >
        <Button variant="primary" onclick={() => load(true)}>
          {strings.guides.retry}
        </Button>
      </EmptyState>
    {:else if filtered.length === 0}
      <EmptyState
        icon={BookOpen}
        title={`${strings.guides.emptySearch}: «${search}»`}
        description={strings.guides.emptyHint}
      >
        <Button variant="secondary" onclick={() => (search = '')}>
          {strings.buttons.clear}
        </Button>
      </EmptyState>
    {:else}
      <div class="guides__grid">
        {#each filtered as guide (guide.id)}
          <GuideCard {guide} onOpen={openGuide} />
        {/each}
      </div>
    {/if}
  </div>
{/if}

<style>
  .guides {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
    max-width: 1400px;
    margin: 0 auto;
    width: 100%;
  }

  .guides__header {
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    gap: var(--space-3);
    flex-wrap: wrap;
  }

  .guides__title-wrap {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .guides__title {
    margin: 0;
    font-size: var(--font-size-xl);
    font-weight: 700;
    letter-spacing: -0.02em;
    color: var(--text-primary);
  }

  .guides__subtitle {
    margin: 0;
    font-size: var(--font-size-sm);
    color: var(--text-muted);
  }

  .guides__filters {
    display: flex;
    align-items: center;
    gap: var(--space-3);
    flex-wrap: wrap;
  }

  .guides__search {
    flex: 1;
    min-width: 220px;
  }

  .guides__grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
    gap: var(--space-4);
  }

  .guides__loading {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: var(--space-3);
    padding: var(--space-6) var(--space-4);
    min-height: 350px;
    text-align: center;
  }

  .guides__spinner {
    width: 40px;
    height: 40px;
    border: 3px solid var(--border-strong);
    border-top-color: var(--accent);
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  .guides__loading-title {
    margin: 0;
    font-size: var(--font-size-lg);
    font-weight: 600;
    color: var(--text-primary);
  }

  .guides__loading-hint {
    margin: 0;
    font-size: var(--font-size-sm);
    color: var(--text-muted);
  }

  @media (max-width: 768px) {
    .guides__title { font-size: var(--font-size-lg); }
    .guides__grid { grid-template-columns: 1fr; }
    .guides__search { min-width: 100%; }
  }
</style>