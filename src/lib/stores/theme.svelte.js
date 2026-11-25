const STORAGE_KEY = 'canifol_theme';

function getInitialTheme() {
  if (typeof window === 'undefined') return 'dark';
  try {
    const saved = localStorage.getItem(STORAGE_KEY);
    if (saved === 'light' || saved === 'dark') return saved;
  } catch (e) {
    // localStorage can be unavailable (private mode) — fall back to dark.
  }
  return 'dark';
}

let currentTheme = $state(getInitialTheme());

function apply(theme) {
  if (typeof document !== 'undefined') {
    document.documentElement.setAttribute('data-theme', theme);
  }
}

// Apply immediately so the first paint already uses the saved theme.
apply(currentTheme);

export const themeStore = {
  get current() { return currentTheme; },
  get isDark() { return currentTheme === 'dark'; },
  set(value) {
    const theme = value === 'light' ? 'light' : 'dark';
    currentTheme = theme;
    apply(theme);
    try {
      localStorage.setItem(STORAGE_KEY, theme);
    } catch (e) {
      // Ignore quota/security errors: the theme still applies for this session.
    }
  },
  toggle() {
    this.set(currentTheme === 'dark' ? 'light' : 'dark');
  }
};
