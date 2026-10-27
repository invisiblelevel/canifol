import SmdCode from './SmdCode.svelte';
import ColorCode from './ColorCode.svelte';
import OhmLaw from './OhmLaw.svelte';
import Connections from './Connections.svelte';
import VoltageDivider from './VoltageDivider.svelte';
import ResistorPick from './ResistorPick.svelte';

export const RESISTOR_CALCULATORS = [
  { id: 'smd',         component: SmdCode },
  { id: 'color',       component: ColorCode },
  { id: 'ohm',         component: OhmLaw },
  { id: 'connections', component: Connections },
  { id: 'divider',     component: VoltageDivider },
  { id: 'pick',        component: ResistorPick }
];