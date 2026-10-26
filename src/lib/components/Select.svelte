<script>
  let {
    value = $bindable(''),
    options = [],
    label = '',
    hint = '',
    error = '',
    placeholder = '',
    disabled = false,
    compact = false,
    id = '',
    ...rest
  } = $props();

  const selectId = $derived(id || `sel-${Math.random().toString(36).slice(2, 9)}`);
  const hasError = $derived(!!error);
  const describedBy = $derived(
    [hint ? `${selectId}-hint` : '', error ? `${selectId}-error` : '']
      .filter(Boolean)
      .join(' ') || undefined
  );
</script>

<div class="field" class:field--error={hasError} class:field--compact={compact}>
  {#if label && !compact}
    <label class="field__label" for={selectId}>{label}</label>
  {/if}

  <div class="field__control">
    <select
      id={selectId}
      class="field__select"
      class:placeholder={!value && !!placeholder}
      {disabled}
      aria-label={compact ? label || undefined : undefined}
      aria-invalid={hasError || undefined}
      aria-describedby={describedBy}
      bind:value
      {...rest}
    >
      {#if placeholder}
        <option value="" disabled>{placeholder}</option>
      {/if}
      {#each options as opt (opt.value)}
        <option value={opt.value} disabled={opt.disabled || false}>
          {opt.label}
        </option>
      {/each}
    </select>
    <span class="field__chevron" aria-hidden="true">
      <svg width="12" height="8" viewBox="0 0 12 8" fill="none">
        <path d="M1 1.5L6 6.5L11 1.5" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"/>
      </svg>
    </span>
  </div>

  {#if !compact}
    {#if error}
      <p class="field__error" id={`${selectId}-error`}>{error}</p>
    {:else if hint}
      <p class="field__hint" id={`${selectId}-hint`}>{hint}</p>
    {/if}
  {/if}
</div>

<style>
  .field {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
    width: 100%;
  }

  .field--compact {
    width: auto;
    flex-shrink: 0;
  }

  .field__label {
    font-size: var(--font-size-sm);
    font-weight: 500;
    color: var(--text-secondary);
  }

  .field__control {
    position: relative;
    display: flex;
    align-items: center;
  }

  .field__select {
    width: 100%;
    min-height: var(--touch-target);
    padding: 0 calc(var(--space-3) + 24px) 0 var(--space-3);
    background: var(--bg-elevated-2);
    border: 1px solid var(--border-strong);
    border-radius: var(--radius-md);
    color: var(--text-primary);
    font-size: var(--font-size-base);
    cursor: pointer;
    box-shadow: inset 0 1px 2px rgba(0, 0, 0, 0.15);
    transition:
      border-color var(--transition-fast),
      background var(--transition-fast),
      box-shadow var(--transition-fast);
    appearance: none;
    -webkit-appearance: none;
  }

  .field--compact .field__select {
    min-height: var(--touch-target);
    min-width: 70px;
    padding: 0 calc(var(--space-2) + 20px) 0 var(--space-2);
    font-size: var(--font-size-sm);
  }

  .field__select.placeholder {
    color: var(--text-muted);
  }

  .field__select:hover:not(:disabled) {
    border-color: var(--text-muted);
  }

  .field__select:focus {
    outline: none;
    border-color: var(--accent);
    box-shadow: 0 0 0 3px var(--accent-dim), inset 0 1px 2px rgba(0, 0, 0, 0.15);
  }

  .field__select:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .field__select option {
    background: var(--bg-elevated-2);
    color: var(--text-primary);
  }

  .field__chevron {
    position: absolute;
    right: var(--space-3);
    display: inline-flex;
    align-items: center;
    color: var(--text-muted);
    pointer-events: none;
  }

  .field--compact .field__chevron {
    right: var(--space-2);
  }

  .field__hint {
    font-size: var(--font-size-sm);
    color: var(--text-muted);
  }

  .field__error {
    font-size: var(--font-size-sm);
    color: var(--danger);
  }

  .field--error .field__select {
    border-color: var(--danger);
  }
  .field--error .field__select:focus {
    box-shadow: 0 0 0 3px rgba(242, 106, 106, 0.2), inset 0 1px 2px rgba(0, 0, 0, 0.15);
  }
</style>