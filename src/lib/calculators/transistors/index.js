import AnalogSearch from './AnalogSearch.svelte';
import ParamSearch from './ParamSearch.svelte';
import TransistorCalc from './TransistorCalc.svelte';

export const TRANSISTOR_CALCULATORS = [
  { id: 'analog', component: AnalogSearch },
  { id: 'param',  component: ParamSearch },
  { id: 'calc',   component: TransistorCalc }
];