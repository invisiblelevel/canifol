import { invoke } from '@tauri-apps/api/core';

const STORAGE_KEY = 'canifol_locale';

function getInitialLocale() {
  if (typeof window === 'undefined') return 'ru';
  const saved = localStorage.getItem(STORAGE_KEY);
  if (saved === 'ru' || saved === 'en' || saved === 'zh') return saved;
  const browserLang = navigator.language?.slice(0, 2);
  if (browserLang === 'en') return 'en';
  if (browserLang === 'zh') return 'zh';
  return 'ru';
}

let currentLocale = $state(getInitialLocale());

export async function detectSystemLocale() {
  if (typeof window !== 'undefined' && localStorage.getItem(STORAGE_KEY)) return;

  try {
    const sysLocale = await invoke('get_system_locale');
    const lang = sysLocale ? sysLocale.toLowerCase() : 'en';
    if (lang.startsWith('ru')) {
      currentLocale = 'ru';
    } else if (lang.startsWith('zh')) {
      currentLocale = 'zh';
    } else {
      currentLocale = 'en';
    }
  } catch (e) {
    console.warn('Не удалось определить язык системы', e);
  }
}

export const localeStore = {
  get current() { return currentLocale; },
  set current(value) {
    if (value === 'ru' || value === 'en' || value === 'zh') {
      currentLocale = value;
      if (typeof window !== 'undefined') {
        localStorage.setItem(STORAGE_KEY, value);
      }
    }
  }
};