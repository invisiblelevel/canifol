<script>
  import Card from '../components/Card.svelte';
  import HelpButton from '../components/HelpButton.svelte';
  import SectionHelpModal from '../components/SectionHelpModal.svelte';
  import { getStrings } from '../strings.svelte.js';
  import { CAPACITOR_CALCULATORS } from '../calculators/capacitors/index.js';

  const strings = $derived(getStrings());

  let activeId = $state(CAPACITOR_CALCULATORS[0]?.id ?? '');
  let showHelp = $state(false);
  const active = $derived(CAPACITOR_CALCULATORS.find((c) => c.id === activeId) ?? CAPACITOR_CALCULATORS[0]);
  const h = $derived(strings.help.capacitors);
</script>

<Card title={strings.capacitors.title}>
  {#snippet actions()}
    <HelpButton onclick={() => showHelp = true} />
  {/snippet}

  <nav class="subtabs no-select" aria-label="Sub-calculators">
    {#each CAPACITOR_CALCULATORS as calc (calc.id)}
      <button type="button" class="subtabs__item" class:active={activeId === calc.id} onclick={() => (activeId = calc.id)}>
        {strings.capacitors.sub[calc.id]}
      </button>
    {/each}
  </nav>
  <div class="calc-body">
    {#key activeId}
      <div class="animate-in">
        {#if active}<active.component />{/if}
      </div>
    {/key}
  </div>
</Card>

<SectionHelpModal open={showHelp} onClose={() => showHelp = false} title={h.title}>
  <h3>{h.markingTitle}</h3><p>{h.markingText}</p>
  <h3>{h.reactanceTitle}</h3><p>{h.reactanceText}</p>
  <h3>{h.connTitle}</h3><p>{h.connText}</p>
  <h3>{h.esrTitle}</h3><p>{h.esrText}</p>
</SectionHelpModal>

<style>
  .subtabs { display: flex; gap: var(--space-1); padding-bottom: var(--space-3); margin-bottom: var(--space-4); border-bottom: 1px solid var(--border-subtle); overflow-x: auto; }
  .subtabs__item { min-height: 36px; padding: 0 var(--space-3); border-radius: var(--radius-md); font-size: var(--font-size-sm); color: var(--text-secondary); background: transparent; white-space: nowrap; transition: background var(--transition-fast), color var(--transition-fast); }
  .subtabs__item:hover { background: var(--bg-elevated-2); color: var(--text-primary); }
  .subtabs__item.active { background: var(--accent-dim); color: var(--accent); }
  .calc-body { min-height: 100px; }
</style>