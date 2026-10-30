<script>
  import Modal from './Modal.svelte';
  import Button from './Button.svelte';
  import Logo from './Logo.svelte';
  import { Heart } from '@lucide/svelte';
  import { getStrings } from '../strings.svelte.js';

  let { open = false, onClose = () => {}, onOpenSupport = () => {} } = $props();
  const strings = $derived(getStrings());
  const APP_VERSION = '1.3.0';
</script>

<Modal {open} {onClose}>
  <div class="about">
    <div class="about__header">
      <Logo size={56} />
      <div>
        <h1 class="about__title">{strings.app.name}</h1>
        <div class="about__version">
          {strings.about.version} {APP_VERSION}
        </div>
      </div>
    </div>

    <p class="about__desc">{strings.about.description}</p>

    <div class="about__divider"></div>

    <Button variant="secondary" fullWidth onclick={onOpenSupport}>
      <Heart size={16} strokeWidth={2} />
      {strings.about.supportButton}
    </Button>

    <div class="about__divider"></div>

    <div class="about__footer">
      <p>{strings.about.license}</p>
      <p>{strings.about.copyright}</p>
    </div>
  </div>
</Modal>

<style>
  .about {
    display: flex;
    flex-direction: column;
    padding-right: var(--space-4);
  }
  .about__header {
    display: flex;
    align-items: center;
    gap: var(--space-3);
    color: var(--accent);
    margin-bottom: var(--space-4);
  }
  .about__title {
    font-size: var(--font-size-2xl);
    font-weight: 700;
    letter-spacing: -0.02em;
  }
  .about__version {
    font-family: var(--font-mono);
    font-size: var(--font-size-sm);
    color: var(--text-muted);
    margin-top: 2px;
  }
  .about__desc {
    font-size: var(--font-size-base);
    color: var(--text-secondary);
    line-height: 1.5;
  }
  .about__divider {
    height: 1px;
    background: var(--border-subtle);
    margin: var(--space-4) 0;
  }
  .about__footer {
    display: flex;
    flex-direction: column;
    gap: 4px;
    text-align: center;
  }
  .about__footer p {
    font-size: var(--font-size-sm);
    color: var(--text-muted);
  }
</style>