/**
 * Стейт раскладки: отслеживает размер экрана.
 * Расширение .svelte.js нужно, чтобы руны ($state) работали в .js-файлах.
 */

const MOBILE_BREAKPOINT = 768;

let isMobile = $state(false);

function updateIsMobile() {
  if (typeof window === 'undefined') return;
  isMobile = window.matchMedia(`(max-width: ${MOBILE_BREAKPOINT}px)`).matches;
}

if (typeof window !== 'undefined') {
  updateIsMobile();
  window.addEventListener('resize', updateIsMobile);
}

export const layoutStore = {
  get isMobile() {
    return isMobile;
  }
};