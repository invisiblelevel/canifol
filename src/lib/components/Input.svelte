<script>
  let {
    value = $bindable(''),
    label = '',
    placeholder = '',
    hint = '',
    error = '',
    type = 'text',
    inputmode = undefined,
    enterkeyhint = undefined,
    suffix = '',
    disabled = false,
    readonly = false,
    computed = false,
    id = '',
    ...rest
  } = $props();

  let inputEl = $state(null);

  const inputId = $derived(id || `inp-${Math.random().toString(36).slice(2, 9)}`);
  const hasError = $derived(!!error);
  const describedBy = $derived(
    [hint ? `${inputId}-hint` : '', error ? `${inputId}-error` : '']
      .filter(Boolean)
      .join(' ') || undefined
  );

  export function focus() { inputEl?.focus(); }
  export function select() { inputEl?.select(); }
  export function el() { return inputEl; }
</script>

<div
  class="field"
  class:field--error={hasError}
  class:field--disabled={disabled}
  class:field--computed={computed}
>
  {#if label}
    <label class="field__label" for={inputId}>
      {label}
      {#if computed}
        <span class="field__computed-badge">вычислено</span>
      {/if}
    </label>
  {/if}

  <div class="field__control" class:has-suffix={!!suffix}>
    <input
      id={inputId}
      class="field__input"
      {type}
      {placeholder}
      {disabled}
      {readonly}
      {inputmode}
      {enterkeyhint}
      autocomplete="off"
      aria-invalid={hasError || undefined}
      aria-describedby={describedBy}
      bind:value
      bind:this={inputEl}
      {...rest}
    />
    {#if suffix}
      <span class="field__suffix" aria-hidden="true">{suffix}</span>
    {/if}
  </div>

  {#if error}
    <p class="field__error" id={`${inputId}-error`}>{error}</p>
  {:else if hint}
    <p class="field__hint" id={`${inputId}-hint`}>{hint}</p>
  {/if}
</div>

<style>
  .field {
    display: flex;
    flex-direction: column;
    gap: 6px;
    width: 100%;
  }

  .field__label {
    font-size: var(--font-size-sm);
    font-weight: 500;
    color: var(--text-secondary);
    display: flex;
    align-items: center;
    gap: 8px;
    letter-spacing: 0.2px;
  }

  .field__computed-badge {
    font-size: 10px;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    color: var(--accent);
    background: var(--accent-dim);
    padding: 2px 6px;
    border-radius: 4px;
    font-weight: 700;
  }

  .field__control {
    position: relative;
    display: flex;
    align-items: center;
  }

  .field__input {
    width: 100%;
    min-height: var(--touch-target);
    padding: 0 14px;
    background: var(--bg-base);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-md);
    color: var(--text-primary);
    font-size: var(--font-size-base);
    font-family: var(--font-mono);
    letter-spacing: 0.3px;
    box-shadow: inset 0 1px 2px rgba(0, 0, 0, 0.1);
    transition:
      border-color var(--transition-fast),
      background var(--transition-fast),
      box-shadow var(--transition-fast);
  }

  .field__input::placeholder {
    color: var(--text-muted);
    font-family: var(--font-sans);
    opacity: 0.7;
    letter-spacing: 0;
  }

  .field__input:hover:not(:disabled):not(:read-only) {
    border-color: var(--border-strong);
  }

  .field__input:focus {
    outline: none;
    border-color: var(--accent);
    box-shadow: 0 0 0 3px var(--accent-dim), inset 0 1px 2px rgba(0, 0, 0, 0.1);
  }

  .field__input:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .field__input:read-only {
    background: var(--bg-elevated);
    cursor: default;
    border-color: transparent;
  }

  .has-suffix .field__input {
    padding-right: calc(var(--space-3) + 3ch);
  }

  .field__suffix {
    position: absolute;
    right: 14px;
    color: var(--text-muted);
    font-size: var(--font-size-sm);
    font-family: var(--font-mono);
    pointer-events: none;
    user-select: none;
  }

  .field__hint {
    font-size: 12px;
    color: var(--text-muted);
    padding-left: 2px;
    line-height: 1.4;
  }

  .field__error {
    font-size: 12px;
    color: var(--danger);
    padding-left: 2px;
  }

  .field--error .field__input {
    border-color: var(--danger);
  }
  .field--error .field__input:focus {
    box-shadow: 0 0 0 3px rgba(232, 90, 90, 0.15), inset 0 1px 2px rgba(0, 0, 0, 0.1);
  }

  /* --- Вычисленное --- */
  .field--computed .field__input {
    background: var(--accent-dim);
    border-color: rgba(245, 184, 61, 0.3);
    color: var(--accent);
    font-weight: 600;
  }
  .field--computed .field__input:focus {
    border-color: var(--accent);
    box-shadow: 0 0 0 3px var(--accent-dim);
  }

  .field__input[type='number']::-webkit-outer-spin-button,
  .field__input[type='number']::-webkit-inner-spin-button {
    -webkit-appearance: none;
    margin: 0;
  }
  .field__input[type='number'] {
    -moz-appearance: textfield;
    appearance: textfield;
  }

  .field__input:-webkit-autofill,
  .field__input:-webkit-autofill:hover,
  .field__input:-webkit-autofill:focus,
  .field__input:-webkit-autofill:active {
    -webkit-text-fill-color: var(--text-primary) !important;
    -webkit-box-shadow: 0 0 0 1000px var(--bg-base) inset !important;
    box-shadow: 0 0 0 1000px var(--bg-base) inset !important;
    transition: background-color 9999s ease-in-out 0s;
    caret-color: var(--text-primary);
  }

  .field--computed .field__input:-webkit-autofill,
  .field--computed .field__input:-webkit-autofill:hover,
  .field--computed .field__input:-webkit-autofill:focus {
    -webkit-text-fill-color: var(--accent) !important;
    -webkit-box-shadow: 0 0 0 1000px var(--accent-dim) inset !important;
    box-shadow: 0 0 0 1000px var(--accent-dim) inset !important;
  }
</style>