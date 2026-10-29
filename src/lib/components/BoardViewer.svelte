<script>
  // ... (без изменений, как в прошлой версии)
  import { invoke } from '@tauri-apps/api/core';
  import { onMount, tick } from 'svelte';
  import { getStrings } from '../strings.svelte.js';

  const strings = $derived(getStrings());
  let { path = '', data: externalData = null } = $props();

  let canvas;
  let ctx;
  let data = $state(externalData || { pins: [], lines: [], arcs: [], components: [], pads: [], routes: [] });
  let loading = $state(!externalData);
  let error = $state('');

  let scale = $state(1);
  let offsetX = $state(0);
  let offsetY = $state(0);
  let isDragging = false;
  let lastX = 0, lastY = 0;
  let isFullscreen = $state(false);

  let showPins = $state(true);
  let showLines = $state(true);
  let showComponents = $state(true);
  let showRoutes = $state(false);

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

  async function loadFile() {
    if (!path) { loading = false; return; }
    loading = true;
    error = '';
    try {
      const result = await invoke('read_boardview', { path });
      data = result;
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
    ctx.fillStyle = '#1a1a1a'; ctx.fillRect(0, 0, rect.width, rect.height);

    const { minX, minY, w, h } = bounds;
    const padding = 20;
    const fitScale = Math.min((rect.width - padding * 2) / w, (rect.height - padding * 2) / h);

    ctx.save();
    ctx.translate(rect.width / 2 + offsetX, rect.height / 2 + offsetY);
    ctx.scale(scale * fitScale, scale * fitScale);
    ctx.translate(-(minX + w / 2), -(minY + h / 2));

    if (showLines && data.lines && data.lines.length > 0) {
      ctx.strokeStyle = '#00aa00'; ctx.lineWidth = 1 / (scale * fitScale); ctx.beginPath();
      for (const line of data.lines) { ctx.moveTo(line.x1, line.y1); ctx.lineTo(line.x2, line.y2); }
      ctx.stroke();
    }

    if (showRoutes && data.routes && data.routes.length > 0) {
      const topRoutes = data.routes.filter(r => r.layer === 'TOP');
      const bottomRoutes = data.routes.filter(r => r.layer !== 'TOP');
      if (topRoutes.length > 0) { ctx.strokeStyle = '#00ff00'; ctx.lineWidth = 1 / (scale * fitScale); ctx.beginPath(); for (const r of topRoutes) { ctx.moveTo(r.x1, r.y1); ctx.lineTo(r.x2, r.y2); } ctx.stroke(); }
      if (bottomRoutes.length > 0) { ctx.strokeStyle = '#ff8800'; ctx.lineWidth = 1 / (scale * fitScale); ctx.beginPath(); for (const r of bottomRoutes) { ctx.moveTo(r.x1, r.y1); ctx.lineTo(r.x2, r.y2); } ctx.stroke(); }
    }

    if (showLines && data.arcs) {
      ctx.strokeStyle = '#00aa00'; ctx.lineWidth = 1 / (scale * fitScale);
      for (const arc of data.arcs) { ctx.beginPath(); ctx.moveTo(arc.x1, arc.y1); ctx.lineTo(arc.x2, arc.y2); ctx.stroke(); }
    }

    if (showComponents && data.components) {
      const worldStroke = 0.5 / (scale * fitScale);
      for (const comp of data.components) {
        if (comp.bounds_x1 === undefined) continue;
        const cw = comp.bounds_x2 - comp.bounds_x1; const ch = comp.bounds_y2 - comp.bounds_y1;
        const ic = isIC(comp.ref_name);
        const isSearched = searchedComponent && searchedComponent.ref_name === comp.ref_name;
        if (isSearched) { ctx.strokeStyle = '#ff00ff'; ctx.lineWidth = worldStroke * 4; ctx.strokeRect(comp.bounds_x1, comp.bounds_y1, cw, ch); }
        else if (ic) { ctx.strokeStyle = '#00aaff'; ctx.lineWidth = worldStroke * 2; ctx.strokeRect(comp.bounds_x1, comp.bounds_y1, cw, ch); }
        else { ctx.fillStyle = 'rgba(80, 80, 80, 0.4)'; ctx.strokeStyle = '#aaaaaa'; ctx.lineWidth = worldStroke * 2; ctx.fillRect(comp.bounds_x1, comp.bounds_y1, cw, ch); ctx.strokeRect(comp.bounds_x1, comp.bounds_y1, cw, ch); }
      }
    }
    ctx.restore();

    if (showPins && data.pins) {
      for (const p of data.pins) {
        const { x, y } = worldToScreen(p.x, p.y, rect);
        if (x < -10 || x > rect.width + 10 || y < -10 || y > rect.height + 10) continue;
        let fillColor = '#ffaa00'; let radius = 3;
        if (selectedPin && selectedPin.name === p.name) { fillColor = '#00ff00'; radius = 5; }
        else if (connectedNames.has(p.name) || searchedPinNames.has(p.name)) { fillColor = '#ff00ff'; radius = 4; }
        ctx.beginPath(); ctx.arc(x, y, radius + 1, 0, Math.PI * 2); ctx.fillStyle = '#000000'; ctx.fill();
        ctx.beginPath(); ctx.arc(x, y, radius, 0, Math.PI * 2); ctx.fillStyle = fillColor; ctx.fill();
      }
    }

    if (showComponents && scale > 1.5 && data.components) {
      ctx.textAlign = 'center'; ctx.textBaseline = 'middle'; let count = 0; const MAX_LABELS = 300;
      for (const comp of data.components) {
        if (count >= MAX_LABELS) break;
        if (comp.bounds_x1 === undefined) continue;
        const cx = (comp.bounds_x1 + comp.bounds_x2) / 2; const cy = (comp.bounds_y1 + comp.bounds_y2) / 2;
        const { x, y } = worldToScreen(cx, cy, rect);
        if (x < 0 || x > rect.width || y < 0 || y > rect.height) continue;
        const ic = isIC(comp.ref_name); const isSearched = searchedComponent && searchedComponent.ref_name === comp.ref_name;
        const fontSize = isSearched ? 20 : (ic ? 16 : 11);
        ctx.font = `${(ic || isSearched) ? 'bold ' : ''}${fontSize}px monospace`;
        const label = comp.device ? `${comp.ref_name} (${comp.device})` : comp.ref_name;
        const textW = ctx.measureText(label).width;
        ctx.fillStyle = 'rgba(0, 0, 0, 0.7)'; ctx.fillRect(x - textW/2 - 2, y - fontSize/2 - 2, textW + 4, fontSize + 4);
        ctx.fillStyle = isSearched ? '#ff00ff' : (ic ? '#00aaff' : '#ffffff'); ctx.fillText(label, x, y); count++;
      }
    }

    if (hoveredPin && hoveredPin.name !== selectedPin?.name) {
      const { x, y } = worldToScreen(hoveredPin.x, hoveredPin.y, rect);
      ctx.beginPath(); ctx.arc(x, y, 8, 0, Math.PI * 2); ctx.strokeStyle = '#ffaa00'; ctx.lineWidth = 2; ctx.stroke();
    }
  }

  $effect(() => { scale; offsetX; offsetY; hoveredPin; showPins; showLines; showComponents; showRoutes; selectedPin; connectedNames; searchedComponent; searchedPinNames; requestDraw(); });

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

  function handleTouchEnd() {
    isTouchDragging = false;
  }

  function toggleFullscreen() {
    isFullscreen = !isFullscreen;
    setTimeout(() => requestDraw(), 100);
  }
</script>

<svelte:window onkeydown={(e) => { if (e.key === 'Escape' && isFullscreen) toggleFullscreen(); }} />

<div class="board-viewer" class:fullscreen={isFullscreen}>
  {#if loading}
    <div class="loading">{strings.boardview.loading}</div>
  {:else if error}
    <div class="error">{error}</div>
  {:else}
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
    <div class="layers-panel">
      <label><input type="checkbox" bind:checked={showPins} /> {strings.boardview.showPins}</label>
      <label><input type="checkbox" bind:checked={showLines} /> {strings.boardview.showLines}</label>
      <label><input type="checkbox" bind:checked={showComponents} /> {strings.boardview.showComponents}</label>
      <label><input type="checkbox" bind:checked={showRoutes} /> {strings.boardview.showRoutes}</label>
    </div>
    <div class="info">
      {strings.boardview.pinsCount}: {data.pins.length} · Компонентов: {data.components.length} · {strings.boardview.zoom}: {scale.toFixed(2)}x
      {#if selectedPin} · <span style="color: #00ff00;">{selectedPin.name}</span> (Net: {selectedPin.net || 'no net'}) · Связей: {connectedNames.size}
      {:else if hoveredPin} · {hoveredPin.name} ({hoveredPin.net || 'no net'}){/if}
    </div>
    <div class="search-bar">
      <input type="text" bind:value={searchQuery} oninput={() => { const q = searchQuery.trim().toUpperCase(); if (!q) { searchedComponent = null; searchedPinNames = new Set(); searchNotFound = false; requestDraw(); return; } const comp = data.components.find(c => c.ref_name.toUpperCase() === q); if (comp) { searchedComponent = comp; const names = new Set(); for (const p of data.pins) { if (p.name.startsWith(comp.ref_name + '.')) names.add(p.name); } searchedPinNames = names; searchNotFound = false; } else { searchedComponent = null; searchedPinNames = new Set(); searchNotFound = true; } requestDraw(); }} placeholder="Поиск: R123, Q21, U5..." class="search-input" />
      {#if searchNotFound}<span class="not-found">Не найдено</span>{/if}
    </div>
    <button class="fullscreen-btn" onclick={toggleFullscreen} title="Fullscreen">{#if isFullscreen}✕{:else}⛶{/if}</button>
  {/if}
</div>

<style>
  .board-viewer { position: relative; width: 100%; height: 70vh; background: #1a1a1a; border-radius: var(--radius-md); overflow: hidden; }
  .board-viewer.fullscreen { position: fixed; top: 0; left: 0; right: 0; bottom: 0; z-index: 9999; height: 100vh; border-radius: 0; }
  canvas { width: 100%; height: 100%; cursor: grab; display: block; touch-action: none; }
  canvas:active { cursor: grabbing; }
  .loading, .error { position: absolute; top: 50%; left: 50%; transform: translate(-50%, -50%); color: var(--text-muted); font-size: 1.2rem; }
  .error { color: var(--danger); }
  .info { position: absolute; bottom: 10px; left: 10px; background: rgba(0,0,0,0.7); padding: 4px 12px; border-radius: var(--radius-sm); font-size: 0.8rem; color: var(--text-muted); pointer-events: none; }
  .layers-panel { position: absolute; top: 10px; left: 10px; background: rgba(0,0,0,0.7); padding: 8px 12px; border-radius: var(--radius-md); display: flex; flex-direction: column; gap: 4px; font-size: 12px; color: #fff; }
  .layers-panel label { display: flex; align-items: center; gap: 4px; cursor: pointer; }
  .layers-panel input[type="checkbox"] { cursor: pointer; }
  .search-bar { position: absolute; bottom: 50px; right: 10px; display: flex; align-items: center; gap: 8px; background: rgba(0,0,0,0.85); padding: 6px 12px; border-radius: var(--radius-md); border: 1px solid var(--border-subtle); }
  .search-input { background: transparent; border: none; color: #fff; font-size: 14px; width: 220px; outline: none; }
  .search-input::placeholder { color: #666; }
  .not-found { color: #ff4444; font-size: 12px; white-space: nowrap; }
  .fullscreen-btn { position: absolute; top: 10px; right: 10px; width: 40px; height: 40px; background: rgba(0,0,0,0.7); color: #ffaa00; border: 1px solid var(--border-subtle); border-radius: var(--radius-md); font-size: 20px; cursor: pointer; display: flex; align-items: center; justify-content: center; transition: background 0.2s; }
  .fullscreen-btn:hover { background: rgba(255, 170, 0, 0.2); }
</style>