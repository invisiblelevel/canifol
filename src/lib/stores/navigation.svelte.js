import { Zap, Battery, Cable, FileText, CircuitBoard, Cpu, Network, ScanLine, Layers, Star } from '@lucide/svelte';

export const SECTIONS = [
  { id: 'favorites',   icon: Star },
  { id: 'transistors', icon: Cpu },
  { id: 'resistors',   icon: Zap },
  { id: 'capacitors',  icon: Battery },
  { id: 'inductors',   icon: Cable },
  { id: 'bms',         icon: CircuitBoard },
  { id: 'datasheets',  icon: FileText },
  { id: 'pinouts',     icon: Network },
  { id: 'smd',         icon: ScanLine },
  { id: 'boardview',   icon: Layers }
];

let current = $state('transistors');
let payload = $state(null);
let pinoutKey = $state(0);

export const navStore = {
  get current() { return current; },
  set current(value) {
    if (SECTIONS.some((s) => s.id === value)) current = value;
  },
  get payload() { return payload; },
  setPayload(value) {
    payload = value;
  },
  clearPayload() {
    payload = null;
  },
  
  get pinoutKey() { return pinoutKey; },
  forcePinoutReload() { pinoutKey++; },

  setSettings() {
    current = 'settings';
  },
  sections: SECTIONS
};