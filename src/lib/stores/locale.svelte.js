import { invoke } from '@tauri-apps/api/core';

const STORAGE_KEY = 'canifol_locale';

// Начальное значение ставим по браузеру (на случай если Tauri ещё не готов)
function getInitialLocale() {
  if (typeof window === 'undefined') return 'ru';
  const saved = localStorage.getItem(STORAGE_KEY);
  if (saved === 'ru' || saved === 'en') return saved;
  const browserLang = navigator.language?.slice(0, 2);
  if (browserLang === 'en') return 'en';
  return 'ru';
}

let currentLocale = $state(getInitialLocale());

// Асинхронная проверка системного языка (вызывается при старте приложения)
export async function detectSystemLocale() {
  // Если пользователь уже выбирал язык вручную — не трогаем
  if (typeof window !== 'undefined' && localStorage.getItem(STORAGE_KEY)) return;

  try {
    const sysLocale = await invoke('get_system_locale'); // например "ru-RU" или "en-US"
    if (sysLocale && sysLocale.toLowerCase().startsWith('ru')) {
      currentLocale = 'ru';
    } else {
      currentLocale = 'en';
    }
  } catch (e) {
    console.warn('Не удалось определить язык системы через Tauri, оставляем браузерный', e);
  }
}

export const localeStore = {
  get current() { return currentLocale; },
  set current(value) {
    if (value === 'ru' || value === 'en') {
      currentLocale = value;
      if (typeof window !== 'undefined') {
        localStorage.setItem(STORAGE_KEY, value);
      }
    }
  }
};