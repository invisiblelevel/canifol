// Все элементы рисуются в локальной системе координат (0,0 = центр).
// Параметры: ctx, theme (объект цветов), selected (bool), opts (доп. настройки).

export const THEME = {
  paper: '#faf7f2',
  ink: '#2a1f10',
  wire: '#3d2f1f',
  pin: '#c47a2a',
  pinHover: '#ff6b35',
  pinSelected: '#2563eb',
  selected: '#2563eb',
  highlight: '#ff6b35',
  chipFill: '#f5ead8',
  text: '#2a1f10',
};

function stroke(ctx, color, width = 2) {
  ctx.strokeStyle = color;
  ctx.lineWidth = width;
  ctx.lineCap = 'round';
  ctx.lineJoin = 'round';
}

// --- Символы ---

export const SYMBOLS = {
  resistor: {
    label: 'Resistor',
    pins: () => [{ x: -20, y: 0 }, { x: 20, y: 0 }],
    w: 40, h: 16,
    draw(ctx, theme, selected) {
      stroke(ctx, selected ? theme.selected : theme.ink, 2);
      ctx.beginPath();
      ctx.moveTo(-20, 0); ctx.lineTo(-12, 0);
      for (let i = 0; i < 6; i++) {
        const x = -12 + (i + 0.5) * 4;
        const y = i % 2 === 0 ? -6 : 6;
        ctx.lineTo(x, y);
      }
      ctx.lineTo(12, 0); ctx.lineTo(20, 0);
      ctx.stroke();
    },
  },

  capacitor: {
    label: 'Capacitor',
    pins: () => [{ x: -14, y: 0 }, { x: 14, y: 0 }],
    w: 28, h: 24,
    draw(ctx, theme, selected) {
      stroke(ctx, selected ? theme.selected : theme.ink, 2.5);
      ctx.beginPath(); ctx.moveTo(-14, 0); ctx.lineTo(-4, 0); ctx.stroke();
      ctx.beginPath(); ctx.moveTo(4, 0); ctx.lineTo(14, 0); ctx.stroke();
      ctx.beginPath(); ctx.moveTo(-4, -10); ctx.lineTo(-4, 10); ctx.stroke();
      ctx.beginPath(); ctx.moveTo(4, -10); ctx.lineTo(4, 10); ctx.stroke();
    },
  },

  inductor: {
    label: 'Inductor',
    pins: () => [{ x: -20, y: 0 }, { x: 20, y: 0 }],
    w: 40, h: 16,
    draw(ctx, theme, selected) {
      stroke(ctx, selected ? theme.selected : theme.ink, 2);
      ctx.beginPath(); ctx.moveTo(-20, 0); ctx.lineTo(-14, 0); ctx.stroke();
      for (let i = 0; i < 4; i++) {
        const cx = -14 + i * 7 + 3.5;
        ctx.beginPath();
        ctx.arc(cx, 0, 3.5, Math.PI, 0, true);
        ctx.stroke();
      }
      ctx.beginPath(); ctx.moveTo(14, 0); ctx.lineTo(20, 0); ctx.stroke();
    },
  },

  diode: {
    label: 'Diode',
    pins: () => [{ x: -16, y: 0 }, { x: 16, y: 0 }],
    w: 32, h: 16,
    draw(ctx, theme, selected) {
      stroke(ctx, selected ? theme.selected : theme.ink, 2);
      // leads
      ctx.beginPath(); ctx.moveTo(-16, 0); ctx.lineTo(-6, 0); ctx.stroke();
      ctx.beginPath(); ctx.moveTo(6, 0); ctx.lineTo(16, 0); ctx.stroke();
      // triangle
      ctx.beginPath();
      ctx.moveTo(-6, -7); ctx.lineTo(6, 0); ctx.lineTo(-6, 7); ctx.closePath();
      ctx.fillStyle = theme.chipFill; ctx.fill(); ctx.stroke();
      // bar
      ctx.beginPath(); ctx.moveTo(6, -7); ctx.lineTo(6, 7); ctx.stroke();
    },
  },

  ground: {
    label: 'Ground',
    pins: () => [{ x: 0, y: -10 }],
    w: 20, h: 12,
    draw(ctx, theme, selected) {
      stroke(ctx, selected ? theme.selected : theme.ink, 2);
      ctx.beginPath(); ctx.moveTo(0, -10); ctx.lineTo(0, -2); ctx.stroke();
      ctx.beginPath(); ctx.moveTo(-10, -2); ctx.lineTo(10, -2); ctx.stroke();
      ctx.beginPath(); ctx.moveTo(-6, 3); ctx.lineTo(6, 3); ctx.stroke();
      ctx.beginPath(); ctx.moveTo(-2, 8); ctx.lineTo(2, 8); ctx.stroke();
    },
  },

  vcc: {
    label: 'VCC',
    pins: () => [{ x: 0, y: 12 }],
    w: 20, h: 24,
    draw(ctx, theme, selected) {
      stroke(ctx, selected ? theme.selected : theme.ink, 2);
      ctx.beginPath(); ctx.moveTo(0, 12); ctx.lineTo(0, -2); ctx.stroke();
      ctx.beginPath(); ctx.moveTo(-6, 4); ctx.lineTo(0, -8); ctx.lineTo(6, 4); ctx.closePath();
      ctx.fillStyle = theme.chipFill; ctx.fill(); ctx.stroke();
    },
  },

  transistor_npn: {
    label: 'Transistor NPN',
    pins: () => [{ x: -10, y: 0 }, { x: 10, y: -8 }, { x: 10, y: 8 }],
    w: 30, h: 24,
    draw(ctx, theme, selected) {
      stroke(ctx, selected ? theme.selected : theme.ink, 2);
      ctx.beginPath(); ctx.arc(0, 0, 12, 0, Math.PI * 2); ctx.stroke();
      
      // База
      ctx.beginPath(); ctx.moveTo(-12, 0); ctx.lineTo(-4, 0); ctx.stroke();
      ctx.beginPath(); ctx.moveTo(-4, -6); ctx.lineTo(-4, 6); ctx.stroke();
      
      // Коллектор (верхний)
      ctx.beginPath(); ctx.moveTo(-4, -3); ctx.lineTo(8, -8); ctx.lineTo(10, -8); ctx.stroke();
      
      // Эмиттер (нижний)
      ctx.beginPath(); ctx.moveTo(-4, 3); ctx.lineTo(8, 8); ctx.lineTo(10, 8); ctx.stroke();
      
      // СТРЕЛКА НАРУЖУ (NPN) — СИНЯЯ
      ctx.beginPath();
      ctx.moveTo(8, 8);       // Острие стрелки
      ctx.lineTo(3, 7);       // Верхний угол
      ctx.lineTo(4, 3);       // Нижний угол
      ctx.closePath();
      ctx.fillStyle = '#3b82f6'; // Синий
      ctx.fill();
    },
  },

  transistor_pnp: {
    label: 'Transistor PNP',
    pins: () => [{ x: -10, y: 0 }, { x: 10, y: -8 }, { x: 10, y: 8 }],
    w: 30, h: 24,
    draw(ctx, theme, selected) {
      stroke(ctx, selected ? theme.selected : theme.ink, 2);
      ctx.beginPath(); ctx.arc(0, 0, 12, 0, Math.PI * 2); ctx.stroke();
      
      // База
      ctx.beginPath(); ctx.moveTo(-12, 0); ctx.lineTo(-4, 0); ctx.stroke();
      ctx.beginPath(); ctx.moveTo(-4, -6); ctx.lineTo(-4, 6); ctx.stroke();
      
      // Коллектор (верхний)
      ctx.beginPath(); ctx.moveTo(-4, -3); ctx.lineTo(8, -8); ctx.lineTo(10, -8); ctx.stroke();
      
      // Эмиттер (нижний)
      ctx.beginPath(); ctx.moveTo(-4, 3); ctx.lineTo(8, 8); ctx.lineTo(10, 8); ctx.stroke();
      
      // СТРЕЛКА ВНУТРЬ (PNP) — ЗЕЛЁНАЯ
      ctx.beginPath();
      ctx.moveTo(-2, 4);      // Острие стрелки (направлено к базе)
      ctx.lineTo(4, 6);       // Верхний угол
      ctx.lineTo(5, 2);       // Нижний угол
      ctx.closePath();
      ctx.fillStyle = '#10b981'; // Зелёный
      ctx.fill();
    },
  },

  microchip: {
    label: 'Microchip',
    pins: (opts) => {
      const n = ((opts?.pinCount || 8) / 2) | 0;
      const pitch = 16;
      const height = (n - 1) * pitch + 20;
      const width = 60;
      const pins = [];
      for (let i = 0; i < n; i++) {
        const y = -height / 2 + 10 + i * pitch;
        pins.push({ x: -width / 2, y });
      }
      for (let i = 0; i < n; i++) {
        const y = -height / 2 + 10 + i * pitch;
        pins.push({ x: width / 2, y });
      }
      return pins;
    },
    w: 60, h: 80,
    draw(ctx, theme, selected, opts) {
      const pinCount = opts?.pinCount || 8;
      const n = pinCount / 2;
      const pitch = 16;
      const height = (n - 1) * pitch + 20;
      const width = 60;

      // body
      ctx.fillStyle = theme.chipFill;
      ctx.strokeStyle = selected ? theme.selected : theme.ink;
      ctx.lineWidth = 2;
      ctx.beginPath();
      ctx.rect(-width / 2, -height / 2, width, height);
      ctx.fill(); ctx.stroke();

      // notch (pin 1)
      ctx.beginPath();
      ctx.arc(-width / 2 + 10, -height / 2 + 10, 4, 0, Math.PI * 2);
      ctx.stroke();

      // pins
      stroke(ctx, selected ? theme.selected : theme.ink, 1.5);
      for (let i = 0; i < n; i++) {
        const y = -height / 2 + 10 + i * pitch;
        ctx.beginPath(); ctx.moveTo(-width / 2, y); ctx.lineTo(-width / 2 - 8, y); ctx.stroke();
        ctx.beginPath(); ctx.moveTo(width / 2, y); ctx.lineTo(width / 2 + 8, y); ctx.stroke();
      }
    },
  },
};

// --- Хелперы ---

export function getPins(element) {
  const sym = SYMBOLS[element.type];
  if (!sym) return [];
  return sym.pins(element.opts || {});
}

export function pinWorldPos(element, localPin) {
  const rot = element.rotation || 0;
  const cos = Math.cos(rot), sin = Math.sin(rot);
  return {
    x: element.x + localPin.x * cos - localPin.y * sin,
    y: element.y + localPin.x * sin + localPin.y * cos,
  };
}

export function drawElement(ctx, element, theme, selected) {
  const sym = SYMBOLS[element.type];
  if (!sym) return;
  ctx.save();
  ctx.translate(element.x, element.y);
  ctx.rotate(element.rotation || 0);
  sym.draw(ctx, theme, selected, element.opts || {});
  ctx.restore();
}