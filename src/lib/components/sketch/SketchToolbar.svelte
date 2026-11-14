<script>
  import { Pencil, MousePointer2, Spline, Circle, Type, Undo2, Redo2, Trash2, Download, Eraser, RotateCw, ZoomIn, ZoomOut, Maximize, Minimize, Box, HelpCircle, Image, ImageOff } from '@lucide/svelte';
  import { SYMBOLS } from '../../utils/sketch/symbols.js';

  let {
    mode = 'select',
    canUndo = false,
    canRedo = false,
    hasSelection = false,
    isFullscreen = false,
    hasBg = false,
    activeStamp = $bindable(null),
    strings = {},
    onModeChange = () => {},
    onUndo = () => {},
    onRedo = () => {},
    onClear = () => {},
    onExport = () => {},
    onRotate = () => {},
    onZoomIn = () => {},
    onZoomOut = () => {},
    onToggleFullscreen = () => {},
    onShowHelp = () => {},
    onLoadBackground = () => {},
    onRemoveBackground = () => {},
  } = $props();

  let showStamps = $state(false);

  const modes = $derived([
    { id: 'select', icon: MousePointer2, title: strings?.sketch?.toolSelect || 'Select' },
    { id: 'pencil', icon: Pencil, title: strings?.sketch?.toolPencil || 'Pencil' },
    { id: 'wire', icon: Spline, title: strings?.sketch?.toolWire || 'Wire' },
    { id: 'point', icon: Circle, title: strings?.sketch?.toolPoint || 'Point' },
    { id: 'text', icon: Type, title: strings?.sketch?.toolText || 'Text' },
    { id: 'eraser', icon: Eraser, title: strings?.sketch?.toolEraser || 'Eraser' },
  ]);

  function selectStamp(type) {
    activeStamp = type;
    onModeChange('select');
    showStamps = false;
  }

  function pickMode(m) {
    activeStamp = null;
    onModeChange(m);
  }
</script>

<div class="toolbar">
  <div class="group">
    {#each modes as m}
      {@const Icon = m.icon}
      <button
        class="tool-btn"
        class:active={mode === m.id && !activeStamp}
        onclick={() => pickMode(m.id)}
        title={m.title}
      >
        <Icon size={16} />
      </button>
    {/each}
  </div>

  <div class="divider"></div>

  <div class="group stamp-group">
    <button
      class="tool-btn stamp-btn"
      class:active={activeStamp !== null}
      onclick={() => showStamps = !showStamps}
      title="Штампы"
    >
      <Box size={16} />
      {#if activeStamp}
        <span class="stamp-label">{activeStamp}</span>
      {/if}
    </button>

    {#if showStamps}
      <div class="stamp-dropdown">
        {#each Object.keys(SYMBOLS) as type}
          <button 
            class="stamp-item" 
            onclick={(e) => { e.stopPropagation(); selectStamp(type); }}
          >
            {SYMBOLS[type]?.label || type}
          </button>
        {/each}
      </div>
    {/if}
  </div>

  <div class="divider"></div>

  <div class="group">
    <button class="tool-btn" onclick={onLoadBackground} title="Загрузить фото платы">
      <Image size={16} />
    </button>
    {#if hasBg}
      <button class="tool-btn danger" onclick={onRemoveBackground} title="Удалить фон">
        <ImageOff size={16} />
      </button>
    {/if}
  </div>

  <div class="divider"></div>

  <div class="group">
    <button class="tool-btn" onclick={onUndo} disabled={!canUndo} title={strings?.sketch?.undo || 'Undo'}>
      <Undo2 size={16} />
    </button>
    <button class="tool-btn" onclick={onRedo} disabled={!canRedo} title={strings?.sketch?.redo || 'Redo'}>
      <Redo2 size={16} />
    </button>
    <button class="tool-btn" onclick={onRotate} disabled={!hasSelection} title={strings?.sketch?.rotate || 'Rotate'}>
      <RotateCw size={16} />
    </button>
  </div>

  <div class="divider"></div>

  <div class="group">
    <button class="tool-btn" onclick={onZoomOut} title={strings?.sketch?.zoomOut || 'Zoom out'}>
      <ZoomOut size={16} />
    </button>
    <button class="tool-btn" onclick={onZoomIn} title={strings?.sketch?.zoomIn || 'Zoom in'}>
      <ZoomIn size={16} />
    </button>
  </div>

  <div class="divider"></div>

  <div class="group">
    <button class="tool-btn danger" onclick={onClear} title={strings?.sketch?.clear || 'Clear'}>
      <Trash2 size={16} />
    </button>
    <button class="tool-btn" onclick={onToggleFullscreen} title={isFullscreen ? (strings?.sketch?.exitFullscreen || 'Exit fullscreen') : (strings?.sketch?.fullscreen || 'Fullscreen')}>
      {#if isFullscreen}<Minimize size={16} />{:else}<Maximize size={16} />{/if}
    </button>
    <button class="tool-btn primary" onclick={onExport} title={strings?.sketch?.exportPng || 'Export PNG'}>
      <Download size={16} /> PNG
    </button>
  </div>

  <div class="group" style="margin-left: auto;">
    <button class="tool-btn" onclick={onShowHelp} title="Help">
      <HelpCircle size={16} />
    </button>
  </div>
</div>

<style>
  .toolbar {
    display: flex; align-items: center; gap: 4px;
    padding: 6px 8px; background: var(--bg-elevated);
    border-bottom: 1px solid var(--border-subtle); flex-wrap: wrap;
    position: relative; z-index: 50;
  }
  .group { display: flex; gap: 2px; align-items: center; position: relative; }
  .divider { width: 1px; height: 22px; background: var(--border-subtle); margin: 0 6px; }
  .tool-btn {
    display: flex; align-items: center; gap: 4px;
    padding: 6px 10px; background: transparent; color: var(--text-secondary);
    border: 1px solid transparent; border-radius: var(--radius-sm);
    font-size: 12px; font-weight: 600; cursor: pointer; transition: all 0.15s;
  }
  .tool-btn:hover:not(:disabled) { background: var(--bg-elevated-2); color: var(--text-primary); }
  .tool-btn.active { background: var(--accent-dim); color: var(--accent); border-color: var(--accent); }
  .tool-btn:disabled { opacity: 0.3; cursor: not-allowed; }
  .tool-btn.danger:hover:not(:disabled) { color: var(--danger); }
  .tool-btn.primary { background: var(--accent); color: #000; }
  .tool-btn.primary:hover { background: var(--accent-strong); }

  .stamp-group { position: relative; }
  .stamp-label { font-size: 11px; margin-left: 4px; color: var(--accent); text-transform: capitalize; }
  .stamp-dropdown {
    position: absolute; top: 100%; left: 0; margin-top: 4px;
    background: var(--bg-elevated-2); border: 1px solid var(--border-strong);
    border-radius: var(--radius-md); box-shadow: var(--shadow-lg);
    z-index: 9999; display: flex; flex-direction: column; min-width: 150px;
    max-height: 300px; overflow-y: auto; padding: 4px;
  }
  .stamp-item {
    padding: 6px 12px; background: transparent; color: var(--text-primary);
    border: none; text-align: left; border-radius: 4px; cursor: pointer;
    font-size: 13px;
  }
  .stamp-item:hover { background: var(--bg-elevated-3); }
</style>