<script>
  import Modal from './Modal.svelte';

  let { open = false, onClose = () => {}, title = '', children } = $props();
</script>

<Modal {open} {onClose}>
  <div class="section-help">
    <h1 class="section-help__title">{title}</h1>
    <div class="section-help__body">
      {@render children()}
    </div>
  </div>
</Modal>

<style>
  .section-help {
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
    padding-right: var(--space-4);
  }
  .section-help__title {
    font-size: var(--font-size-xl);
    color: var(--accent);
    padding-bottom: var(--space-2);
    border-bottom: 1px solid var(--border-subtle);
  }
  .section-help__body {
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
    font-size: var(--font-size-base);
    color: var(--text-secondary);
    line-height: 1.55;
  }
  .section-help__body :global(h3) {
    font-size: var(--font-size-base);
    font-weight: 600;
    color: var(--text-primary);
    margin-top: var(--space-2);
  }
  .section-help__body :global(ul) {
    padding-left: var(--space-4);
    display: flex;
    flex-direction: column;
    gap: 4px;
  }
  .section-help__body :global(code) {
    font-family: var(--font-mono);
    font-size: var(--font-size-sm);
    background: var(--bg-elevated-2);
    padding: 2px 6px;
    border-radius: 4px;
    color: var(--accent);
  }
  .section-help__body :global(strong) {
    color: var(--text-primary);
  }
</style>