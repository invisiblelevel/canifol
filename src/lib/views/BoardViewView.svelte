<script>
  import { open } from '@tauri-apps/plugin-dialog';
  import { invoke } from '@tauri-apps/api/core';
  import BoardViewer from '../components/BoardViewer.svelte';
  import Card from '../components/Card.svelte';
  import HelpButton from '../components/HelpButton.svelte';
  import SectionHelpModal from '../components/SectionHelpModal.svelte';
  import { getStrings } from '../strings.svelte.js';

  const strings = $derived(getStrings());
  let filePath = $state('');
  let boardData = $state(null);
  let errorMsg = $state('');
  let showHelp = $state(false);
  let isAndroid = /Android/i.test(navigator.userAgent);

  async function openFile() {
    errorMsg = '';
    let selected;
    if (isAndroid) {
      selected = await open({ multiple: false });
    } else {
      selected = await open({
        multiple: false,
        filters: [{ name: 'BoardView', extensions: ['fz', 'cae', 'cad', 'bdv', 'asc'] }]
      });
    }
    if (!selected) return;

    if (isAndroid) {
      try {
        boardData = await invoke('read_boardview_uri', { uri: selected });
        filePath = selected;
      } catch (e) {
        errorMsg = String(e);
        boardData = null;
        filePath = selected;
      }
    } else {
      filePath = selected;
      boardData = null;
    }
  }
</script>

<Card title={strings.sections.boardview}>
  {#snippet actions()}
    <HelpButton onclick={() => showHelp = true} />
  {/snippet}

  <div class="controls">
    <button class="btn-open" onclick={openFile}>
      {strings.boardview.openFile}
    </button>
    {#if filePath}
      <span class="file-path">{filePath}</span>
    {/if}
  </div>

  {#if errorMsg}
    <div class="error-box">Ошибка: {errorMsg}</div>
  {/if}

  {#if boardData}
    <BoardViewer data={boardData} />
  {:else if filePath && !errorMsg}
    <BoardViewer path={filePath} />
  {:else if !filePath}
    <div class="placeholder">{strings.boardview.noFile}</div>
  {/if}
</Card>

<SectionHelpModal open={showHelp} onClose={() => showHelp = false} title={strings.boardview.helpTitle}>
  <h3>{strings.boardview.helpFormatsTitle}</h3>
  <ul>
    {#each strings.boardview.helpFormats as fmt}
      <li><strong>{fmt.name}:</strong> {fmt.desc}</li>
    {/each}
  </ul>
  <h3>{strings.boardview.helpFeaturesTitle}</h3>
  <ul>
    {#each strings.boardview.helpFeatures as feat}
      <li>{feat}</li>
    {/each}
  </ul>
</SectionHelpModal>

<style>
  .controls { display: flex; align-items: center; gap: 12px; margin-bottom: 16px; flex-wrap: wrap; }
  .btn-open { padding: 8px 16px; background: var(--accent-dim); color: var(--accent); border-radius: var(--radius-md); font-weight: 600; transition: background var(--transition-fast); }
  .btn-open:hover { background: var(--accent); color: #000; }
  .file-path { font-family: var(--font-mono); font-size: var(--font-size-sm); color: var(--text-muted); word-break: break-all; }
  .placeholder { padding: 40px; text-align: center; color: var(--text-muted); border: 1px dashed var(--border-subtle); border-radius: var(--radius-md); }
  .error-box { padding: 16px; background: rgba(239, 83, 80, 0.1); border: 1px solid rgba(239, 83, 80, 0.3); border-radius: var(--radius-md); color: #ff4444; font-family: var(--font-mono); font-size: 12px; word-break: break-all; }
</style>