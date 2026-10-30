<script>
  import { layoutStore } from '../stores/layout.svelte.js';
  import { localeStore } from '../stores/locale.svelte.js';
  import { getStrings } from '../strings.svelte.js';
  const strings = $derived(getStrings());
  import Tabs from './Tabs.svelte';
  import Logo from './Logo.svelte';
  import { Menu, X, Info, Settings } from '@lucide/svelte';
  import { navStore } from '../stores/navigation.svelte.js';

  let { children, title = '', onOpenAbout = () => {} } = $props();

  let menuOpen = $state(false);

  function toggleMenu() { menuOpen = !menuOpen; }
  function closeMenu() { menuOpen = false; }
  function handleOpenAbout() { menuOpen = false; onOpenAbout(); }
  function openSettings() { navStore.setSettings(); menuOpen = false; }

  $effect(() => {
    if (!layoutStore.isMobile) menuOpen = false;
  });
</script>

<div class="shell" class:mobile={layoutStore.isMobile}>
  {#if layoutStore.isMobile}
    <header class="topbar no-select">
      <button type="button" class="topbar__burger" aria-label={strings.nav.menu} onclick={toggleMenu}>
        <Menu size={22} strokeWidth={2} />
      </button>
      <div class="topbar__brand">
        <Logo size={28} />
        <span class="topbar__title">{strings.app.name}</span>
      </div>

      <button type="button" class="topbar__icon" aria-label="Info" onclick={handleOpenAbout}>
        <Info size={20} strokeWidth={2} />
      </button>
    </header>

    {#if menuOpen}
      <div class="overlay" onclick={closeMenu} role="presentation"></div>
      <aside class="drawer" role="dialog" aria-label={strings.nav.sections}>
        <div class="drawer__head">
          <span>{strings.nav.sections}</span>
          <button type="button" class="drawer__close" aria-label={strings.nav.close} onclick={closeMenu}>
            <X size={18} strokeWidth={2} />
          </button>
        </div>
        <div onclick={closeMenu} class="drawer-tabs">
          <Tabs />
        </div>
        
        <div class="drawer__footer">
          <div class="lang-switch">
            <button type="button" class="lang-btn" class:active={localeStore.current === 'ru'} onclick={() => localeStore.current = 'ru'}>RU</button>
            <button type="button" class="lang-btn" class:active={localeStore.current === 'en'} onclick={() => localeStore.current = 'en'}>EN</button>
            <button type="button" class="lang-btn" class:active={localeStore.current === 'zh'} onclick={() => localeStore.current = 'zh'}>ZH</button>
          </div>
          
          <div class="footer-row">
            <button type="button" class="drawer__link" onclick={handleOpenAbout}>
              <Info size={16} strokeWidth={2} />
              <span>About</span>
            </button>
            <button type="button" class="drawer__link" onclick={openSettings}>
              <Settings size={16} strokeWidth={2} />
              <span>{strings.settings.title}</span>
            </button>
          </div>
        </div>
      </aside>
    {/if}
  {:else}
    <aside class="sidebar">
      <div class="sidebar__brand">
  <Logo size={72} />
  <div class="sidebar__brand-name">{strings.app.name}</div>
  <div class="sidebar__brand-tagline">{strings.app.tagline}</div>
</div>

      <Tabs />

      <div class="sidebar__footer">
        <div class="lang-switch">
          <button type="button" class="lang-btn" class:active={localeStore.current === 'ru'} onclick={() => localeStore.current = 'ru'}>RU</button>
          <button type="button" class="lang-btn" class:active={localeStore.current === 'en'} onclick={() => localeStore.current = 'en'}>EN</button>
          <button type="button" class="lang-btn" class:active={localeStore.current === 'zh'} onclick={() => localeStore.current = 'zh'}>ZH</button>
        </div>
        
        <div class="footer-row">
          <button type="button" class="sidebar__link" onclick={handleOpenAbout}>
            <Info size={16} strokeWidth={2} />
            <span>About</span>
          </button>
          <button type="button" class="sidebar__link" onclick={openSettings}>
            <Settings size={16} strokeWidth={2} />
            <span>{strings.settings.title}</span>
          </button>
        </div>
      </div>
    </aside>
  {/if}

  <main class="content">
    {#if title}<h1 class="content__title">{title}</h1>{/if}
    {@render children()}
  </main>
</div>

<style>
  .shell { display: grid; height: 100%; overflow: hidden; }
  .shell:not(.mobile) { grid-template-columns: var(--sidebar-width) 1fr; }
  .shell.mobile { grid-template-rows: var(--header-height) 1fr; grid-template-columns: 1fr; }

  .sidebar { display: flex; flex-direction: column; background: var(--bg-elevated); border-right: 1px solid var(--border-subtle); padding: var(--space-3); overflow-y: auto; }
  .sidebar__brand { padding: var(--space-2) var(--space-3) var(--space-4); border-bottom: 1px solid var(--border-subtle); margin-bottom: var(--space-3); }
  .sidebar__brand {
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
  padding: var(--space-2) var(--space-3) var(--space-4);
  border-bottom: 1px solid var(--border-subtle);
  margin-bottom: var(--space-3);
  gap: var(--space-2);
}
.sidebar__brand-name {
  font-size: var(--font-size-xl);
  font-weight: 700;
  color: var(--accent);
  letter-spacing: 0.5px;
}
.sidebar__brand-tagline {
  font-size: var(--font-size-sm);
  color: var(--text-muted);
}
  .sidebar__brand-name { font-size: var(--font-size-xl); font-weight: 700; letter-spacing: 0.5px; }
  .sidebar__brand-tagline { font-size: var(--font-size-sm); color: var(--text-muted); margin-top: var(--space-1); }
  
  .sidebar__footer { margin-top: auto; padding-top: var(--space-3); border-top: 1px solid var(--border-subtle); display: flex; flex-direction: column; gap: var(--space-2); padding-bottom: env(safe-area-inset-bottom); }
  .footer-row { display: flex; align-items: center; gap: 2px; width: 100%; }
  
  .sidebar__link, .drawer__link {
    display: flex;
    align-items: center;
    gap: var(--space-1);
    padding: var(--space-2) var(--space-2);
    border-radius: var(--radius-md);
    color: var(--text-secondary);
    font-size: var(--font-size-sm);
    white-space: nowrap;
    transition: background var(--transition-fast), color var(--transition-fast);
    flex: 1;
    justify-content: flex-start;
  }
  
  .sidebar__link:hover, .drawer__link:hover { 
    background: var(--bg-elevated-2); 
    color: var(--text-primary); 
  }

  .topbar { display: flex; align-items: center; gap: var(--space-2); padding: 0 var(--space-3); padding-top: env(safe-area-inset-top); background: var(--bg-elevated); border-bottom: 1px solid var(--border-subtle); position: relative; z-index: 10; }
  .topbar__burger { display: flex; align-items: center; justify-content: center; width: var(--touch-target); height: var(--touch-target); color: var(--text-primary); border-radius: var(--radius-md); }
  .topbar__burger:hover { background: var(--bg-elevated-2); }
  .topbar__brand { display: flex; align-items: center; gap: var(--space-2); color: var(--accent); flex: 1; }
  .topbar__title { font-size: var(--font-size-lg); font-weight: 600; }
  .topbar__icon { display: flex; align-items: center; justify-content: center; width: var(--touch-target); height: var(--touch-target); color: var(--text-secondary); border-radius: var(--radius-md); transition: background var(--transition-fast), color var(--transition-fast); }
  .topbar__icon:hover { background: var(--bg-elevated-2); color: var(--text-primary); }

  .overlay { position: fixed; inset: 0; background: var(--bg-overlay); z-index: 20; }
  .drawer { position: fixed; top: 0; left: 0; bottom: 0; width: min(80vw, 300px); background: var(--bg-elevated); border-right: 1px solid var(--border-subtle); z-index: 30; padding: var(--space-3); display: flex; flex-direction: column; overflow-y: auto; padding-top: env(safe-area-inset-top); }
  .drawer__head { display: flex; align-items: center; justify-content: space-between; gap: var(--space-2); font-size: var(--font-size-sm); text-transform: uppercase; letter-spacing: 0.5px; color: var(--text-muted); padding: var(--space-2) var(--space-3); border-bottom: 1px solid var(--border-subtle); margin-bottom: var(--space-2); }
  .drawer__close { display: flex; align-items: center; justify-content: center; width: 32px; height: 32px; border-radius: var(--radius-md); color: var(--text-secondary); }
  .drawer__close:hover { background: var(--bg-elevated-2); color: var(--text-primary); }
  .drawer__footer { margin-top: auto; padding-top: var(--space-3); border-top: 1px solid var(--border-subtle); display: flex; flex-direction: column; gap: var(--space-2); padding-bottom: env(safe-area-inset-bottom); }
  
  .drawer-tabs { cursor: pointer; }

  .lang-switch { display: flex; gap: 4px; padding: 4px; background: var(--bg-elevated-2); border: 1px solid var(--border-subtle); border-radius: var(--radius-md); }
  .lang-btn { flex: 1; min-height: 32px; border-radius: var(--radius-sm); font-size: var(--font-size-sm); font-weight: 600; color: var(--text-secondary); background: transparent; transition: background var(--transition-fast), color var(--transition-fast); }
  .lang-btn.active { background: var(--accent-dim); color: var(--accent); }

  .content { overflow-y: auto; padding: var(--space-4); min-width: 0; }
  .content__title { font-size: var(--font-size-xl); margin-bottom: var(--space-4); }
  @media (max-width: 768px) { .content { padding: var(--space-3); } .content__title { font-size: var(--font-size-lg); margin-bottom: var(--space-3); } }
</style>