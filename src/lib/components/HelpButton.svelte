<script>
  import { HelpCircle } from '@lucide/svelte';
  import { getStrings } from '../strings.svelte.js';

  let { onclick = () => {} } = $props();
  const strings = $derived(getStrings());
  const label = $derived(strings.common.help);
</script>

<button
  type="button"
  class="help-btn"
  aria-label={label}
  title={label}
  {onclick}
>
  <HelpCircle size={18} strokeWidth={2} />
</button>

<style>
  .help-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 32px;
    height: 32px;
    border-radius: var(--radius-md);
    color: var(--text-muted);
    background: transparent;
    transition: background var(--transition-fast), color var(--transition-fast);
  }
  .help-btn:hover {
    background: var(--bg-elevated-2);
    color: var(--accent);
  }
</style>