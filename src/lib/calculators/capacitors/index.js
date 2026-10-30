import Marking from './Marking.svelte';
import Reactance from './Reactance.svelte';
import Connections from './Connections.svelte';
import EsrTab from './EsrTab.svelte';

export const CAPACITOR_CALCULATORS = [
  { id: 'marking',     component: Marking },
  { id: 'reactance',   component: Reactance },
  { id: 'connections', component: Connections },
  { id: 'esr',         component: EsrTab }
];