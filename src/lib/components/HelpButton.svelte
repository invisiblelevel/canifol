<script>
  import { HelpCircle } from '@lucide/svelte';

  let { onclick = () => {} } = $props();
</script>

<button
  type="button"
  class="help-btn"
  aria-label="Помощь по разделу"
  title="Помощь по разделу"
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