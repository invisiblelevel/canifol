const STORAGE_KEY = 'canifol_ui_scale';

function getInitialScale() {
  if (typeof window === 'undefined') return 100;
  const saved = localStorage.getItem(STORAGE_KEY);
  const num = parseInt(saved, 10);
  if (!isNaN(num) && num >= 70 && num <= 150) return num;
  return 100;
}

let currentScale = $state(getInitialScale());

if (typeof document !== 'undefined') {
  document.documentElement.style.fontSize = `${currentScale}%`;
}

export const uiScaleStore = {
  get current() { return currentScale; },
  set(value) {
    if (value >= 70 && value <= 150) {
      currentScale = value;
      if (typeof window !== 'undefined') {
        localStorage.setItem(STORAGE_KEY, value);
        document.documentElement.style.fontSize = `${value}%`;
      }
    }
  }
};