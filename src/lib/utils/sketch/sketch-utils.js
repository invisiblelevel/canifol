// --- Catmull-Rom сглаживание ---
// --- Chaikin (убирает рваность) ---
function chaikin(points, iterations = 1) {
  if (points.length < 3) return points;
  let result = points;
  for (let iter = 0; iter < iterations; iter++) {
    const next = [result[0]];
    for (let i = 0; i < result.length - 1; i++) {
      const p0 = result[i], p1 = result[i + 1];
      next.push({ x: 0.75 * p0.x + 0.25 * p1.x, y: 0.75 * p0.y + 0.25 * p1.y });
      next.push({ x: 0.25 * p0.x + 0.75 * p1.x, y: 0.25 * p0.y + 0.75 * p1.y });
    }
    next.push(result[result.length - 1]);
    result = next;
  }
  return result;
}

// --- Catmull-Rom (финальное сглаживание) ---
export function smoothStroke(points, samplesPerSegment = 8) {
  if (points.length < 3) return points.slice();
  // Сначала убираем рваность
  const pre = chaikin(points, 1);
  const result = [];
  for (let i = 0; i < pre.length - 1; i++) {
    const p0 = pre[Math.max(0, i - 1)];
    const p1 = pre[i];
    const p2 = pre[i + 1];
    const p3 = pre[Math.min(pre.length - 1, i + 2)];
    for (let s = 0; s < samplesPerSegment; s++) {
      const t = s / samplesPerSegment;
      const t2 = t * t, t3 = t2 * t;
      result.push({
        x: 0.5 * ((2 * p1.x) + (-p0.x + p2.x) * t + (2 * p0.x - 5 * p1.x + 4 * p2.x - p3.x) * t2 + (-p0.x + 3 * p1.x - 3 * p2.x + p3.x) * t3),
        y: 0.5 * ((2 * p1.y) + (-p0.y + p2.y) * t + (2 * p0.y - 5 * p1.y + 4 * p2.y - p3.y) * t2 + (-p0.y + 3 * p1.y - 3 * p2.y + p3.y) * t3),
      });
    }
  }
  result.push(pre[pre.length - 1]);
  return result;
}

// --- Расстояние от точки до отрезка ---
export function distToSegment(px, py, x1, y1, x2, y2) {
  const dx = x2 - x1, dy = y2 - y1;
  const len2 = dx * dx + dy * dy;
  if (len2 === 0) return Math.hypot(px - x1, py - y1);
  let t = ((px - x1) * dx + (py - y1) * dy) / len2;
  t = Math.max(0, Math.min(1, t));
  return Math.hypot(px - (x1 + t * dx), py - (y1 + t * dy));
}

// --- Магнитная привязка к поинтам ---
export function findNearestPin(allPins, x, y, maxDist = 14) {
  let best = null, bestDist = maxDist;
  for (const p of allPins) {
    const d = Math.hypot(p.world.x - x, p.world.y - y);
    if (d < bestDist) { bestDist = d; best = p; }
  }
  return best;
}

// --- Автовыравнивание провода ---
export function alignWire(x1, y1, x2, y2, tolerance = 8) {
  const dx = x2 - x1, dy = y2 - y1;
  const angle = Math.abs(Math.atan2(dy, dx) * 180 / Math.PI);
  // почти горизонтально
  if (angle < tolerance || angle > 180 - tolerance) {
    return { x2, y2: y1 };
  }
  // почти вертикально
  if (Math.abs(angle - 90) < tolerance) {
    return { x2: x1, y2 };
  }
  return { x2, y2 };
}

// --- Проверка попадания точки в прямоугольник (с учётом поворота) ---
export function pointInRotatedRect(px, py, cx, cy, w, h, rot) {
  const cos = Math.cos(-rot), sin = Math.sin(-rot);
  const dx = px - cx, dy = py - cy;
  const lx = dx * cos - dy * sin;
  const ly = dx * sin + dy * cos;
  return Math.abs(lx) <= w / 2 && Math.abs(ly) <= h / 2;
}

// --- Storage ---
const KEY = 'canifol_sketch_v1';

export function saveSketch(state) {
  try {
    localStorage.setItem(KEY, JSON.stringify({ version: 1, ...state }));
    return true;
  } catch (e) {
    console.warn('Failed to save sketch', e);
    return false;
  }
}

export function loadSketch() {
  try {
    const raw = localStorage.getItem(KEY);
    if (!raw) return null;
    const parsed = JSON.parse(raw);
    if (parsed.version !== 1) return null;
    return parsed;
  } catch (e) {
    console.warn('Failed to load sketch', e);
    return null;
  }
}

export function clearSketch() {
  localStorage.removeItem(KEY);
}

// --- Export PNG ---
export function exportCanvasPNG(canvas, filename = 'sketch.png') {
  return new Promise((resolve) => {
    canvas.toBlob((blob) => resolve(blob), 'image/png');
  });
}