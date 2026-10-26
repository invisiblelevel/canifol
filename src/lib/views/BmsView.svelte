<script>
  import Card from '../components/Card.svelte';
  import HelpButton from '../components/HelpButton.svelte';
  import SectionHelpModal from '../components/SectionHelpModal.svelte';
  import { getStrings } from '../strings.svelte.js';
  import { BMS_CALCULATORS } from '../calculators/bms/index.js';

  const strings = $derived(getStrings());

  let activeId = $state(BMS_CALCULATORS[0]?.id ?? '');
  let showHelp = $state(false);
  const active = $derived(BMS_CALCULATORS.find((c) => c.id === activeId) ?? BMS_CALCULATORS[0]);
  const h = $derived(strings.help.bms);
</script>

<Card title={strings.bms.title}>
  {#snippet actions()}
    <HelpButton onclick={() => showHelp = true} />
  {/snippet}

  <nav class="subtabs no-select" aria-label="Sub-calculators">
    {#each BMS_CALCULATORS as calc (calc.id)}
      <button type="button" class="subtabs__item" class:active={activeId === calc.id} onclick={() => (activeId = calc.id)}>
        {strings.bms.sub[calc.id]}
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
  <h3>{h.balancingTitle}</h3><p>{h.balancingText}</p>
  <h3>{h.configTitle}</h3><p>{h.configText}</p>
  <h3>{h.shuntTitle}</h3><p>{h.shuntText}</p>
  <h3>{h.mosfetTitle}</h3><p>{h.mosfetText}</p>
</SectionHelpModal>

<style>
  .subtabs { display: flex; gap: var(--space-1); padding-bottom: var(--space-3); margin-bottom: var(--space-4); border-bottom: 1px solid var(--border-subtle); overflow-x: auto; }
  .subtabs__item { min-height: 36px; padding: 0 var(--space-3); border-radius: var(--radius-md); font-size: var(--font-size-sm); color: var(--text-secondary); background: transparent; white-space: nowrap; transition: background var(--transition-fast), color var(--transition-fast); }
  .subtabs__item:hover { background: var(--bg-elevated-2); color: var(--text-primary); }
  .subtabs__item.active { background: var(--accent-dim); color: var(--accent); }
  .calc-body { min-height: 100px; }
</style>