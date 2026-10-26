<script>
  import Modal from './Modal.svelte';
  import { getStrings } from '../strings.svelte.js';

  let { open = false, onClose = () => {} } = $props();
  const strings = $derived(getStrings());

  let copiedBtc = $state(false);
  let copiedUsdt = $state(false);

  const BTC = 'bc1q2ka70s4vtmrskandqj8l4d6n3kdxyxa7kf3wf7';
  const USDT = 'TUjY9p6oxKmeCQwNZwMfHqHdXuabaHpgT7';

  async function copy(value, type) {
    try {
      await navigator.clipboard.writeText(value);
      if (type === 'btc') {
        copiedBtc = true;
        setTimeout(() => (copiedBtc = false), 2000);
      } else {
        copiedUsdt = true;
        setTimeout(() => (copiedUsdt = false), 2000);
      }
    } catch {}
  }
</script>

<Modal {open} {onClose}>
  <div class="support">
    <h1 class="support__title">{strings.support.title}</h1>
    <p class="support__subtitle">{strings.support.subtitle}</p>

    <div class="support__list">
      <div class="wallet">
        <div class="wallet__label">BTC</div>
        <div class="wallet__row">
          <input type="text" readonly value={BTC} class="wallet__input" />
          <button class="wallet__btn" onclick={() => copy(BTC, 'btc')}>
            {copiedBtc ? strings.support.copied : strings.support.copy}
          </button>
        </div>
      </div>

      <div class="wallet">
        <div class="wallet__label">USDT (TRC-20)</div>
        <div class="wallet__row">
          <input type="text" readonly value={USDT} class="wallet__input" />
          <button class="wallet__btn" onclick={() => copy(USDT, 'usdt')}>
            {copiedUsdt ? strings.support.copied : strings.support.copy}
          </button>
        </div>
      </div>
    </div>

    <div class="support__divider"></div>

    <p class="support__thanks">{strings.support.thanks}</p>
  </div>
</Modal>

<style>
  .support {
    display: flex;
    flex-direction: column;
    padding-right: var(--space-4);
  }
  .support__title {
    font-size: var(--font-size-lg);
    font-weight: 700;
    color: var(--text-primary);
    margin-bottom: 4px;
  }
  .support__subtitle {
    font-size: var(--font-size-sm);
    color: var(--text-muted);
    font-style: italic;
    margin-bottom: var(--space-5);
  }
  .support__list {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
  }
  .wallet__label {
    font-size: var(--font-size-sm);
    color: var(--text-secondary);
    margin-bottom: var(--space-2);
  }
  .wallet__row {
    display: flex;
    gap: var(--space-2);
  }
  .wallet__input {
    flex: 1;
    min-width: 0;
    padding: 8px 12px;
    background: var(--bg-base);
    border: 1px solid var(--border-strong);
    border-radius: var(--radius-sm);
    color: var(--text-primary);
    font-family: var(--font-mono);
    font-size: var(--font-size-sm);
    outline: none;
  }
  .wallet__input:focus {
    border-color: var(--accent);
  }
  .wallet__btn {
    padding: 0 var(--space-4);
    background: var(--warning);
    color: #000;
    font-weight: 600;
    font-size: var(--font-size-sm);
    border-radius: var(--radius-sm);
    white-space: nowrap;
    transition: filter var(--transition-fast);
  }
  .wallet__btn:hover {
    filter: brightness(1.1);
  }
  .support__divider {
    height: 1px;
    background: var(--border-subtle);
    margin: var(--space-5) 0;
  }
  .support__thanks {
    text-align: center;
    font-size: var(--font-size-sm);
    color: var(--text-muted);
  }
  @media (max-width: 500px) {
    .wallet__row { flex-direction: column; }
    .wallet__btn { padding: 10px; }
  }
</style>