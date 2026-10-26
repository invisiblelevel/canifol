<script>
  let { children, title = '', actions = null } = $props();
</script>

<section class="card">
  {#if title || actions}
    <header class="card__head">
      {#if title}
        <h2 class="card__title">{title}</h2>
      {:else}
        <span></span>
      {/if}
      {#if actions}
        <div class="card__actions">
          {@render actions()}
        </div>
      {/if}
    </header>
  {/if}
  <div class="card__body">
    {@render children()}
  </div>
</section>

<style>
  .card {
    background: var(--bg-elevated);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-md);
    overflow: hidden;
  }
  
  .card__head {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--space-3);
    padding: 16px 20px;
    background: linear-gradient(180deg, var(--bg-elevated-2) 0%, var(--bg-elevated) 100%);
    border-bottom: 1px solid var(--border-subtle);
  }
  
  .card__title {
    font-size: var(--font-size-lg);
    font-weight: 700;
    letter-spacing: -0.01em;
    color: var(--text-primary);
    flex: 1;
    margin: 0;
  }
  
  .card__actions {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    flex-shrink: 0;
  }
  
  .card__body {
    padding: 20px;
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
  }
  
  @media (max-width: 768px) {
    .card__body { padding: 16px; gap: var(--space-3); }
    .card__head { padding: 12px 16px; }
    .card__title { font-size: var(--font-size-base); }
  }
</style>