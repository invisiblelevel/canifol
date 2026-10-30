import { localeStore } from '../stores/locale.svelte.js';
import { ru } from './ru/index.js';
import { en } from './en/index.js';
import { zh } from './zh/index.js';

export function getStrings() {
  if (localeStore.current === 'en') return en;
  if (localeStore.current === 'zh') return zh;
  return ru;
}