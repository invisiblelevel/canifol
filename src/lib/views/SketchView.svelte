<script>
  import { onMount } from 'svelte';
  import Card from '../components/Card.svelte';
  import HelpButton from '../components/HelpButton.svelte';
  import SectionHelpModal from '../components/SectionHelpModal.svelte';
  import SketchCanvas from '../components/sketch/SketchCanvas.svelte';
  import SketchToolbar from '../components/sketch/SketchToolbar.svelte';
  import { getStrings } from '../strings.svelte.js';

  const strings = $derived(getStrings());

  let canvasRef;
  let containerRef;
  let mode = $state('select');
  let activeStamp = $state(null);
  let canUndo = $state(false);
  let canRedo = $state(false);
  let hasSelection = $state(false);
  let hasBg = $state(false);
  let showHelp = $state(false);
  let isFullscreen = $state(false);

  onMount(() => {
    if (containerRef) containerRef.focus();
  });

  function refreshFlags() {
    if (!canvasRef) return;
    canUndo = canvasRef.getCanUndo?.() ?? false;
    canRedo = canvasRef.getCanRedo?.() ?? false;
    hasSelection = canvasRef.getHasSelection?.() ?? false;
    hasBg = canvasRef.hasBackground?.() ?? false;
  }

  function onModeChange(m) { 
    mode = m; 
    if (m !== 'select') activeStamp = null; 
    refreshFlags(); 
  }
  
  function onUndo() { canvasRef?.undo_?.(); refreshFlags(); }
  function onRedo() { canvasRef?.redo_?.(); refreshFlags(); }
  function onRotate() { canvasRef?.rotate_?.(); refreshFlags(); }
  function onClear() { canvasRef?.clearAll?.(); refreshFlags(); }
  async function onExport() { await canvasRef?.exportPNG_?.(); }
  function onZoomIn() { canvasRef?.zoomIn_?.(); }
  function onZoomOut() { canvasRef?.zoomOut_?.(); }

  function onToggleFullscreen() {
    isFullscreen = !isFullscreen;
  }

  function onLoadBackground() {
    canvasRef?.loadBackgroundFromFile?.();
    // hasBg обновится через onBgChange
  }

  function onRemoveBackground() {
    canvasRef?.removeBackground?.();
    // hasBg обновится через onBgChange
  }

  function handleBgChange(value) {
    hasBg = value;
  }

  function handleContainerClick() {
    if (containerRef) containerRef.focus();
  }

  function handleKeydown(e) {
    if (e.target.tagName === 'INPUT' || e.target.tagName === 'TEXTAREA') return;

    if (e.ctrlKey || e.metaKey) {
      if (e.code === 'KeyZ') { e.preventDefault(); canvasRef?.undo_?.(); refreshFlags(); }
      else if (e.code === 'KeyY' || (e.shiftKey && e.code === 'KeyZ')) { e.preventDefault(); canvasRef?.redo_?.(); refreshFlags(); }
      return;
    }

    switch (e.code) {
      case 'KeyV': e.preventDefault(); onModeChange('select'); break;
      case 'KeyP': e.preventDefault(); onModeChange('pencil'); break;
      case 'KeyW': e.preventDefault(); onModeChange('wire'); break;
      case 'KeyO': e.preventDefault(); onModeChange('point'); break;
      case 'KeyT': e.preventDefault(); onModeChange('text'); break;
      case 'KeyE': e.preventDefault(); onModeChange('eraser'); break;
      case 'KeyR': e.preventDefault(); canvasRef?.rotate_?.(); refreshFlags(); break;
      case 'Delete':
      case 'Backspace':
        e.preventDefault();
        break;
      case 'Escape':
        activeStamp = null;
        break;
      case 'Equal':
      case 'NumpadAdd':
        e.preventDefault();
        canvasRef?.zoomIn_?.();
        break;
      case 'Minus':
      case 'NumpadSubtract':
        e.preventDefault();
        canvasRef?.zoomOut_?.();
        break;
    }
  }
</script>

<div 
  class="sketch-layout" 
  class:fullscreen={isFullscreen}
  bind:this={containerRef}
  tabindex="0"
  onkeydown={handleKeydown}
  onmousedown={handleContainerClick}
>
  <SketchToolbar
    {mode}
    bind:activeStamp
    {canUndo}
    {canRedo}
    {hasSelection}
    {hasBg}
    {isFullscreen}
    strings={strings}
    onModeChange={onModeChange}
    onUndo={onUndo}
    onRedo={onRedo}
    onRotate={onRotate}
    onClear={onClear}
    onExport={onExport}
    onZoomIn={onZoomIn}
    onZoomOut={onZoomOut}
    onToggleFullscreen={onToggleFullscreen}
    onShowHelp={() => { showHelp = true; }}
    onLoadBackground={onLoadBackground}
    onRemoveBackground={onRemoveBackground}
  />
  <div class="canvas-container">
    <SketchCanvas 
      bind:this={canvasRef} 
      bind:mode 
      bind:activeStamp 
      onBgChange={handleBgChange}
    />
  </div>
</div>

<SectionHelpModal open={showHelp} onClose={() => showHelp = false} title={strings?.sketch?.helpTitle || 'Набросок'}>
  <p>{strings?.sketch?.helpDesc || 'Рисуй схемы как карандашом на бумаге.'}</p>
  <h3>{strings?.sketch?.helpKeys || 'Горячие клавиши'}</h3>
  <ul>
    <li><b>V</b> — {strings?.sketch?.helpV || ''}</li>
    <li><b>P</b> — {strings?.sketch?.helpP || ''}</li>
    <li><b>W</b> — {strings?.sketch?.helpW || ''}</li>
    <li><b>O</b> — {strings?.sketch?.helpO || ''}</li>
    <li><b>T</b> — {strings?.sketch?.helpT || ''}</li>
    <li><b>E</b> — {strings?.sketch?.helpE || ''}</li>
    <li><b>R</b> — {strings?.sketch?.helpR || ''}</li>
    <li><b>Del</b> — {strings?.sketch?.helpDel || ''}</li>
    <li><b>Ctrl+Z</b> / <b>Ctrl+Y</b> — {strings?.sketch?.helpUndo || ''}</li>
  </ul>
  <h3>Управление холстом</h3>
  <p>{strings?.sketch?.helpZoom || 'Зум: колёсико мыши или + / − на клавиатуре.'}</p>
  <p>{strings?.sketch?.helpPan || 'Панорамирование: Shift + левая кнопка мыши или средняя кнопка мыши.'}</p>
  <p>{strings?.sketch?.helpMobile || 'На телефоне: рисуй одним пальцем, масштабируй и двигай холст двумя пальцами.'}</p>
  <h3>Штампы</h3>
  <p>{strings?.sketch?.helpStamps || 'Штампы: выбери элемент из палитры и кликни на холст, чтобы поставить его.'}</p>
  <h3>Фон (фото платы)</h3>
  <p>Загрузи фото платы через кнопку с иконкой картинки. В режиме «Выбрать» можно перетаскивать фон. Ctrl + колесо — масштабирование фона.</p>
  <h3>Автоматическое распознавание</h3>
  <p>{strings?.sketch?.helpMagnet || 'Провода автоматически цепляются к поинтам.'}</p>
</SectionHelpModal>

<style>
  .sketch-layout {
    display: flex;
    flex-direction: column;
    height: 100%;
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-md);
    overflow: hidden;
    position: relative;
    background: var(--bg-elevated);
    outline: none;
  }
  .sketch-layout.fullscreen {
    position: fixed;
    top: 0; left: 0; right: 0; bottom: 0;
    z-index: 9999;
    height: 100vh;
    height: 100dvh;
    border-radius: 0;
    border: none;
  }
  .canvas-container {
    flex: 1;
    min-height: 0;
    position: relative;
    overflow: hidden;
  }
</style>