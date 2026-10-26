<script>
  import Card from '../components/Card.svelte';
  import HelpButton from '../components/HelpButton.svelte';
  import SectionHelpModal from '../components/SectionHelpModal.svelte';
  import { RESISTOR_CALCULATORS } from '../calculators/resistors/index.js';
  import { getStrings } from '../strings.svelte.js';

  const strings = $derived(getStrings());

  let activeId = $state(RESISTOR_CALCULATORS[0]?.id ?? '');
  let showHelp = $state(false);
  const active = $derived(RESISTOR_CALCULATORS.find((c) => c.id === activeId) ?? RESISTOR_CALCULATORS[0]);
</script>

<Card title={strings.resistors.title}>
  {#snippet actions()}
    <HelpButton onclick={() => showHelp = true} />
  {/snippet}

  <nav class="subtabs no-select">
    {#each RESISTOR_CALCULATORS as calc (calc.id)}
      <button type="button" class="subtabs__item" class:active={activeId === calc.id} onclick={() => (activeId = calc.id)}>
        {calc.label}
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

<SectionHelpModal open={showHelp} onClose={() => showHelp = false} title={strings.help.resistors.title}>
  <h3>{strings.help.resistors.smdTitle}</h3>
  <p>{strings.help.resistors.smdText}</p>

  <h3>{strings.help.resistors.colorTitle}</h3>
  <p>{strings.help.resistors.colorText}</p>

  <h3>{strings.help.resistors.ohmTitle}</h3>
  <p>{strings.help.resistors.ohmText}</p>

  <h3>{strings.help.resistors.connTitle}</h3>
  <p>{strings.help.resistors.connText}</p>

  <h3>{strings.help.resistors.dividerTitle}</h3>
  <p>{strings.help.resistors.dividerText}</p>

  <h3>{strings.help.resistors.pickTitle}</h3>
  <p>{strings.help.resistors.pickText}</p>
</SectionHelpModal>

<style>
  .subtabs { display: flex; gap: var(--space-1); padding-bottom: var(--space-3); margin-bottom: var(--space-4); border-bottom: 1px solid var(--border-subtle); overflow-x: auto; }
  .subtabs__item { min-height: 36px; padding: 0 var(--space-3); border-radius: var(--radius-md); font-size: var(--font-size-sm); color: var(--text-secondary); background: transparent; white-space: nowrap; }
  .subtabs__item.active { background: var(--accent-dim); color: var(--accent); }
  .calc-body { min-height: 100px; }
</style>