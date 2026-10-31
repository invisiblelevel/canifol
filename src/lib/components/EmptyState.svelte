<script>
  let { icon = null, title = '', description = '', children } = $props();
</script>

<div class="empty-state">
  <!-- Фоновая сетка (дорожки платы) -->
  <svg class="empty-state__bg" viewBox="0 0 400 300" preserveAspectRatio="xMidYMid slice" xmlns="http://www.w3.org/2000/svg">
    <defs>
      <pattern id="pcb" width="60" height="60" patternUnits="userSpaceOnUse">
        <path d="M 0 30 L 20 30 L 30 20 L 40 20 L 40 40 L 20 40 L 10 30" fill="none" stroke="currentColor" stroke-width="0.5" opacity="0.3"/>
        <circle cx="40" cy="20" r="1.5" fill="currentColor" opacity="0.4"/>
        <circle cx="10" cy="30" r="1.5" fill="currentColor" opacity="0.4"/>
        <path d="M 60 0 L 60 15 L 50 25 M 0 60 L 15 60 L 25 50" fill="none" stroke="currentColor" stroke-width="0.5" opacity="0.2"/>
      </pattern>
    </defs>
    <rect width="100%" height="100%" fill="url(#pcb)" />
  </svg>

  <div class="empty-state__content">
    {#if icon}
      {@const IconComponent = icon}
      <div class="empty-state__icon">
        <IconComponent size={56} strokeWidth={1.5} />
      </div>
    {:else}
      <div class="empty-state__icon">
        <div class="empty-state__icon-placeholder">?</div>
      </div>
    {/if}
    
    <h3 class="empty-state__title">{title}</h3>
    <p class="empty-state__desc">{description}</p>
    <div class="empty-state__actions">
      {@render children?.()}
    </div>
  </div>
</div>

<style>
  /* ... стили без изменений ... */
  .empty-state {
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 48px 24px;
    text-align: center;
    background: var(--bg-elevated-2);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-lg);
    min-height: 350px;
    overflow: hidden;
    color: var(--accent);
  }

  .empty-state__bg {
    position: absolute;
    inset: 0;
    width: 100%;
    height: 100%;
    pointer-events: none;
    opacity: 0.15;
    color: var(--accent);
  }

  .empty-state__content {
    position: relative;
    z-index: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    max-width: 420px;
  }

  .empty-state__icon {
    margin-bottom: 20px;
    color: var(--accent);
    opacity: 0.9;
    filter: drop-shadow(0 4px 12px rgba(245, 184, 61, 0.2));
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .empty-state__icon-placeholder {
    width: 56px;
    height: 56px;
    font-size: 28px;
    font-weight: 700;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--accent-dim);
    border-radius: 50%;
  }

  .empty-state__title {
    font-size: var(--font-size-xl);
    font-weight: 700;
    color: var(--text-primary);
    margin-bottom: 12px;
    letter-spacing: -0.02em;
  }

  .empty-state__desc {
    font-size: var(--font-size-sm);
    color: var(--text-secondary);
    line-height: 1.6;
    margin-bottom: 24px;
  }

  .empty-state__actions {
    display: flex;
    gap: 12px;
  }
</style>