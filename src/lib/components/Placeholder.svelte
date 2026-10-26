<script>
  import { getStrings } from '../strings.svelte.js';
</script>

<div class="placeholder">
  <p>{strings.common.comingSoon}</p>
</div>

<style>
  .placeholder {
    padding: var(--space-4);
    color: var(--text-muted);
    font-size: var(--font-size-sm);
    text-align: center;
  }
</style>