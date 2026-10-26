<script>
  import { onMount } from 'svelte';
  import AppShell from './lib/components/AppShell.svelte';
  import AboutModal from './lib/components/AboutModal.svelte';
  import SupportModal from './lib/components/SupportModal.svelte';
  import { navStore } from './lib/stores/navigation.svelte.js';
  import { detectSystemLocale } from './lib/stores/locale.svelte.js';

  import ResistorsView from './lib/views/ResistorsView.svelte';
  import CapacitorsView from './lib/views/CapacitorsView.svelte';
  import InductorsView from './lib/views/InductorsView.svelte';
  import BmsView from './lib/views/BmsView.svelte';
  import TransistorsView from './lib/views/TransistorsView.svelte';
  import DatasheetsView from './lib/views/DatasheetsView.svelte';
  import SettingsView from './lib/views/SettingsView.svelte';

  let showAbout = $state(false);
  let showSupport = $state(false);

  const views = {
    resistors:  ResistorsView,
    capacitors: CapacitorsView,
    inductors:  InductorsView,
    bms:        BmsView,
    transistors: TransistorsView,
    datasheets: DatasheetsView,
    settings:   SettingsView
  };

  const currentView = $derived(views[navStore.current]);

  onMount(() => {
    detectSystemLocale();
  });
</script>

<AppShell onOpenAbout={() => showAbout = true}>
  {#if currentView === ResistorsView}
    <ResistorsView />
  {:else if currentView === CapacitorsView}
    <CapacitorsView />
  {:else if currentView === InductorsView}
    <InductorsView />
  {:else if currentView === BmsView}
    <BmsView />
  {:else if currentView === TransistorsView}
    <TransistorsView />
  {:else if currentView === SettingsView}
    <SettingsView />
  {:else}
    <DatasheetsView />
  {/if}
</AppShell>

<AboutModal
  open={showAbout}
  onClose={() => showAbout = false}
  onOpenSupport={() => { showAbout = false; showSupport = true; }}
/>

<SupportModal
  open={showSupport}
  onClose={() => showSupport = false}
/>