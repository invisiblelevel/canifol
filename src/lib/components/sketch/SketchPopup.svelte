<script>
  import { Check, X, ChevronDown } from '@lucide/svelte';
  import { SYMBOLS } from '../../utils/sketch/symbols.js';
  import { getStrings } from '../../strings.svelte.js';

  const strings = $derived(getStrings());

  let {
    x = 0, y = 0, flip = false, wrapW = 1200, wrapH = 800,
    suggestedType = null, confidence = 0, onConfirm, onCancel,
  } = $props();

  let showMore = $state(false);
  let selectedPinCount = $state(8);

  const TYPES_WITH_PINS = ['microchip'];
  const POPUP_W = 320;
  const POPUP_H = 140;
  const MARGIN = 8;

  function pick(type, opts = {}) {
    if (TYPES_WITH_PINS.includes(type) && !opts.pinCount) {
      onConfirm({ type, opts: { pinCount: selectedPinCount } });
    } else {
      onConfirm({ type, opts });
    }
  }

  function getLabel(type) {
    return SYMBOLS[type]?.label || type;
  }

  let displayType = $derived(
    suggestedType === 'circle' ? 'transistor_npn'
    : suggestedType === 'rectangle' ? 'microchip'
    : suggestedType
  );

  let left = $derived(Math.max(MARGIN, Math.min(wrapW - POPUP_W - MARGIN, x - POPUP_W / 2)));
  let top = $derived(flip ? Math.min(wrapH - POPUP_H - MARGIN, y + 16) : Math.max(MARGIN, y - POPUP_H - 16));
  
  let headerText = $derived(
    suggestedType === 'circle' ? (strings?.sketch?.drawnCircle || 'Drawn: circle')
    : suggestedType === 'rectangle' ? (strings?.sketch?.drawnRectangle || 'Drawn: rectangle')
    : displayType ? (strings?.sketch?.drawnItem || 'Drawn: {item}?').replace('{item}', getLabel(displayType))
    : (strings?.sketch?.notRecognized || 'Not recognized')
  );
</script>

<div class="popup" style="left: {left}px; top: {top}px;">
  {#if displayType}
    <div class="header">{headerText}</div>

    <div class="actions">
      {#if suggestedType === 'circle'}
        <button onclick={() => pick('transistor_npn')}>NPN</button>
        <button onclick={() => pick('transistor_pnp')}>PNP</button>
        <button onclick={() => pick('vcc')}>VCC</button>
      {:else if suggestedType === 'rectangle'}
        <div class="pin-count">
          <label>{strings?.sketch?.pinCount || 'Pin count:'}</label>
          <select bind:value={selectedPinCount}>
            <option value={4}>4</option>
            <option value={6}>6</option>
            <option value={8}>8</option>
            <option value={14}>14</option>
            <option value={16}>16</option>
            <option value={20}>20</option>
          </select>
        </div>
        <button class="primary" onclick={() => pick('microchip', { pinCount: selectedPinCount })}>
          <Check size={14} /> {strings?.sketch?.btnMicrochip || 'Microchip'}
        </button>
      {:else}
        <button class="primary" onclick={() => pick(displayType)}>
          <Check size={14} /> {getLabel(displayType)}
        </button>
      {/if}

      <button class="more" onclick={() => showMore = !showMore}>
        {strings?.sketch?.btnOther || 'Other'} <ChevronDown size={14} />
      </button>
      <button class="cancel" onclick={onCancel}>
        <X size={14} />
      </button>
    </div>

    {#if showMore}
      <div class="more-list">
        {#each Object.keys(SYMBOLS) as type}
          {#if !TYPES_WITH_PINS.includes(type) || type === 'microchip'}
            <button onclick={() => pick(type)}>{getLabel(type)}</button>
          {/if}
        {/each}
      </div>
    {/if}
  {:else}
    <div class="header">{strings?.sketch?.notRecognized || 'Not recognized'}</div>
    <div class="actions">
      <button onclick={onCancel}>{strings?.sketch?.btnCancel || 'Leave as is'}</button>
    </div>
  {/if}
</div>

<style>
  .popup {
    position: absolute;
    background: var(--bg-elevated-2);
    color: var(--text-primary);
    padding: 8px 10px;
    border-radius: var(--radius-md);
    border: 1px solid var(--border-strong);
    box-shadow: var(--shadow-lg);
    font-size: 12px;
    z-index: 100;
    min-width: 180px;
    max-width: 320px;
    animation: popIn 0.15s ease-out;
  }
  @keyframes popIn {
    from { opacity: 0; transform: scale(0.96); }
    to   { opacity: 1; transform: scale(1); }
  }
  .header { font-weight: 600; margin-bottom: 6px; color: var(--accent); }
  .actions { display: flex; gap: 4px; flex-wrap: wrap; align-items: center; }
  .actions button {
    display: flex; align-items: center; gap: 4px;
    padding: 4px 10px;
    background: var(--bg-elevated-3);
    color: var(--text-primary);
    border: 1px solid var(--border-strong);
    border-radius: 4px;
    font-size: 11px;
    cursor: pointer;
    transition: background 0.15s;
  }
  .actions button:hover { background: var(--bg-elevated-2); }
  .actions button.primary { background: var(--accent); color: #000; border-color: var(--accent); }
  .actions button.primary:hover { background: var(--accent-strong); }
  .actions button.cancel { padding: 4px 6px; }
  .pin-count { display: flex; align-items: center; gap: 6px; width: 100%; margin-bottom: 4px; }
  .pin-count select {
    background: var(--bg-elevated-3); color: var(--text-primary);
    border: 1px solid var(--border-strong); border-radius: 4px;
    padding: 2px 6px; font-size: 11px;
  }
  .more-list {
    margin-top: 6px;
    display: flex;
    flex-wrap: wrap;
    gap: 4px;
    max-height: 150px;
    overflow-y: auto;
  }
  .more-list button {
    padding: 3px 8px;
    background: var(--bg-elevated-3);
    color: var(--text-secondary);
    border: 1px solid var(--border-subtle);
    border-radius: 3px;
    font-size: 10px;
    cursor: pointer;
  }
  .more-list button:hover { background: var(--bg-elevated-2); color: var(--text-primary); }
</style>