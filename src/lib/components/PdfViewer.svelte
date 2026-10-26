<script>
  import PdfViewerLib from 'svelte-pdf-view';

  let { src = '' } = $props();
</script>

<div class="pdf-container">
  <PdfViewerLib url={src} />
</div>

<style>
  .pdf-container {
    width: 100%;
    height: 100%;
    min-height: 500px;
    background: var(--bg-elevated);
    border-radius: var(--radius-md);
    overflow: hidden;
  }
</style>