import Marking from './Marking.svelte';
import Reactance from './Reactance.svelte';
import Connections from './Connections.svelte';
import Geometry from './Geometry.svelte';

export const INDUCTOR_CALCULATORS = [
  { id: 'marking',     component: Marking },
  { id: 'reactance',   component: Reactance },
  { id: 'connections', component: Connections },
  { id: 'geometry',    component: Geometry }
];