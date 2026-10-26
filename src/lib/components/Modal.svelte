<script>
  import { X } from '@lucide/svelte';
  import { getStrings } from '../strings.svelte.js';

  let { open = false, onClose = () => {}, children } = $props();
  const strings = $derived(getStrings());

  function handleBackdrop(e) {
    if (e.target === e.currentTarget) onClose();
  }
</script>

<svelte:window onkeydown={(e) => e.key === 'Escape' && open && onClose()} />

{#if open}
  <div class="modal-backdrop" onclick={handleBackdrop} role="presentation">
    <div class="modal" role="dialog" aria-modal="true" onclick={(e) => e.stopPropagation()}>
      <button type="button" class="modal__close" onclick={onClose} aria-label={strings.nav.close}>
        <X size={18} strokeWidth={2} />
      </button>
      <div class="modal__body">
        {@render children()}
      </div>
    </div>
  </div>
{/if}

<style>
  .modal-backdrop {
    position: fixed;
    inset: 0;
    z-index: 50;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: var(--space-4);
    background: rgba(0, 0, 0, 0.7);
    backdrop-filter: blur(4px);
    animation: fadeIn 180ms ease both;
  }
  .modal {
    position: relative;
    width: 100%;
    max-width: 460px;
    max-height: 90vh;
    overflow-y: auto;
    background: var(--bg-elevated);
    border: 1px solid var(--border-strong);
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-lg);
    padding: var(--space-5);
    animation: scaleIn 200ms cubic-bezier(0.2, 0.6, 0.3, 1) both;
  }
  .modal__close {
    position: absolute;
    top: var(--space-3);
    right: var(--space-3);
    width: 32px;
    height: 32px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: var(--radius-md);
    color: var(--text-muted);
    transition: background var(--transition-fast), color var(--transition-fast);
  }
  .modal__close:hover {
    background: var(--bg-elevated-2);
    color: var(--text-primary);
  }
  .modal__body {
    display: flex;
    flex-direction: column;
  }
  @keyframes fadeIn {
    from { opacity: 0; }
    to { opacity: 1; }
  }
  @keyframes scaleIn {
    from { opacity: 0; transform: scale(0.96) translateY(4px); }
    to { opacity: 1; transform: scale(1) translateY(0); }
  }
  @media (max-width: 768px) {
    .modal-backdrop { padding: var(--space-3); }
    .modal { padding: var(--space-4); }
  }
</style>