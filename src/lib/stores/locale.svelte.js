import { invoke } from '@tauri-apps/api/core';

const STORAGE_KEY = 'canifol_locale';

function getInitialLocale() {
  if (typeof window === 'undefined') return 'ru';
  let saved = null;
  try {
    saved = localStorage.getItem(STORAGE_KEY);
  } catch (e) {
    // localStorage unavailable — use the browser language below.
  }
  if (saved === 'ru' || saved === 'en' || saved === 'zh') return saved;
  const browserLang = (navigator.language || '').slice(0, 2).toLowerCase();
  if (browserLang === 'ru') return 'ru';
  if (browserLang === 'zh') return 'zh';
  return 'en';
}

function applyLang(locale) {
  if (typeof document !== 'undefined') {
    document.documentElement.setAttribute('lang', locale);
  }
}

let currentLocale = $state(getInitialLocale());
applyLang(currentLocale);

function setLocale(value) {
  if (value !== 'ru' && value !== 'en' && value !== 'zh') return;
  currentLocale = value;
  applyLang(value);
  try {
    localStorage.setItem(STORAGE_KEY, value);
  } catch (e) {
    // Non-fatal: the language still applies for this session.
  }
}

export async function detectSystemLocale() {
  try {
    if (localStorage.getItem(STORAGE_KEY)) return;
  } catch (e) {
    // Continue with system detection.
  }

  try {
    const sysLocale = await invoke('get_system_locale');
    const lang = sysLocale ? String(sysLocale).toLowerCase() : 'en';
    if (lang.startsWith('ru')) setLocale('ru');
    else if (lang.startsWith('zh')) setLocale('zh');
    else setLocale('en');
  } catch (e) {
    console.warn('Failed to detect the system locale:', e);
  }
}

export const localeStore = {
  get current() { return currentLocale; },
  set current(value) { setLocale(value); }
};
