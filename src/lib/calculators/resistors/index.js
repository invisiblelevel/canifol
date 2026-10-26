import SmdCode from './SmdCode.svelte';
import ColorCode from './ColorCode.svelte';
import OhmLaw from './OhmLaw.svelte';
import Connections from './Connections.svelte';
import VoltageDivider from './VoltageDivider.svelte';
import ResistorPick from './ResistorPick.svelte';

export const RESISTOR_CALCULATORS = [
  { id: 'smd',         label: 'SMD-код',    component: SmdCode },
  { id: 'color',       label: 'Цвета',      component: ColorCode },
  { id: 'ohm',         label: 'Закон Ома',  component: OhmLaw },
  { id: 'connections', label: 'Соединения', component: Connections },
  { id: 'divider',     label: 'Делитель',   component: VoltageDivider },
  { id: 'pick',        label: 'Подбор',     component: ResistorPick }
];