import BalancingResistor from './BalancingResistor.svelte';
import BatteryConfig from './BatteryConfig.svelte';
import CurrentShunt from './CurrentShunt.svelte';
import MosfetProtection from './MosfetProtection.svelte';

export const BMS_CALCULATORS = [
  { id: 'balancing', component: BalancingResistor },
  { id: 'config',    component: BatteryConfig },
  { id: 'shunt',     component: CurrentShunt },
  { id: 'mosfet',    component: MosfetProtection }
];