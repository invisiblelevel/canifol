import { Zap, Battery, Cable, FileText, CircuitBoard, Cpu, Network, ScanLine, Layers } from '@lucide/svelte';

export const SECTIONS = [
  { id: 'transistors', icon: Cpu },
  { id: 'resistors',   icon: Zap },
  { id: 'capacitors',  icon: Battery },
  { id: 'inductors',   icon: Cable },
  { id: 'bms',         icon: CircuitBoard },
  { id: 'datasheets',  icon: FileText },
  { id: 'pinouts',     icon: Network },
  { id: 'smd',         icon: ScanLine },
  { id: 'boardview',   icon: Layers }   // <-- НОВЫЙ РАЗДЕЛ
];

let current = $state('transistors');

export const navStore = {
  get current() { return current; },
  set current(value) {
    if (SECTIONS.some((s) => s.id === value)) current = value;
  },
  setSettings() {
    current = 'settings';
  },
  sections: SECTIONS
};