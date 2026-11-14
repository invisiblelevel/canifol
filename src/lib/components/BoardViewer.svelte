<script>
  import { invoke } from '@tauri-apps/api/core';
  import { onMount, tick } from 'svelte';
  import { getStrings } from '../strings.svelte.js';
  import { Palette, Search, Maximize2, Minimize2 } from '@lucide/svelte';

  const strings = $derived(getStrings());
  let { path = '', data: externalData = null } = $props();

  let canvas;
  let ctx;
  let data = $state(externalData || { format: '', pins: [], lines: [], arcs: [], components: [], pads: [], routes: [] });
  let loading = $state(!externalData);
  let error = $state('');

  let scale = $state(1);
  let offsetX = $state(0);
  let offsetY = $state(0);
  let isDragging = false;
  let lastX = 0, lastY = 0;
  let isFullscreen = $state(false);

  // --- ТЕМЫ ---
  const THEMES = {
    canifol: {
      bg: '#1a1a1a',
      lineBoard: '#00aa00',
      lineOutline: '#00ff88',
      lineComp: '#00aaff',
      compTopIC: '#00aaff',
      compTopP: '#aaaaaa',
      compBotIC: '#ff8800',
      compBotP: '#aa6600',
      fillTop: 'rgba(80, 80, 80, 0.4)',
      fillBottom: 'rgba(120, 80, 40, 0.4)',
      pinTop: '#ffaa00',
      pinBottom: '#ff6600',
      routeTop: '#00ff00',
      routeBottom: '#ff8800',
      text: '#ffffff',
      labelBg: 'rgba(0, 0, 0, 0.7)',
    },
    obv: {
      bg: '#2b2d31',
      lineBoard: '#4a9eff',
      lineOutline: '#a0c4ff',
      lineComp: '#888888',
      compTopIC: '#e0e0e0',
      compTopP: '#aaaaaa',
      compBotIC: '#ff6b6b',
      compBotP: '#cc5555',
      fillTop: 'rgba(60, 60, 60, 0.5)',
      fillBottom: 'rgba(80, 50, 50, 0.5)',
      pinTop: '#ffd93d',
      pinBottom: '#ff9a3d',
      routeTop: '#4a9eff',
      routeBottom: '#ff6b6b',
      text: '#ffffff',
      labelBg: 'rgba(0, 0, 0, 0.8)',
    }
  };
  let themeName = $state(localStorage.getItem('canifol_bv_theme') || 'canifol');
  const theme = $derived(THEMES[themeName]);

  function toggleTheme() {
    themeName = themeName === 'canifol' ? 'obv' : 'canifol';
    localStorage.setItem('canifol_bv_theme', themeName);
    requestDraw();
  }

  // Видимость слоёв
  let showTop = $state(true);
  let showBottom = $state(true);
  let hasBottomLayer = $state(true);

  // Что рисуем
  let showPins = $state(true);
  let showLines = $state(true);
  let showComponents = $state(true);
  let showRoutes = $state(false);
  let showLabels = $state(true);

  let bounds = { minX: 0, minY: 0, maxX: 0, maxY: 0, w: 1, h: 1 };
  let hoveredPin = $state(null);
  let selectedPin = $state(null);
  let connectedNames = $state(new Set());
  let loadedPath = '';

  let searchQuery = $state('');
  let searchedComponent = $state(null);
  let searchedPinNames = $state(new Set());
  let searchNotFound = $state(false);

  let touchStartDist = 0;
  let touchStartScale = 1;
  let touchStartX = 0;
  let touchStartY = 0;
  let touchStartOffsetX = 0;
  let touchStartOffsetY = 0;
  let isTouchDragging = false;

  let drawPending = false;
  function requestDraw() {
    if (drawPending) return;
    drawPending = true;
    requestAnimationFrame(() => { drawPending = false; draw(); });
  }

  function compSide(comp) {
    if (!comp || !comp.layer) return 'TOP';
    return comp.layer.toUpperCase() === 'BOTTOM' ? 'BOTTOM' : 'TOP';
  }
  function pinSide(pin) {
    if (!pin || !pin.side) return 'TOP';
    return pin.side.toLowerCase() === 'bottom' ? 'BOTTOM' : 'TOP';
  }
  function isCompVisible(comp) {
    const s = compSide(comp);
    if (s === 'TOP' && !showTop) return false;
    if (s === 'BOTTOM' && !showBottom) return false;
    return true;
  }
  function isPinVisible(pin) {
    const s = pinSide(pin);
    if (s === 'TOP' && !showTop) return false;
    if (s === 'BOTTOM' && !showBottom) return false;
    return true;
  }

  async function loadFile() {
    if (!path) { loading = false; return; }
    loading = true;
    error = '';
    try {
      const result = await invoke('read_boardview', { path });
      data = result;
      
      const topPinsCount = data.pins.filter(p => p.side === 'top').length;
      const botPinsCount = data.pins.filter(p => p.side === 'bottom').length;
      const isSingleSided = botPinsCount < topPinsCount * 0.05;
      
      if (isSingleSided) {
        showTop = true;
        showBottom = false;
        hasBottomLayer = false;
      } else {
        showTop = true;
        showBottom = true;
        hasBottomLayer = true;
      }

      selectedPin = null; connectedNames = new Set();
      searchedComponent = null; searchedPinNames = new Set(); searchNotFound = false;

      let minX = Infinity, minY = Infinity, maxX = -Infinity, maxY = -Infinity;
      for (const p of data.pins) {
        if (p.x < minX) minX = p.x; if (p.y < minY) minY = p.y;
        if (p.x > maxX) maxX = p.x; if (p.y > maxY) maxY = p.y;
      }
      for (const l of data.lines) {
        if (l.x1 < minX) minX = l.x1; if (l.y1 < minY) minY = l.y1;
        if (l.x1 > maxX) maxX = l.x1; if (l.y1 > maxY) maxY = l.y1;
        if (l.x2 < minX) minX = l.x2; if (l.y2 < minY) minY = l.y2;
        if (l.x2 > maxX) maxX = l.x2; if (l.y2 > maxY) maxY = l.y2;
      }
      for (const r of data.routes) {
        if (r.x1 < minX) minX = r.x1; if (r.y1 < minY) minY = r.y1;
        if (r.x1 > maxX) maxX = r.x1; if (r.y1 > maxY) maxY = r.y1;
        if (r.x2 < minX) minX = r.x2; if (r.y2 < minY) minY = r.y2;
        if (r.x2 > maxX) maxX = r.x2; if (r.y2 > maxY) maxY = r.y2;
      }
      if (minX === Infinity) { minX = 0; minY = 0; maxX = 100; maxY = 100; }
      bounds = { minX, minY, maxX, maxY, w: (maxX - minX) || 1, h: (maxY - minY) || 1 };
      loadedPath = path;
      loading = false;
      await tick();
      if (canvas) ctx = canvas.getContext('2d');
      setTimeout(() => requestDraw(), 100);
    } catch (e) {
      error = String(e); loading = false;
    }
  }

  onMount(async () => {
    await tick();
    if (canvas) ctx = canvas.getContext('2d');

    if (canvas && canvas.parentElement) {
      const ro = new ResizeObserver(() => requestDraw());
      ro.observe(canvas.parentElement);
    }

    if (externalData) {
      let minX = Infinity, minY = Infinity, maxX = -Infinity, maxY = -Infinity;
      for (const p of data.pins) {
        if (p.x < minX) minX = p.x; if (p.y < minY) minY = p.y;
        if (p.x > maxX) maxX = p.x; if (p.y > maxY) maxY = p.y;
      }
      if (minX === Infinity) { minX = 0; minY = 0; maxX = 100; maxY = 100; }
      bounds = { minX, minY, maxX, maxY, w: (maxX - minX) || 1, h: (maxY - minY) || 1 };
      loading = false;
      setTimeout(() => requestDraw(), 200);
    } else {
      await loadFile();
    }
  });

  $effect(() => {
    if (path && path !== loadedPath) loadFile();
  });

  function worldToScreen(x, y, rect) {
    const { minX, minY, w, h } = bounds;
    const padding = 20;
    const fitScale = Math.min((rect.width - padding * 2) / w, (rect.height - padding * 2) / h);
    const sx = (x - (minX + w / 2)) * scale * fitScale + rect.width / 2 + offsetX;
    const sy = (y - (minY + h / 2)) * scale * fitScale + rect.height / 2 + offsetY;
    return { x: sx, y: sy, fitScale };
  }

  function isIC(refName) { return /^(U|PQU|M|Q)/.test(refName); }

  function findPinAt(mx, my, rect) {
    let closest = null; let minDist = 15;
    for (const p of data.pins) {
      if (!isPinVisible(p)) continue;
      const { x, y } = worldToScreen(p.x, p.y, rect);
      if (x < -15 || x > rect.width + 15 || y < -15 || y > rect.height + 15) continue;
      const dist = Math.hypot(x - mx, y - my);
      if (dist < minDist) { minDist = dist; closest = p; }
    }
    return closest;
  }

  function handlePinClick(pin) {
    if (selectedPin && selectedPin.name === pin.name) {
      selectedPin = null; connectedNames = new Set();
    } else {
      selectedPin = pin;
      const names = new Set();
      for (const p of data.pins) {
        if (p.net === pin.net && p.name !== pin.name) names.add(p.name);
      }
      connectedNames = names;
    }
    requestDraw();
  }

  function draw() {
    if (!canvas || !ctx) return;
    const dpr = window.devicePixelRatio || 1;
    const rect = canvas.getBoundingClientRect();
    if (rect.width < 10 || rect.height < 10) return;
    canvas.width = rect.width * dpr; canvas.height = rect.height * dpr;
    ctx.setTransform(1, 0, 0, 1, 0, 0); ctx.scale(dpr, dpr);
    ctx.clearRect(0, 0, rect.width, rect.height);
    ctx.fillStyle = theme.bg; ctx.fillRect(0, 0, rect.width, rect.height);

    const { minX, minY, w, h } = bounds;
    const padding = 20;
    const fitScale = Math.min((rect.width - padding * 2) / w, (rect.height - padding * 2) / h);

    ctx.save();
    ctx.translate(rect.width / 2 + offsetX, rect.height / 2 + offsetY);
    ctx.scale(scale * fitScale, scale * fitScale);
    ctx.translate(-(minX + w / 2), -(minY + h / 2));

    if (showLines && data.lines && data.lines.length > 0) {
      const boardLines = data.lines.filter(l => l.layer !== 'COMPONENT' && l.layer !== 'OUTLINE');
      if (boardLines.length > 0) {
        ctx.strokeStyle = theme.lineBoard;
        ctx.lineWidth = 1 / (scale * fitScale);
        ctx.beginPath();
        for (const line of boardLines) {
          ctx.moveTo(line.x1, line.y1);
          ctx.lineTo(line.x2, line.y2);
        }
        ctx.stroke();
      }

      const outlineLines = data.lines.filter(l => l.layer === 'OUTLINE' || l.layer === 'BOARD');
      if (outlineLines.length > 0) {
        ctx.strokeStyle = theme.lineOutline;
        ctx.lineWidth = 2 / (scale * fitScale);
        ctx.beginPath();
        for (const line of outlineLines) {
          ctx.moveTo(line.x1, line.y1);
          ctx.lineTo(line.x2, line.y2);
        }
        ctx.stroke();
      }

      const compLines = data.lines.filter(l => l.layer === 'COMPONENT');
      if (compLines.length > 0) {
        ctx.strokeStyle = theme.lineComp;
        ctx.lineWidth = 1.5 / (scale * fitScale);
        ctx.beginPath();
        for (const line of compLines) {
          ctx.moveTo(line.x1, line.y1);
          ctx.lineTo(line.x2, line.y2);
        }
        ctx.stroke();
      }
    }

    if (showRoutes && data.routes && data.routes.length > 0) {
      const topRoutes = data.routes.filter(r => r.layer === 'TOP');
      const bottomRoutes = data.routes.filter(r => r.layer !== 'TOP');
      if (showTop && topRoutes.length > 0) {
        ctx.strokeStyle = theme.routeTop; ctx.lineWidth = 1 / (scale * fitScale);
        ctx.beginPath(); for (const r of topRoutes) { ctx.moveTo(r.x1, r.y1); ctx.lineTo(r.x2, r.y2); } ctx.stroke();
      }
      if (showBottom && bottomRoutes.length > 0) {
        ctx.strokeStyle = theme.routeBottom; ctx.lineWidth = 1 / (scale * fitScale);
        ctx.beginPath(); for (const r of bottomRoutes) { ctx.moveTo(r.x1, r.y1); ctx.lineTo(r.x2, r.y2); } ctx.stroke();
      }
    }

    if (showLines && data.arcs) {
      ctx.strokeStyle = theme.lineBoard; ctx.lineWidth = 1 / (scale * fitScale);
      for (const arc of data.arcs) { ctx.beginPath(); ctx.moveTo(arc.x1, arc.y1); ctx.lineTo(arc.x2, arc.y2); ctx.stroke(); }
    }

    if (showComponents && data.components) {
      const worldStroke = 0.5 / (scale * fitScale);
      for (const comp of data.components) {
        if (comp.bounds_x1 === undefined) continue;
        if (!isCompVisible(comp)) continue;

        const cw = comp.bounds_x2 - comp.bounds_x1;
        const ch = comp.bounds_y2 - comp.bounds_y1;
        const side = compSide(comp);
        const ic = isIC(comp.ref_name);
        const isSearched = searchedComponent && searchedComponent.ref_name === comp.ref_name;

        const strokeTop = ic ? theme.compTopIC : theme.compTopP;
        const strokeBottom = ic ? theme.compBotIC : theme.compBotP;
        const strokeColor = side === 'TOP' ? strokeTop : strokeBottom;

        if (isSearched) {
          ctx.strokeStyle = '#ff00ff';
          ctx.lineWidth = worldStroke * 4;
          ctx.strokeRect(comp.bounds_x1, comp.bounds_y1, cw, ch);
        } else if (ic) {
          ctx.strokeStyle = strokeColor;
          ctx.lineWidth = worldStroke * 2;
          ctx.strokeRect(comp.bounds_x1, comp.bounds_y1, cw, ch);
        } else {
          const fill = side === 'TOP' ? theme.fillTop : theme.fillBottom;
          ctx.fillStyle = fill;
          ctx.strokeStyle = strokeColor;
          ctx.lineWidth = worldStroke * 2;
          ctx.fillRect(comp.bounds_x1, comp.bounds_y1, cw, ch);
          ctx.strokeRect(comp.bounds_x1, comp.bounds_y1, cw, ch);
        }
      }
    }
    ctx.restore();

    if (showPins && data.pins) {
      for (const p of data.pins) {
        if (!isPinVisible(p)) continue;
        const { x, y } = worldToScreen(p.x, p.y, rect);
        if (x < -10 || x > rect.width + 10 || y < -10 || y > rect.height + 10) continue;

        const side = pinSide(p);
        let fillColor = side === 'TOP' ? theme.pinTop : theme.pinBottom;
        let radius = 3;

        if (selectedPin && selectedPin.name === p.name) { fillColor = '#00ff00'; radius = 5; }
        else if (connectedNames.has(p.name) || searchedPinNames.has(p.name)) { fillColor = '#ff00ff'; radius = 4; }

        ctx.beginPath(); ctx.arc(x, y, radius + 1, 0, Math.PI * 2); ctx.fillStyle = '#000000'; ctx.fill();
        ctx.beginPath(); ctx.arc(x, y, radius, 0, Math.PI * 2); ctx.fillStyle = fillColor; ctx.fill();
      }
    }

    if (showComponents && showLabels && scale > 1.5 && data.components) {
      ctx.textAlign = 'center'; ctx.textBaseline = 'middle'; let count = 0; const MAX_LABELS = 300;
      for (const comp of data.components) {
        if (count >= MAX_LABELS) break;
        if (comp.bounds_x1 === undefined) continue;
        if (!isCompVisible(comp)) continue;

        const cx = (comp.bounds_x1 + comp.bounds_x2) / 2;
        const cy = (comp.bounds_y1 + comp.bounds_y2) / 2;
        const { x, y } = worldToScreen(cx, cy, rect);
        if (x < 0 || x > rect.width || y < 0 || y > rect.height) continue;

        const side = compSide(comp);
        const ic = isIC(comp.ref_name);
        const isSearched = searchedComponent && searchedComponent.ref_name === comp.ref_name;
        const fontSize = isSearched ? 20 : (ic ? 16 : 11);
        ctx.font = `${(ic || isSearched) ? 'bold ' : ''}${fontSize}px monospace`;
        const label = comp.device ? `${comp.ref_name} (${comp.device})` : comp.ref_name;
        const textW = ctx.measureText(label).width;
        ctx.fillStyle = theme.labelBg;
        ctx.fillRect(x - textW/2 - 2, y - fontSize/2 - 2, textW + 4, fontSize + 4);
        ctx.fillStyle = isSearched ? '#ff00ff' : (side === 'TOP' ? (ic ? theme.compTopIC : theme.text) : theme.compBotIC);
        ctx.fillText(label, x, y); count++;
      }
    }

    if (hoveredPin && hoveredPin.name !== selectedPin?.name && isPinVisible(hoveredPin)) {
      const { x, y } = worldToScreen(hoveredPin.x, hoveredPin.y, rect);
      ctx.beginPath(); ctx.arc(x, y, 8, 0, Math.PI * 2); ctx.strokeStyle = theme.pinTop; ctx.lineWidth = 2; ctx.stroke();
    }
  }

  $effect(() => {
    scale; offsetX; offsetY; hoveredPin; showPins; showLines; showComponents; showRoutes;
    showTop; showBottom; showLabels; themeName;
    selectedPin; connectedNames; searchedComponent; searchedPinNames;
    requestDraw();
  });

  function handleWheel(e) { e.preventDefault(); scale *= e.deltaY > 0 ? 0.9 : 1.1; requestDraw(); }
  function handleMouseMove(e) {
    const rect = canvas.getBoundingClientRect(); const mx = e.clientX - rect.left; const my = e.clientY - rect.top;
    if (isDragging) { offsetX += e.clientX - lastX; offsetY += e.clientY - lastY; lastX = e.clientX; lastY = e.clientY; requestDraw(); return; }
    const closest = findPinAt(mx, my, rect);
    if (closest !== hoveredPin) { hoveredPin = closest; requestDraw(); }
  }
  function handleMouseDown(e) {
    const rect = canvas.getBoundingClientRect(); const mx = e.clientX - rect.left; const my = e.clientY - rect.top;
    const pin = findPinAt(mx, my, rect);
    if (pin) { handlePinClick(pin); } else { isDragging = true; lastX = e.clientX; lastY = e.clientY; }
  }
  function handleMouseUp() { isDragging = false; }

  function handleTouchStart(e) {
    if (e.touches.length === 1) {
      isTouchDragging = true;
      touchStartX = e.touches[0].clientX;
      touchStartY = e.touches[0].clientY;
      touchStartOffsetX = offsetX;
      touchStartOffsetY = offsetY;
    } else if (e.touches.length === 2) {
      isTouchDragging = false;
      const dx = e.touches[0].clientX - e.touches[1].clientX;
      const dy = e.touches[0].clientY - e.touches[1].clientY;
      touchStartDist = Math.hypot(dx, dy);
      touchStartScale = scale;
    }
  }

  function handleTouchMove(e) {
    e.preventDefault();
    if (e.touches.length === 1 && isTouchDragging) {
      const dx = e.touches[0].clientX - touchStartX;
      const dy = e.touches[0].clientY - touchStartY;
      offsetX = touchStartOffsetX + dx;
      offsetY = touchStartOffsetY + dy;
      requestDraw();
    } else if (e.touches.length === 2) {
      const dx = e.touches[0].clientX - e.touches[1].clientX;
      const dy = e.touches[0].clientY - e.touches[1].clientY;
      const dist = Math.hypot(dx, dy);
      if (touchStartDist > 0) {
        scale = touchStartScale * (dist / touchStartDist);
        requestDraw();
      }
    }
  }

  function handleTouchEnd() { isTouchDragging = false; }

  function toggleFullscreen() {
    isFullscreen = !isFullscreen;
    setTimeout(() => requestDraw(), 100);
  }

  function setOnlyTop() { showTop = true; showBottom = false; requestDraw(); }
  function setOnlyBottom() { showTop = false; showBottom = true; requestDraw(); }
  function setBoth() { showTop = true; showBottom = true; requestDraw(); }
</script>

<svelte:window onkeydown={(e) => { if (e.key === 'Escape' && isFullscreen) toggleFullscreen(); }} />

<div class="board-viewer" class:fullscreen={isFullscreen}>
  {#if loading}
    <div class="loading">{strings.boardview.loading}</div>
  {:else if error}
    <div class="error">{error}</div>
  {:else}
    <!-- Верхняя панель -->
    <div class="bv-toolbar">
      {#if hasBottomLayer}
        <div class="side-group">
          <button class="chip" class:active={showTop && !showBottom} onclick={setOnlyTop}>{strings.boardview.onlyTop}</button>
          <button class="chip" class:active={!showTop && showBottom} onclick={setOnlyBottom}>{strings.boardview.onlyBottom}</button>
          <button class="chip" class:active={showTop && showBottom} onclick={setBoth}>{strings.boardview.both}</button>
        </div>
        <div class="divider"></div>
      {/if}
      
      <button class="chip" class:active={showPins} onclick={() => { showPins = !showPins; requestDraw(); }}>
        <span class="desktop-only">{strings.boardview.showPins}</span>
        <span class="mobile-only">P</span>
      </button>
      <button class="chip" class:active={showLines} onclick={() => { showLines = !showLines; requestDraw(); }}>
        <span class="desktop-only">{strings.boardview.showLines}</span>
        <span class="mobile-only">T</span>
      </button>
      <button class="chip" class:active={showComponents} onclick={() => { showComponents = !showComponents; requestDraw(); }}>
        <span class="desktop-only">{strings.boardview.showComponents}</span>
        <span class="mobile-only">C</span>
      </button>
      <button class="chip" class:active={showLabels} onclick={() => { showLabels = !showLabels; requestDraw(); }}>
        <span class="desktop-only">{strings.boardview.showLabels}</span>
        <span class="mobile-only">L</span>
      </button>
      <button class="chip" class:active={showRoutes} onclick={() => { showRoutes = !showRoutes; requestDraw(); }}>
        <span class="desktop-only">{strings.boardview.showRoutes}</span>
        <span class="mobile-only">R</span>
      </button>

      <div class="spacer"></div>

      <div class="search-box">
        <Search size={14} />
        <input
          type="text"
          bind:value={searchQuery}
          oninput={() => {
            const q = searchQuery.trim().toUpperCase();
            if (!q) { searchedComponent = null; searchedPinNames = new Set(); searchNotFound = false; requestDraw(); return; }
            const comp = data.components.find(c => c.ref_name.toUpperCase() === q);
            if (comp) {
              searchedComponent = comp;
              const s = compSide(comp);
              if (s === 'TOP') showTop = true; else showBottom = true;
              const names = new Set();
              for (const p of data.pins) {
                if (p.name.startsWith(comp.ref_name + '.')) names.add(p.name);
              }
              searchedPinNames = names; searchNotFound = false;
            } else {
              searchedComponent = null; searchedPinNames = new Set(); searchNotFound = true;
            }
            requestDraw();
          }}
          placeholder={strings.boardview.searchPlaceholder || 'Search...'}
        />
      </div>

      <button class="icon-btn" onclick={toggleTheme} title="Theme">
        <Palette size={16} />
      </button>
      <button class="icon-btn" onclick={toggleFullscreen} title="Fullscreen">
        {#if isFullscreen}<Minimize2 size={16} />{:else}<Maximize2 size={16} />{/if}
      </button>
    </div>

    <canvas
      bind:this={canvas}
      onwheel={handleWheel}
      onmousedown={handleMouseDown}
      onmousemove={handleMouseMove}
      onmouseup={handleMouseUp}
      onmouseleave={handleMouseUp}
      ontouchstart={handleTouchStart}
      ontouchmove={handleTouchMove}
      ontouchend={handleTouchEnd}
    ></canvas>

    <div class="info">
      {#if data.format}<span class="fmt-badge">{data.format.toUpperCase()}</span> · {/if}
      {strings.boardview.pinsCount}: {data.pins.filter(isPinVisible).length}
      · {strings.boardview.componentsCount || 'Компонентов'}: {data.components.filter(isCompVisible).length}
      · {strings.boardview.zoom}: {scale.toFixed(2)}x
      {#if selectedPin} · <span style="color: #00ff00;">{selectedPin.name}</span> ({selectedPin.net || 'no net'}) · {strings.boardview.connections || 'Связей'}: {connectedNames.size}
      {:else if hoveredPin} · {hoveredPin.name} ({hoveredPin.net || 'no net'}){/if}
    </div>

    {#if searchNotFound}
      <div class="not-found-toast">{strings.boardview.notFound || 'Not found'}</div>
    {/if}
  {/if}
</div>

<style>
  .board-viewer {
    position: relative;
    width: 100%;
    height: 70vh;
    background: #1a1a1a;
    border-radius: var(--radius-md);
    overflow: hidden;
    display: flex;
    flex-direction: column;
  }
  .board-viewer.fullscreen {
    position: fixed; top: 0; left: 0; right: 0; bottom: 0;
    z-index: 9999; height: 100vh; border-radius: 0;
  }

  /* Toolbar */
  .bv-toolbar {
    display: flex;
    align-items: center;
    gap: 4px;
    padding: 6px 8px;
    background: rgba(20, 20, 20, 0.95);
    border-bottom: 1px solid var(--border-subtle);
    flex-wrap: nowrap;
    overflow-x: auto;
    flex-shrink: 0;
    scrollbar-width: none;
  }
  .bv-toolbar::-webkit-scrollbar { display: none; }

  .side-group { display: flex; gap: 2px; flex-shrink: 0; }
  .divider { width: 1px; height: 20px; background: var(--border-subtle); margin: 0 2px; flex-shrink: 0; }
  .spacer { flex: 1; min-width: 8px; }

  .chip {
    padding: 4px 8px;
    background: rgba(255, 255, 255, 0.06);
    color: var(--text-muted);
    border: 1px solid transparent;
    border-radius: 4px;
    font-size: 11px;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.15s;
    white-space: nowrap;
    flex-shrink: 0;
  }
  .chip:hover { background: rgba(255, 255, 255, 0.12); color: var(--text-primary); }
  .chip.active {
    background: var(--accent, #ffaa00);
    color: #000;
    border-color: var(--accent, #ffaa00);
  }

  .search-box {
    display: flex;
    align-items: center;
    gap: 4px;
    background: rgba(255, 255, 255, 0.08);
    border: 1px solid var(--border-subtle);
    border-radius: 4px;
    padding: 3px 8px;
    color: var(--text-muted);
    min-width: 120px;
    flex-shrink: 1;
  }
  .search-box input {
    background: transparent; border: none; outline: none;
    color: var(--text-primary); font-size: 12px; width: 100%;
  }
  .search-box input::placeholder { color: var(--text-muted); }

  .icon-btn {
    width: 28px; height: 28px;
    display: flex; align-items: center; justify-content: center;
    background: rgba(255, 255, 255, 0.08);
    color: var(--text-muted);
    border: 1px solid var(--border-subtle);
    border-radius: 4px;
    cursor: pointer;
    transition: all 0.15s;
    flex-shrink: 0;
  }
  .icon-btn:hover { background: rgba(255, 255, 255, 0.15); color: var(--text-primary); }

  .mobile-only { display: none; }
  .desktop-only { display: inline; }

  canvas {
    flex: 1;
    width: 100%;
    cursor: grab;
    display: block;
    touch-action: none;
    min-height: 0;
  }
  canvas:active { cursor: grabbing; }

  .loading, .error {
    position: absolute; top: 50%; left: 50%;
    transform: translate(-50%, -50%);
    color: var(--text-muted); font-size: 1.2rem;
  }
  .error { color: var(--danger); }

  .info {
    position: absolute; bottom: 10px; left: 10px;
    background: rgba(0,0,0,0.7);
    padding: 4px 12px;
    border-radius: var(--radius-sm);
    font-size: 0.8rem;
    color: var(--text-muted);
    pointer-events: none;
    display: flex; align-items: center; gap: 4px; flex-wrap: wrap;
  }
  .fmt-badge {
    background: #ffaa00; color: #000;
    padding: 1px 6px; border-radius: 4px;
    font-weight: 600; font-size: 0.7rem;
  }
  .not-found-toast {
    position: absolute; bottom: 50px; right: 20px;
    background: rgba(239, 83, 80, 0.95);
    color: white;
    padding: 6px 14px;
    border-radius: 6px;
    font-size: 13px;
  }

  @media (max-width: 768px) {
    .bv-toolbar { padding: 4px 6px; gap: 3px; }
    .chip { padding: 3px 6px; font-size: 10px; }
    .search-box { min-width: 80px; padding: 2px 6px; }
    .search-box input { font-size: 11px; }
    .icon-btn { width: 24px; height: 24px; }
    
    .desktop-only { display: none; }
    .mobile-only { display: inline; }
  }
</style>