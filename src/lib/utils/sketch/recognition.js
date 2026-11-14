// --- Распознавание: геометрические признаки + $P-fallback ---
const N = 32;

function distance(a, b) {
  return Math.hypot(b.x - a.x, b.y - a.y);
}

function pathLength(points) {
  let d = 0;
  for (let i = 1; i < points.length; i++) d += distance(points[i - 1], points[i]);
  return d;
}

function bbox(points) {
  let minX = Infinity, maxX = -Infinity, minY = Infinity, maxY = -Infinity;
  for (const p of points) {
    if (p.x < minX) minX = p.x; if (p.x > maxX) maxX = p.x;
    if (p.y < minY) minY = p.y; if (p.y > maxY) maxY = p.y;
  }
  return { minX, maxX, minY, maxY, w: maxX - minX, h: maxY - minY };
}

// --- Признаки ---

function closure(points) {
  if (points.length < 4) return 0;
  const { w, h } = bbox(points);
  const size = Math.max(w, h, 1);
  const start = points[0];
  const end = points[points.length - 1];
  const d = Math.hypot(end.x - start.x, end.y - start.y);
  return Math.max(0, 1 - d / size);
}

function fillRatio(points) {
  const { minX, minY, w, h } = bbox(points);
  if (w === 0 || h === 0) return 0;
  const G = 10;
  const cells = new Set();
  for (const p of points) {
    const gx = Math.min(G - 1, Math.floor(((p.x - minX) / w) * G));
    const gy = Math.min(G - 1, Math.floor(((p.y - minY) / h) * G));
    cells.add(gx + ':' + gy);
  }
  return cells.size / (G * G);
}

function rectangularity(points) {
  const { minX, maxX, minY, maxY, w, h } = bbox(points);
  const eps = Math.max(1, 0.08 * Math.max(w, h));
  let near = 0;
  for (const p of points) {
    const d = Math.min(
      Math.abs(p.x - minX), Math.abs(p.x - maxX),
      Math.abs(p.y - minY), Math.abs(p.y - maxY)
    );
    if (d < eps) near++;
  }
  return near / points.length;
}

function circularity(points) {
  const n = points.length;
  if (n < 4) return 0;
  let cx = 0, cy = 0;
  for (const p of points) { cx += p.x; cy += p.y; }
  cx /= n; cy /= n;

  let sum = 0, sum2 = 0;
  for (const p of points) {
    const r = Math.hypot(p.x - cx, p.y - cy);
    sum += r; sum2 += r * r;
  }
  const mean = sum / n;
  const variance = sum2 / n - mean * mean;
  const std = Math.sqrt(Math.max(0, variance));
  if (mean === 0) return 0;
  return Math.max(0, 1 - std / mean);
}

function turnCount(points) {
  if (points.length < 3) return 0;
  let turns = 0;
  let lastAngle = null;
  for (let i = 1; i < points.length; i++) {
    const dx = points[i].x - points[i - 1].x;
    const dy = points[i].y - points[i - 1].y;
    if (Math.hypot(dx, dy) < 1) continue;
    const a = Math.atan2(dy, dx);
    if (lastAngle !== null) {
      let diff = a - lastAngle;
      while (diff > Math.PI) diff -= Math.PI * 2;
      while (diff < -Math.PI) diff += Math.PI * 2;
      if (Math.abs(diff) > Math.PI / 3) turns++;
    }
    lastAngle = a;
  }
  return turns;
}

function peakCount(points) {
  if (points.length < 5) return 0;
  let peaks = 0;
  let prevDir = 0;
  for (let i = 1; i < points.length; i++) {
    const dy = points[i].y - points[i - 1].y;
    const dir = dy > 0 ? 1 : dy < 0 ? -1 : 0;
    if (dir !== 0 && prevDir !== 0 && dir !== prevDir) peaks++;
    if (dir !== 0) prevDir = dir;
  }
  return peaks;
}

// --- $P fallback ---

function resample(points, n = N) {
  if (points.length < 2) return points.slice();
  const I = pathLength(points) / (n - 1);
  let D = 0;
  const pts = points.map(p => ({ x: p.x, y: p.y }));
  const result = [{ x: pts[0].x, y: pts[0].y }];

  for (let i = 1; i < pts.length; i++) {
    const d = distance(pts[i - 1], pts[i]);
    if (D + d >= I) {
      const t = (I - D) / (d || 1);
      const nx = pts[i - 1].x + t * (pts[i].x - pts[i - 1].x);
      const ny = pts[i - 1].y + t * (pts[i].y - pts[i - 1].y);
      result.push({ x: nx, y: ny });
      pts.splice(i, 0, { x: nx, y: ny });
      D = 0;
    } else {
      D += d;
    }
  }

  while (result.length < n) result.push({ ...result[result.length - 1] });
  return result.slice(0, n);
}

function normalize(points) {
  const { minX, minY, w, h } = bbox(points);
  const size = Math.max(w, h) || 1;
  const scaled = points.map(p => ({ x: (p.x - minX) / size, y: (p.y - minY) / size }));
  let cx = 0, cy = 0;
  for (const p of scaled) { cx += p.x; cy += p.y; }
  cx /= scaled.length; cy /= scaled.length;
  return scaled.map(p => ({ x: p.x - cx, y: p.y - cy }));
}

function line(x1, y1, x2, y2, n = 10) {
  const pts = [];
  for (let i = 0; i <= n; i++) {
    const t = i / n;
    pts.push({ x: x1 + t * (x2 - x1), y: y1 + t * (y2 - y1) });
  }
  return pts;
}

const capacitor = [...line(0.1, 0.35, 0.9, 0.35), ...line(0.9, 0.65, 0.1, 0.65)];

const diode = [
  { x: 0.1, y: 0.15 }, { x: 0.9, y: 0.5 }, { x: 0.1, y: 0.85 }, { x: 0.1, y: 0.15 }
];

const ground = [
  ...line(0.1, 0.3, 0.9, 0.3),
  ...line(0.25, 0.55, 0.75, 0.55),
  ...line(0.4, 0.8, 0.6, 0.8)
];

const vcc = [
  ...line(0.5, 0.9, 0.5, 0.15),
  ...line(0.15, 0.4, 0.5, 0.1),
  ...line(0.5, 0.1, 0.85, 0.4)
];

const FALLBACK_TEMPLATES = [
  { type: 'capacitor', points: capacitor },
  { type: 'diode', points: diode },
  { type: 'ground', points: ground },
  { type: 'vcc', points: vcc },
];

function matchFallback(points) {
  if (points.length < 6) return null;
  const candidate = normalize(resample(points, N));
  let best = null, bestDist = Infinity;
  for (const tmpl of FALLBACK_TEMPLATES) {
    const normTmpl = normalize(resample(tmpl.points, N));
    let sum = 0;
    for (let i = 0; i < N; i++) sum += distance(candidate[i], normTmpl[i]);
    const avg = sum / N;
    if (avg < bestDist) { bestDist = avg; best = tmpl; }
  }
  if (!best || bestDist > 0.35) return null;
  return { type: best.type, confidence: 1 - bestDist / 0.35 };
}

// --- Главная функция ---

export function matchTemplates(points) {
  if (!points || points.length < 4) {
    return { type: null, confidence: 0, distance: Infinity };
  }

  const { w, h } = bbox(points);
  const size = Math.max(w, h, 1);
  const ratio = w / Math.max(h, 1);
  const fill = fillRatio(points);
  const closed = closure(points);
  const rect = rectangularity(points);
  const circ = circularity(points);
  const turns = turnCount(points);
  const peaks = peakCount(points);
  const totalLen = pathLength(points);
  const lenRatio = totalLen / size;

  // 1) Линия — не распознаём.
  const looksLikeLine =
    (ratio > 5 || ratio < 1 / 5) &&
    fill < 0.25 &&
    turns <= 2;
  const looksLikeSimpleLine =
    fill < 0.15 &&
    turns <= 1 &&
    lenRatio < 1.4;
  if (looksLikeLine || looksLikeSimpleLine) {
    return { type: null, confidence: 0, distance: Infinity };
  }

  // 2) Прямоугольник: замкнут хотя бы частично, прямоугольность высокая,
  //    круглость невысокая, поворотов мало (4 угла).
  if (closed > 0.35 && fill > 0.22 && rect > 0.75 && circ < 0.7 && turns <= 6) {
    return { type: 'rectangle', confidence: Math.min(1, 0.6 + rect * 0.4), distance: 0 };
  }

  // 3) Круг: СИЛЬНО замкнут, круглый, не вытянут.
  if (closed > 0.85 && circ > 0.7 && fill > 0.2 && ratio > 0.6 && ratio < 1.6) {
    return { type: 'circle', confidence: Math.min(1, 0.5 + circ * 0.5), distance: 0 };
  }

  // 4) Резистор: вытянут, много поворотов, длина >> bbox.
  if (ratio > 2 && turns >= 4 && lenRatio > 1.8) {
    return { type: 'resistor', confidence: Math.min(1, 0.5 + turns * 0.05), distance: 0 };
  }

  // 5) Катушка: вытянута, несколько горбов, мало резких поворотов.
  if (ratio > 1.5 && peaks >= 4 && turns <= 6) {
    return { type: 'inductor', confidence: Math.min(1, 0.5 + peaks * 0.05), distance: 0 };
  }

  // 6) Fallback.
  const fb = matchFallback(points);
  if (fb) return { ...fb, distance: 0 };

  return { type: null, confidence: 0, distance: Infinity };
}