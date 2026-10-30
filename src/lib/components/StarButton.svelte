<script>
  import { Star } from '@lucide/svelte';

  let { active = false, onclick = () => {}, title = '' } = $props();
</script>

<button
  type="button"
  class="star-btn"
  class:active
  {title}
  onclick={onclick}
>
  <Star size={16} strokeWidth={2} fill={active ? 'currentColor' : 'none'} />
</button>

<style>
  .star-btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 32px;
    height: 32px;
    border-radius: var(--radius-sm);
    color: var(--text-muted);
    transition: color var(--transition-fast), background var(--transition-fast);
    flex-shrink: 0;
  }
  .star-btn:hover {
    background: var(--bg-elevated-2);
    color: var(--accent);
  }
  .star-btn.active {
    color: var(--accent);
  }
</style>