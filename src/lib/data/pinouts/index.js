import connectorsJson from '../pinouts.json';
import icsJson from './ics.json';
import sovietJson from './soviet.json';
import devBoardsJson from './devBoards.json';

function toArray(obj, layoutKey = null) {
  return Object.entries(obj).map(([id, value]) => {
    if (typeof value === 'number') {
      return { id, pins: Array.from({ length: value }, (_, i) => String(i + 1)) };
    }
    return {
      id,
      pins: Array.from({ length: value.pins }, (_, i) => String(i + 1)),
      layout: value.layout,
      pinCount: value.pins
    };
  });
}

export const pinouts = {
  connectors: connectorsJson.connectors,
  ics: toArray(icsJson),
  soviet: toArray(sovietJson),
  devBoards: toArray(devBoardsJson)
};