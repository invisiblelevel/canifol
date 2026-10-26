<script>
  let {
    variant = 'secondary',
    size = 'md',
    type = 'button',
    disabled = false,
    loading = false,
    fullWidth = false,
    children,
    ...rest
  } = $props();

  const isDisabled = $derived(disabled || loading);
</script>

<button
  {type}
  class="btn"
  class:btn--primary={variant === 'primary'}
  class:btn--secondary={variant === 'secondary'}
  class:btn--ghost={variant === 'ghost'}
  class:btn--danger={variant === 'danger'}
  class:btn--sm={size === 'sm'}
  class:btn--md={size === 'md'}
  class:btn--lg={size === 'lg'}
  class:btn--full={fullWidth}
  class:btn--loading={loading}
  disabled={isDisabled}
  aria-busy={loading}
  {...rest}
>
  {#if loading}
    <span class="btn__spinner" aria-hidden="true"></span>
  {/if}
  <span class="btn__label">{@render children?.()}</span>
</button>

<style>
  .btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: var(--space-2);
    min-height: var(--touch-target);
    padding: 0 var(--space-4);
    border-radius: var(--radius-md);
    border: 1px solid transparent;
    font-size: var(--font-size-base);
    font-weight: 500;
    line-height: 1;
    white-space: nowrap;
    user-select: none;
    transition:
      background var(--transition-fast),
      border-color var(--transition-fast),
      color var(--transition-fast),
      box-shadow var(--transition-fast),
      transform var(--transition-fast);
  }

  .btn:focus-visible {
    outline: 2px solid var(--accent);
    outline-offset: 2px;
  }

  .btn:active:not(:disabled) {
    transform: translateY(1px) scale(0.99);
  }

  .btn:disabled {
    opacity: 0.45;
    cursor: not-allowed;
  }

  /* --- Primary (градиент + глоу при ховере) --- */
  .btn--primary {
    background: var(--grad-primary);
    color: #061019;
    border-color: transparent;
    box-shadow: 0 1px 2px rgba(0, 0, 0, 0.2), 0 0 0 1px rgba(94, 200, 245, 0.3) inset;
  }
  .btn--primary:hover:not(:disabled) {
    box-shadow: 0 2px 8px rgba(94, 200, 245, 0.3), 0 0 0 1px rgba(94, 200, 245, 0.5) inset;
  }

  /* --- Secondary --- */
  .btn--secondary {
    background: var(--bg-elevated-2);
    color: var(--text-primary);
    border-color: var(--border-strong);
    box-shadow: var(--shadow-sm);
  }
  .btn--secondary:hover:not(:disabled) {
    background: var(--bg-elevated-3);
    border-color: var(--text-muted);
  }

  /* --- Ghost --- */
  .btn--ghost {
    background: transparent;
    color: var(--text-secondary);
    border-color: transparent;
  }
  .btn--ghost:hover:not(:disabled) {
    background: var(--bg-elevated-2);
    color: var(--text-primary);
  }

  /* --- Danger --- */
  .btn--danger {
    background: var(--danger);
    color: #fff;
    border-color: var(--danger);
    box-shadow: 0 1px 2px rgba(0, 0, 0, 0.2);
  }
  .btn--danger:hover:not(:disabled) {
    filter: brightness(1.08);
    box-shadow: 0 2px 8px rgba(242, 106, 106, 0.3);
  }

  /* --- Sizes --- */
  .btn--sm {
    min-height: 34px;
    padding: 0 var(--space-3);
    font-size: var(--font-size-sm);
  }
  .btn--lg {
    min-height: 52px;
    padding: 0 var(--space-5);
    font-size: var(--font-size-lg);
  }

  .btn--full {
    width: 100%;
  }

  .btn__spinner {
    width: 16px;
    height: 16px;
    border: 2px solid currentColor;
    border-right-color: transparent;
    border-radius: 50%;
    animation: btn-spin 0.7s linear infinite;
  }

  .btn__label {
    display: inline-flex;
    align-items: center;
  }

  @keyframes btn-spin {
    to { transform: rotate(360deg); }
  }
</style>