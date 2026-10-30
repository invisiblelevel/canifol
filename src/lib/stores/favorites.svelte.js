const STORAGE_KEY = 'canifol_favorites';

function makeDatasheetKey(item) {
  return `${item.name}_${item.manufacturer}_${item.datasheet_url}`;
}

function makeSmdKey(item) {
  return `${item.code}_${item.model}_${item.package}`;
}

function loadFavorites() {
  if (typeof window === 'undefined') return { smd: [], pinouts: [], datasheets: [] };
  try {
    const saved = localStorage.getItem(STORAGE_KEY);
    if (!saved) return { smd: [], pinouts: [], datasheets: [] };
    const parsed = JSON.parse(saved);
    return {
      smd: Array.isArray(parsed.smd) ? parsed.smd : [],
      pinouts: Array.isArray(parsed.pinouts) ? parsed.pinouts : [],
      datasheets: Array.isArray(parsed.datasheets) ? parsed.datasheets : []
    };
  } catch (e) {
    console.warn('Не удалось загрузить избранное', e);
    return { smd: [], pinouts: [], datasheets: [] };
  }
}

let current = $state(loadFavorites());

function save() {
  if (typeof window !== 'undefined') {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(current));
  }
}

export const favoritesStore = {
  get smd() { return current.smd; },
  get pinouts() { return current.pinouts; },
  get datasheets() { return current.datasheets; },

  isFavoriteSmd(item) {
    const key = makeSmdKey(item);
    return current.smd.some(x => makeSmdKey(x) === key);
  },
  toggleSmd(item) {
    const key = makeSmdKey(item);
    const idx = current.smd.findIndex(x => makeSmdKey(x) === key);
    if (idx >= 0) {
      current.smd.splice(idx, 1);
    } else {
      current.smd.push({ ...item, added: Date.now() });
    }
    save();
  },

  isFavoritePinout(id) {
    return current.pinouts.some(item => item.id === id);
  },
  togglePinout(item) {
    const idx = current.pinouts.findIndex(x => x.id === item.id);
    if (idx >= 0) {
      current.pinouts.splice(idx, 1);
    } else {
      current.pinouts.push({ ...item, added: Date.now() });
    }
    save();
  },

  isFavoriteDatasheet(item) {
    const key = makeDatasheetKey(item);
    return current.datasheets.some(x => makeDatasheetKey(x) === key);
  },
  toggleDatasheet(item) {
    const key = makeDatasheetKey(item);
    const idx = current.datasheets.findIndex(x => makeDatasheetKey(x) === key);
    if (idx >= 0) {
      current.datasheets.splice(idx, 1);
    } else {
      current.datasheets.push({ ...item, added: Date.now() });
    }
    save();
  },
  updateDatasheet(item) {
    const key = makeDatasheetKey(item);
    const idx = current.datasheets.findIndex(x => makeDatasheetKey(x) === key);
    if (idx >= 0) {
      current.datasheets[idx] = { ...current.datasheets[idx], ...item };
      save();
    }
  },
  offlineOf(item) {
    const key = makeDatasheetKey(item);
    const found = current.datasheets.find(x => makeDatasheetKey(x) === key);
    return !!(found && found.offline);
  },

  removeSmd(item) {
    const key = makeSmdKey(item);
    const idx = current.smd.findIndex(x => makeSmdKey(x) === key);
    if (idx >= 0) { current.smd.splice(idx, 1); save(); }
  },
  removePinout(id) {
    const idx = current.pinouts.findIndex(x => x.id === id);
    if (idx >= 0) { current.pinouts.splice(idx, 1); save(); }
  },
  removeDatasheet(item) {
    const key = makeDatasheetKey(item);
    const idx = current.datasheets.findIndex(x => makeDatasheetKey(x) === key);
    if (idx >= 0) { current.datasheets.splice(idx, 1); save(); }
  }
};