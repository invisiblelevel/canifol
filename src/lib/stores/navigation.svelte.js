import { Zap, Battery, Cable, FileText, CircuitBoard, Cpu } from '@lucide/svelte';

export const SECTIONS = [
  { id: 'transistors', icon: Cpu },
  { id: 'resistors',   icon: Zap },
  { id: 'capacitors',  icon: Battery },
  { id: 'inductors',   icon: Cable },
  { id: 'bms',         icon: CircuitBoard },
  { id: 'datasheets',  icon: FileText }
];

let current = $state('transistors');

export const navStore = {
  get current() { return current; },
  set current(value) {
    if (SECTIONS.some((s) => s.id === value)) current = value;
  },
  // Отдельный метод для настроек, чтобы не добавлять их в SECTIONS
  setSettings() {
    current = 'settings';
  },
  sections: SECTIONS
};