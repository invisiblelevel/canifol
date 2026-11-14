<script>
  import { onMount } from 'svelte';
  import { save, open } from '@tauri-apps/plugin-dialog';
  import { writeFile, readFile } from '@tauri-apps/plugin-fs';
  import { matchTemplates } from '../../utils/sketch/recognition.js';
  import { SYMBOLS, THEME, getPins, pinWorldPos, drawElement } from '../../utils/sketch/symbols.js';
  import { smoothStroke, findNearestPin, alignWire, pointInRotatedRect, saveSketch, loadSketch, exportCanvasPNG } from '../../utils/sketch/sketch-utils.js';
  import SketchPopup from './SketchPopup.svelte';
  import { getStrings } from '../../strings.svelte.js';

  const strings = $derived(getStrings());

  let {
    onEmptyChange = () => {},
    onBgChange = () => {},
    mode = $bindable('select'),
    activeStamp = $bindable(null)
  } = $props();

  let canvas, ctx;

  // ФОН
  let backgroundImage = $state(null); // { src, img, x, y, scale, opacity }
  let bgDragging = $state(false);
  let bgDragStart = null;
  let bgDragOffset = null;

  let strokes = $state([]);
  let elements = $state([]);
  let wires = $state([]);
  let points = $state([]);
  let texts = $state([]);
  let selection = $state(null);

  let zoom = $state(1);
  let panX = $state(0);
  let panY = $state(0);

  let undoStack = [];
  let redoStack = [];

  let currentStroke = $state(null);
  let wireStart = $state(null);
  let wirePreview = $state(null);
  let isDragging = $state(false);
  let dragStart = null;
  let dragOffset = null;

  let isPanning = $state(false);
  let panStart = { x: 0, y: 0, panX: 0, panY: 0 };

  let popup = $state(null);
  let popupKey = $state(0);

  let pointerDown = false;
  let lastPointer = { x: 0, y: 0, time: 0 };

  let textModal = $state(null);
  let confirmModal = $state(null);

  const dpr = typeof window !== 'undefined' ? window.devicePixelRatio || 1 : 1;

  onMount(async () => {
    const saved = loadSketch();
    if (saved) {
      strokes = (saved.strokes || []).map(s => ({
        ...s,
        smooth: smoothStroke(s.points, 8),
      }));
      elements = saved.elements || [];
      wires = saved.wires || [];
      points = saved.points || [];
      texts = saved.texts || [];
      if (saved.background) {
        await loadBackgroundFromBase64(saved.background);
      }
    }
    resizeCanvas();
    redraw();

    const ro = new ResizeObserver(() => { resizeCanvas(); redraw(); });
    if (canvas?.parentElement) ro.observe(canvas.parentElement);

    return () => { ro.disconnect(); };
  });

  async function loadBackgroundFromBase64(base64) {
    return new Promise((resolve) => {
      const img = new Image();
      img.onload = () => {
        backgroundImage = {
          src: base64,
          img,
          x: img.width / 2,
          y: img.height / 2,
          scale: 1,
          opacity: 0.5,
        };
        onBgChange(true);
        resolve();
      };
      img.onerror = () => resolve();
      img.src = base64;
    });
  }

  async function loadBackgroundFromFile() {
    try {
      const selected = await open({
        multiple: false,
        filters: [{ name: 'Image', extensions: ['png', 'jpg', 'jpeg', 'webp', 'gif'] }]
      });
      if (!selected) return;

      const bytes = await readFile(selected);
      const blob = new Blob([bytes]);
      const reader = new FileReader();
      reader.onload = () => {
        const base64 = reader.result;
        loadBackgroundFromBase64(base64).then(() => {
          persist();
          redraw();
        });
      };
      reader.readAsDataURL(blob);
    } catch (e) {
      alert('Ошибка загрузки картинки: ' + e);
    }
  }

  function removeBackground() {
    backgroundImage = null;
    onBgChange(false);
    persist();
    redraw();
  }

  function resizeCanvas() {
    if (!canvas) return;
    const rect = canvas.getBoundingClientRect();
    canvas.width = rect.width * dpr;
    canvas.height = rect.height * dpr;
    if (!ctx) ctx = canvas.getContext('2d');
    ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
  }

  function snapshot() {
    undoStack.push(JSON.stringify({ strokes, elements, wires, points, texts }));
    if (undoStack.length > 100) undoStack.shift();
    redoStack = [];
  }

  function undo() {
    if (!undoStack.length) return;
    redoStack.push(JSON.stringify({ strokes, elements, wires, points, texts }));
    const prev = JSON.parse(undoStack.pop());
    strokes = prev.strokes; elements = prev.elements; wires = prev.wires;
    points = prev.points; texts = prev.texts;
    selection = null;
    persist();
    redraw();
  }

  function redo() {
    if (!redoStack.length) return;
    undoStack.push(JSON.stringify({ strokes, elements, wires, points, texts }));
    const next = JSON.parse(redoStack.pop());
    strokes = next.strokes; elements = next.elements; wires = next.wires;
    points = next.points; texts = next.texts;
    selection = null;
    persist();
    redraw();
  }

  function persist() {
    const cleanStrokes = strokes.map(({ smooth, ...rest }) => rest);
    saveSketch({
      strokes: cleanStrokes,
      elements,
      wires,
      points,
      texts,
      background: backgroundImage ? backgroundImage.src : null,
    });
    onEmptyChange(
      strokes.length === 0 && elements.length === 0 && wires.length === 0 &&
      points.length === 0 && texts.length === 0 && !backgroundImage
    );
  }

  function redraw() {
    if (!ctx || !canvas) return;
    const rect = canvas.getBoundingClientRect();
    const W = rect.width, H = rect.height;

    ctx.fillStyle = THEME.paper;
    ctx.fillRect(0, 0, W, H);

    ctx.save();
    ctx.translate(panX, panY);
    ctx.translate(W / 2, H / 2);
    ctx.scale(zoom, zoom);
    ctx.translate(-W / 2, -H / 2);

    // ФОН
    if (backgroundImage && backgroundImage.img) {
      ctx.save();
      ctx.globalAlpha = backgroundImage.opacity;
      const iw = backgroundImage.img.width * backgroundImage.scale;
      const ih = backgroundImage.img.height * backgroundImage.scale;
      ctx.drawImage(
        backgroundImage.img,
        backgroundImage.x - iw / 2,
        backgroundImage.y - ih / 2,
        iw, ih
      );
      ctx.restore();
    }

    // Сетка
    ctx.strokeStyle = 'rgba(42, 31, 16, 0.05)';
    ctx.lineWidth = 1 / zoom;
    for (let x = 0; x < W; x += 20) { ctx.beginPath(); ctx.moveTo(x, 0); ctx.lineTo(x, H); ctx.stroke(); }
    for (let y = 0; y < H; y += 20) { ctx.beginPath(); ctx.moveTo(0, y); ctx.lineTo(W, y); ctx.stroke(); }

    // Штрихи
    for (const s of strokes) {
      if (s.points.length < 2) continue;
      const isSelected = selection?.kind === 'stroke' && selection.id === s.id;
      const smooth = s.smooth || smoothStroke(s.points, 8);
      ctx.beginPath();
      ctx.moveTo(smooth[0].x, smooth[0].y);
      for (let i = 1; i < smooth.length; i++) ctx.lineTo(smooth[i].x, smooth[i].y);
      ctx.strokeStyle = isSelected ? THEME.selected : (s.color || THEME.ink);
      ctx.lineWidth = (s.width || 2) / zoom;
      ctx.lineCap = 'round'; ctx.lineJoin = 'round';
      ctx.stroke();
    }

    // Провода
    for (const w of wires) {
      const p1 = wireEndWorld(w.from);
      const p2 = wireEndWorld(w.to);
      if (!p1 || !p2) continue;
      const isSelected = selection?.kind === 'wire' && selection.id === w.id;
      ctx.beginPath();
      ctx.moveTo(p1.x, p1.y); ctx.lineTo(p2.x, p2.y);
      ctx.strokeStyle = isSelected ? THEME.selected : THEME.wire;
      ctx.lineWidth = (isSelected ? 3 : 2) / zoom;
      ctx.stroke();
    }

    // Элементы и их пины
    for (const el of elements) {
      const isSelected = selection?.kind === 'element' && selection.id === el.id;
      drawElement(ctx, el, THEME, isSelected);

      const pins = getPins(el);
      for (const pin of pins) {
        const worldPos = pinWorldPos(el, pin);
        ctx.beginPath();
        ctx.arc(worldPos.x, worldPos.y, 3.5 / zoom, 0, Math.PI * 2);
        ctx.fillStyle = isSelected ? THEME.pinSelected : THEME.pin;
        ctx.fill();
        ctx.strokeStyle = THEME.ink;
        ctx.lineWidth = 1 / zoom;
        ctx.stroke();
      }
    }

    // Поинты
    for (const p of points) {
      const isSelected = selection?.kind === 'point' && selection.id === p.id;
      ctx.beginPath();
      ctx.arc(p.x, p.y, 4.5 / zoom, 0, Math.PI * 2);
      ctx.fillStyle = isSelected ? THEME.pinSelected : THEME.pin;
      ctx.fill();
      ctx.strokeStyle = THEME.ink;
      ctx.lineWidth = 1 / zoom;
      ctx.stroke();
    }

    // Подписи
    ctx.font = `bold ${12 / zoom}px "Segoe UI", system-ui, sans-serif`;
    ctx.textAlign = 'center';
    ctx.textBaseline = 'middle';

    for (const el of elements) {
      if (!el.value) continue;
      const offset = el.valueOffset || { x: 0, y: -25 };
      const x = el.x + offset.x;
      const y = el.y + offset.y;
      const isSelected = selection?.kind === 'element' && selection.id === el.id;
      ctx.fillStyle = isSelected ? THEME.selected : THEME.text;
      ctx.fillText(el.value, x, y);
    }

    for (const t of texts) {
      const isSelected = selection?.kind === 'text' && selection.id === t.id;
      ctx.fillStyle = isSelected ? THEME.selected : THEME.text;
      ctx.fillText(t.content, t.x, t.y);
    }

    // Превью провода
    if (wireStart && wirePreview) {
      ctx.beginPath();
      ctx.moveTo(wireStart.world.x, wireStart.world.y);
      ctx.lineTo(wirePreview.x, wirePreview.y);
      ctx.strokeStyle = THEME.highlight;
      ctx.lineWidth = 2 / zoom;
      ctx.setLineDash([4 / zoom, 4 / zoom]);
      ctx.stroke();
      ctx.setLineDash([]);

      if (wirePreview.snapped) {
        ctx.beginPath();
        ctx.arc(wirePreview.x, wirePreview.y, 8 / zoom, 0, Math.PI * 2);
        ctx.strokeStyle = THEME.highlight;
        ctx.lineWidth = 2 / zoom;
        ctx.stroke();
      }
    }

    // Текущий штрих
    if (currentStroke && currentStroke.length > 1) {
      const smooth = smoothStroke(currentStroke, 8);
      ctx.beginPath();
      ctx.moveTo(smooth[0].x, smooth[0].y);
      for (let i = 1; i < smooth.length; i++) ctx.lineTo(smooth[i].x, smooth[i].y);
      ctx.strokeStyle = THEME.ink;
      ctx.lineWidth = 2.5 / zoom;
      ctx.lineCap = 'round'; ctx.lineJoin = 'round';
      ctx.stroke();
    }

    ctx.restore();
  }

  function collectAllPins() {
    const list = [];
    for (const el of elements) {
      const pins = getPins(el);
      pins.forEach((pin, idx) => {
        list.push({ kind: 'pin', elementId: el.id, pinIndex: idx, world: pinWorldPos(el, pin) });
      });
    }
    for (const p of points) {
      list.push({ kind: 'point', id: p.id, world: { x: p.x, y: p.y } });
    }
    return list;
  }

  function wireEndWorld(ref) {
    if (ref.type === 'point') {
      const p = points.find(p => p.id === ref.id);
      return p ? { x: p.x, y: p.y } : null;
    }
    if (ref.type === 'pin') {
      const el = elements.find(e => e.id === ref.elementId);
      if (!el) return null;
      const pins = getPins(el);
      if (ref.pinIndex >= pins.length) return null;
      return pinWorldPos(el, pins[ref.pinIndex]);
    }
    return null;
  }

  function getPos(e) {
    const rect = canvas.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const y = e.clientY - rect.top;
    return {
      x: (x - panX - rect.width / 2) / zoom + rect.width / 2,
      y: (y - panY - rect.height / 2) / zoom + rect.height / 2,
    };
  }

  function pointInBackground(pos) {
    if (!backgroundImage || !backgroundImage.img) return false;
    const iw = backgroundImage.img.width * backgroundImage.scale;
    const ih = backgroundImage.img.height * backgroundImage.scale;
    return (
      pos.x >= backgroundImage.x - iw / 2 &&
      pos.x <= backgroundImage.x + iw / 2 &&
      pos.y >= backgroundImage.y - ih / 2 &&
      pos.y <= backgroundImage.y + ih / 2
    );
  }

  function onPointerDown(e) {
    if (popup || textModal || confirmModal) return;
    if (e.button === 1 || (e.button === 0 && e.shiftKey)) {
      isPanning = true;
      panStart = { x: e.clientX, y: e.clientY, panX, panY };
      e.preventDefault();
      return;
    }
    if (e.button !== 0) return;

    const pos = getPos(e);

    if (mode === 'select' && !activeStamp && pointInBackground(pos)) {
      bgDragging = true;
      bgDragStart = pos;
      bgDragOffset = { x: backgroundImage.x - pos.x, y: backgroundImage.y - pos.y };
      return;
    }

    if (activeStamp) {
      snapshot();
      elements.push({
        id: 'el_' + Date.now(),
        type: activeStamp,
        x: pos.x,
        y: pos.y,
        rotation: 0,
        opts: activeStamp === 'microchip' ? { pinCount: 8 } : {},
        value: '',
        valueOffset: { x: 0, y: -25 },
      });
      persist(); redraw();
      return;
    }

    pointerDown = true;
    lastPointer = { ...pos, time: Date.now() };

    if (mode === 'pencil' || mode === 'eraser') {
      currentStroke = [pos];
      return;
    }

    if (mode === 'point') {
      snapshot();
      points.push({ id: 'pt_' + Date.now(), x: pos.x, y: pos.y });
      persist(); redraw();
      return;
    }

    if (mode === 'text') {
      textModal = { x: pos.x, y: pos.y, value: '' };
      return;
    }

    if (mode === 'wire') {
      const pins = collectAllPins();
      const nearest = findNearestPin(pins, pos.x, pos.y, 35);
      if (nearest) {
        wireStart = nearest.kind === 'pin'
          ? { type: 'pin', elementId: nearest.elementId, pinIndex: nearest.pinIndex, world: nearest.world }
          : { type: 'point', id: nearest.id, world: nearest.world };
        wirePreview = { x: pos.x, y: pos.y, snapped: false, nearest: null };
        redraw();
      }
      return;
    }

    if (mode === 'select') {
      const pins = collectAllPins();
      for (const p of pins) {
        if (Math.hypot(p.world.x - pos.x, p.world.y - pos.y) < 10 / zoom) {
          selection = p.kind === 'pin'
            ? { kind: 'element', id: p.elementId }
            : { kind: 'point', id: p.id };
          redraw();
          return;
        }
      }
      for (const el of elements.slice().reverse()) {
        const sym = SYMBOLS[el.type];
        const w = sym.w + 16, h = sym.h + 16;
        if (pointInRotatedRect(pos.x, pos.y, el.x, el.y, w, h, el.rotation || 0)) {
          selection = { kind: 'element', id: el.id };
          isDragging = true;
          dragStart = pos;
          dragOffset = { x: el.x - pos.x, y: el.y - pos.y };
          redraw();
          return;
        }
      }
      for (const t of texts.slice().reverse()) {
        if (Math.hypot(t.x - pos.x, t.y - pos.y) < 40 / zoom) {
          selection = { kind: 'text', id: t.id };
          isDragging = true;
          dragStart = pos;
          dragOffset = { x: t.x - pos.x, y: t.y - pos.y };
          redraw();
          return;
        }
      }
      for (const el of elements) {
        if (!el.value) continue;
        const offset = el.valueOffset || { x: 0, y: -25 };
        const vx = el.x + offset.x, vy = el.y + offset.y;
        if (Math.hypot(vx - pos.x, vy - pos.y) < 25 / zoom) {
          selection = { kind: 'element', id: el.id };
          isDragging = 'value';
          dragStart = pos;
          dragOffset = { x: offset.x, y: offset.y };
          redraw();
          return;
        }
      }
      for (const w of wires) {
        const p1 = wireEndWorld(w.from), p2 = wireEndWorld(w.to);
        if (!p1 || !p2) continue;
        const d = distToSegment(pos.x, pos.y, p1.x, p1.y, p2.x, p2.y);
        if (d < 6 / zoom) {
          selection = { kind: 'wire', id: w.id };
          redraw();
          return;
        }
      }
      selection = null;
      redraw();
    }
  }

  function onPointerMove(e) {
    if (bgDragging && backgroundImage) {
      const pos = getPos(e);
      backgroundImage.x = pos.x + bgDragOffset.x;
      backgroundImage.y = pos.y + bgDragOffset.y;
      redraw();
      return;
    }

    if (isPanning) {
      panX = panStart.panX + (e.clientX - panStart.x);
      panY = panStart.panY + (e.clientY - panStart.y);
      redraw();
      return;
    }

    if (!pointerDown) return;
    const pos = getPos(e);

    if (mode === 'pencil') {
      const dx = pos.x - lastPointer.x, dy = pos.y - lastPointer.y;
      const speed = Math.hypot(dx, dy);
      if (speed > 0.3 / zoom) {
        currentStroke.push(pos);
        lastPointer = { ...pos, time: Date.now() };
        redraw();
      }
      return;
    }

    if (mode === 'eraser') { eraseAt(pos); return; }

    if (mode === 'wire' && wireStart) {
      const pins = collectAllPins();
      const nearest = findNearestPin(pins, pos.x, pos.y, 35);
      let target = { x: pos.x, y: pos.y };
      let snapped = false;
      let snappedNearest = null;
      if (nearest) {
        target = nearest.world; snapped = true; snappedNearest = nearest;
      } else {
        const aligned = alignWire(wireStart.world.x, wireStart.world.y, pos.x, pos.y);
        target = { x: aligned.x2, y: aligned.y2 };
      }
      wirePreview = { ...target, snapped, nearest: snappedNearest };
      redraw();
      return;
    }

    if (mode === 'select' && isDragging && dragStart) {
      if (isDragging === 'value') {
        const el = elements.find(e => e.id === selection.id);
        if (el) { el.valueOffset = { x: pos.x - el.x, y: pos.y - el.y }; redraw(); }
      } else if (selection?.kind === 'element') {
        const el = elements.find(e => e.id === selection.id);
        if (el) { el.x = pos.x + dragOffset.x; el.y = pos.y + dragOffset.y; redraw(); }
      } else if (selection?.kind === 'text') {
        const t = texts.find(t => t.id === selection.id);
        if (t) { t.x = pos.x + dragOffset.x; t.y = pos.y + dragOffset.y; redraw(); }
      }
    }
  }

  function onPointerUp(e) {
    if (bgDragging) { bgDragging = false; persist(); return; }
    if (isPanning) { isPanning = false; return; }
    if (!pointerDown) return;
    pointerDown = false;

    if (mode === 'pencil' && currentStroke && currentStroke.length > 3) {
      const strokeCopy = currentStroke.slice();
      currentStroke = null;
      recognizeStroke(strokeCopy);
      return;
    }
    currentStroke = null;

    if (mode === 'wire' && wireStart) {
      let nearest = wirePreview?.nearest || null;
      if (!nearest) {
        const pins = collectAllPins();
        const pos = getPos(e);
        nearest = findNearestPin(pins, pos.x, pos.y, 35);
      }
      if (nearest) {
        snapshot();
        wires.push({
          id: 'w_' + Date.now(),
          from: wireStart.type === 'pin' ? { type: 'pin', elementId: wireStart.elementId, pinIndex: wireStart.pinIndex } : { type: 'point', id: wireStart.id },
          to: nearest.kind === 'pin' ? { type: 'pin', elementId: nearest.elementId, pinIndex: nearest.pinIndex } : { type: 'point', id: nearest.id },
        });
        persist();
      }
      wireStart = null; wirePreview = null; redraw();
      return;
    }

    if (mode === 'select' && isDragging) {
      snapshot(); isDragging = false; dragStart = null; dragOffset = null;
      persist(); redraw();
    }
  }

  // --- TOUCH EVENTS ---
  let touchPoints = new Map();
  let lastPinchDist = 0;
  let lastPinchCenter = { x: 0, y: 0 };
  let isPinching = false;
  let isTouchPanning = false;

  function onTouchStart(e) {
    if (popup || textModal || confirmModal) return;
    for (const t of e.changedTouches) touchPoints.set(t.identifier, { x: t.clientX, y: t.clientY });

    if (touchPoints.size === 2) {
      isPinching = true; isTouchPanning = true; pointerDown = false;
      currentStroke = null; wireStart = null; wirePreview = null;
      const pts = Array.from(touchPoints.values());
      lastPinchDist = Math.hypot(pts[1].x - pts[0].x, pts[1].y - pts[0].y);
      lastPinchCenter = { x: (pts[0].x + pts[1].x) / 2, y: (pts[0].y + pts[1].y) / 2 };
      return;
    }

    if (touchPoints.size === 1 && !isPinching) {
      const t = e.changedTouches[0];
      onPointerDown({ clientX: t.clientX, clientY: t.clientY, button: 0, shiftKey: false, preventDefault: () => {} });
    }
  }

  function onTouchMove(e) {
    if (popup || textModal || confirmModal) return;
    for (const t of e.changedTouches) touchPoints.set(t.identifier, { x: t.clientX, y: t.clientY });

    if (isPinching && touchPoints.size === 2) {
      const pts = Array.from(touchPoints.values());
      const dist = Math.hypot(pts[1].x - pts[0].x, pts[1].y - pts[0].y);
      const center = { x: (pts[0].x + pts[1].x) / 2, y: (pts[0].y + pts[1].y) / 2 };

      if (lastPinchDist > 0) {
        const factor = dist / lastPinchDist;
        const newZoom = Math.max(0.3, Math.min(4, zoom * factor));
        const rect = canvas.getBoundingClientRect();
        const cx = center.x - rect.left; const cy = center.y - rect.top;
        panX = cx - (cx - panX) * (newZoom / zoom);
        panY = cy - (cy - panY) * (newZoom / zoom);
        zoom = newZoom;
      }
      if (isTouchPanning) {
        panX += center.x - lastPinchCenter.x;
        panY += center.y - lastPinchCenter.y;
      }
      lastPinchDist = dist; lastPinchCenter = center;
      redraw();
      return;
    }

    if (touchPoints.size === 1 && !isPinching) {
      const t = e.changedTouches[0];
      onPointerMove({ clientX: t.clientX, clientY: t.clientY, preventDefault: () => {} });
    }
  }

  function onTouchEnd(e) {
    for (const t of e.changedTouches) touchPoints.delete(t.identifier);
    if (isPinching) {
      if (touchPoints.size < 2) { isPinching = false; isTouchPanning = false; lastPinchDist = 0; }
      return;
    }
    if (touchPoints.size === 0) {
      const t = e.changedTouches[0];
      onPointerUp({ clientX: t.clientX, clientY: t.clientY, preventDefault: () => {} });
    }
  }

  function onWheel(e) {
    e.preventDefault();
    const rect = canvas.getBoundingClientRect();
    const mouseX = e.clientX - rect.left;
    const mouseY = e.clientY - rect.top;

    // Ctrl + колесо = масштабирование фона
    if (e.ctrlKey && backgroundImage) {
      const factor = e.deltaY > 0 ? 0.95 : 1.05;
      backgroundImage.scale *= factor;
      redraw();
      return;
    }

    const factor = e.deltaY > 0 ? 0.9 : 1.1;
    const newZoom = Math.max(0.3, Math.min(4, zoom * factor));
    panX = mouseX - (mouseX - panX) * (newZoom / zoom);
    panY = mouseY - (mouseY - panY) * (newZoom / zoom);
    zoom = newZoom;
    redraw();
  }

  function eraseAt(pos) {
    for (const el of elements.slice().reverse()) {
      if (pointInRotatedRect(pos.x, pos.y, el.x, el.y, 60, 60, 0)) {
        snapshot(); elements = elements.filter(e => e.id !== el.id);
        wires = wires.filter(w => !(w.from.elementId === el.id || w.to.elementId === el.id));
        persist(); redraw(); return;
      }
    }
    for (const t of texts) {
      if (Math.hypot(t.x - pos.x, t.y - pos.y) < 30) {
        snapshot(); texts = texts.filter(x => x.id !== t.id); persist(); redraw(); return;
      }
    }
    for (const p of points) {
      if (Math.hypot(p.x - pos.x, p.y - pos.y) < 10) {
        snapshot(); points = points.filter(x => x.id !== p.id);
        wires = wires.filter(w => !(w.from.type === 'point' && w.from.id === p.id) && !(w.to.type === 'point' && w.to.id === p.id));
        persist(); redraw(); return;
      }
    }
    for (const w of wires) {
      const p1 = wireEndWorld(w.from), p2 = wireEndWorld(w.to);
      if (!p1 || !p2) continue;
      if (distToSegment(pos.x, pos.y, p1.x, p1.y, p2.x, p2.y) < 8) {
        snapshot(); wires = wires.filter(x => x.id !== w.id); persist(); redraw(); return;
      }
    }
    for (const s of strokes) {
      for (const p of s.points) {
        if (Math.hypot(p.x - pos.x, p.y - pos.y) < 8) {
          snapshot(); strokes = strokes.filter(x => x.id !== s.id); persist(); redraw(); return;
        }
      }
    }
  }

  function recognizeStroke(points) {
    const match = matchTemplates(points);
    if (!match.type || match.confidence < 0.35) {
      snapshot(); strokes.push({ id: 's_' + Date.now(), points, smooth: smoothStroke(points, 8), color: THEME.ink, width: 2 });
      persist(); redraw(); return;
    }
    const cx = points.reduce((a, p) => a + p.x, 0) / points.length;
    const cy = points.reduce((a, p) => a + p.y, 0) / points.length;

    const rect = canvas.getBoundingClientRect();
    const W = rect.width, H = rect.height;
    const screenX = (cx - W / 2) * zoom + W / 2 + panX;
    const screenY = (cy - H / 2) * zoom + H / 2 + panY;
    const flip = screenY < 160;

    popup = { x: screenX, y: screenY, flip, wrapW: W, wrapH: H, points, type: match.type, confidence: match.confidence };
    popupKey++;
  }

  function confirmRecognize({ type, opts = {} }) {
    if (!popup) return;
    snapshot();
    let minX = Infinity, maxX = -Infinity, minY = Infinity, maxY = -Infinity;
    for (const p of popup.points) {
      if (p.x < minX) minX = p.x; if (p.x > maxX) maxX = p.x;
      if (p.y < minY) minY = p.y; if (p.y > maxY) maxY = p.y;
    }
    const cx = (minX + maxX) / 2, cy = (minY + maxY) / 2;
    const w = maxX - minX, h = maxY - minY;
    let rotation = 0;
    if (type !== 'ground' && type !== 'vcc' && type !== 'microchip' && h > w * 1.2) rotation = Math.PI / 2;

    elements.push({ id: 'el_' + Date.now(), type, x: cx, y: cy, rotation, opts, value: '', valueOffset: { x: 0, y: -25 } });
    popup = null; persist(); redraw();
  }

  function cancelRecognize() {
    if (!popup) return;
    snapshot();
    strokes.push({ id: 's_' + Date.now(), points: popup.points, smooth: smoothStroke(popup.points, 8), color: THEME.ink, width: 2 });
    popup = null; persist(); redraw();
  }

  function setMode(m) { mode = m; currentStroke = null; wireStart = null; wirePreview = null; redraw(); }

  function rotateSelected() {
    if (selection?.kind === 'element') {
      const el = elements.find(e => e.id === selection.id);
      if (el) { snapshot(); el.rotation = (el.rotation || 0) + Math.PI / 2; persist(); redraw(); }
    }
  }

  function deleteSelected() {
    if (!selection) return;
    snapshot();
    if (selection.kind === 'element') {
      elements = elements.filter(e => e.id !== selection.id);
      wires = wires.filter(w => !(w.from.elementId === selection.id || w.to.elementId === selection.id));
    } else if (selection.kind === 'stroke') strokes = strokes.filter(s => s.id !== selection.id);
    else if (selection.kind === 'wire') wires = wires.filter(w => w.id !== selection.id);
    else if (selection.kind === 'point') points = points.filter(p => p.id !== selection.id);
    else if (selection.kind === 'text') texts = texts.filter(t => t.id !== selection.id);
    selection = null; persist(); redraw();
  }

  export function undo_() { undo(); }
  export function redo_() { redo(); }
  export function rotate_() { rotateSelected(); }
  export function delete_() { deleteSelected(); }
  export function zoomIn_() { zoom = Math.min(4, zoom * 1.2); redraw(); }
  export function zoomOut_() { zoom = Math.max(0.3, zoom / 1.2); redraw(); }
  export function zoomFit_() { zoom = 1; panX = 0; panY = 0; redraw(); }
  export function clearAll() {
    confirmModal = {
      message: strings?.sketch?.confirmClear || 'Очистить всё?',
      onConfirm: () => {
        snapshot();
        strokes = []; elements = []; wires = []; points = []; texts = []; selection = null;
        persist(); redraw();
      }
    };
  }
  export async function exportPNG_() {
    const blob = await exportCanvasPNG(canvas);
    if (!blob) return;
    const path = await save({ filters: [{ name: 'PNG', extensions: ['png'] }], defaultPath: 'sketch.png' });
    if (path) {
      const bytes = new Uint8Array(await blob.arrayBuffer());
      await writeFile(path, bytes);
    }
  }
  export function getState() { return { strokes, elements, wires, points, texts, backgroundImage }; }
  export function getMode() { return mode; }
  export function getCanUndo() { return undoStack.length > 0; }
  export function getCanRedo() { return redoStack.length > 0; }
  export function getHasSelection() { return !!selection; }
  export function getZoom() { return zoom; }
  export function hasBackground() { return !!backgroundImage; }

  export { loadBackgroundFromFile, removeBackground };

  function distToSegment(px, py, x1, y1, x2, y2) {
    const dx = x2 - x1, dy = y2 - y1;
    const len2 = dx * dx + dy * dy;
    if (len2 === 0) return Math.hypot(px - x1, py - y1);
    let t = ((px - x1) * dx + (py - y1) * dy) / len2;
    t = Math.max(0, Math.min(1, t));
    return Math.hypot(px - (x1 + t * dx), py - (y1 + t * dy));
  }

  function addText() {
    if (!textModal || !textModal.value.trim()) {
      textModal = null;
      return;
    }
    snapshot();
    texts.push({ id: 'txt_' + Date.now(), x: textModal.x, y: textModal.y, content: textModal.value.trim() });
    textModal = null;
    persist(); redraw();
  }
</script>

<div class="canvas-wrap">
  <canvas
    bind:this={canvas}
    onpointerdown={onPointerDown}
    onpointermove={onPointerMove}
    onpointerup={onPointerUp}
    onpointerleave={onPointerUp}
    onwheel={onWheel}
    ontouchstart={onTouchStart}
    ontouchmove={onTouchMove}
    ontouchend={onTouchEnd}
    ontouchcancel={onTouchEnd}
    style="cursor: {activeStamp ? 'copy' : bgDragging ? 'move' : isPanning ? 'grabbing' : mode === 'select' ? 'default' : mode === 'eraser' ? 'cell' : 'crosshair'}; touch-action: none;"
  ></canvas>

  {#if popup}
    {#key popupKey}
      <SketchPopup
        x={popup.x} y={popup.y} flip={popup.flip} wrapW={popup.wrapW} wrapH={popup.wrapH}
        suggestedType={popup.type} confidence={popup.confidence}
        onConfirm={confirmRecognize} onCancel={cancelRecognize}
      />
    {/key}
  {/if}

  {#if mode === 'select' && selection?.kind === 'element' && !activeStamp}
    <div class="hint">{strings?.sketch?.hintSelect || 'R — rotate · Del — delete'}</div>
  {/if}

  {#if textModal}
    <div class="modal-overlay">
      <div class="modal-box">
        <h3>{strings?.sketch?.promptText || 'Текст'}</h3>
        <input
          type="text"
          bind:value={textModal.value}
          autofocus
          onkeydown={(e) => { if (e.key === 'Enter') addText(); }}
        />
        <div class="modal-actions">
          <button onclick={() => textModal = null}>Отмена</button>
          <button onclick={addText}>OK</button>
        </div>
      </div>
    </div>
  {/if}

  {#if confirmModal}
    <div class="modal-overlay">
      <div class="modal-box">
        <h3>{confirmModal.message}</h3>
        <div class="modal-actions">
          <button onclick={() => confirmModal = null}>Отмена</button>
          <button onclick={() => { confirmModal.onConfirm(); confirmModal = null; }}>OK</button>
        </div>
      </div>
    </div>
  {/if}
</div>

<style>
  .canvas-wrap {
    position: relative; width: 100%; height: 100%; overflow: hidden; background: #faf7f2;
  }
  canvas {
    display: block; width: 100%; height: 100%; touch-action: none; user-select: none;
  }
  .hint {
    position: absolute; bottom: 8px; left: 50%; transform: translateX(-50%);
    background: rgba(42, 31, 16, 0.85); color: #f0e9e0; padding: 4px 12px;
    border-radius: 6px; font-size: 11px; pointer-events: none;
  }

  .modal-overlay {
    position: absolute; inset: 0; background: rgba(0,0,0,0.6);
    display: flex; align-items: center; justify-content: center; z-index: 200;
  }
  .modal-box {
    background: var(--bg-elevated); padding: 16px; border-radius: var(--radius-md);
    border: 1px solid var(--border-strong); min-width: 280px; max-width: 90%;
  }
  .modal-box h3 { margin: 0 0 12px; font-size: 14px; color: var(--text-primary); }
  .modal-box input {
    width: 100%; padding: 8px 12px; background: var(--bg-base);
    border: 1px solid var(--border-subtle); border-radius: var(--radius-sm);
    color: var(--text-primary); font-size: 14px; margin-bottom: 12px;
  }
  .modal-actions { display: flex; gap: 8px; justify-content: flex-end; }
  .modal-actions button {
    padding: 6px 14px; border-radius: var(--radius-sm); font-size: 13px; cursor: pointer;
    background: var(--bg-elevated-2); color: var(--text-primary); border: 1px solid var(--border-subtle);
  }
  .modal-actions button:last-child { background: var(--accent); color: #000; border-color: var(--accent); }
</style>