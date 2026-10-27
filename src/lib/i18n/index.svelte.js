import { localeStore } from '../stores/locale.svelte.js';
import { ru } from './ru/index.js';
import { en } from './en/index.js';

export function getStrings() {
  return localeStore.current === 'en' ? en : ru;
}
