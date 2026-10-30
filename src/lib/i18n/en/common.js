export const common = {
  app: { name: 'Canifol', tagline: "Repairman's toolkit" },
  nav: { menu: 'Menu', close: 'Close', sections: 'Sections', settings: 'Settings' },
  sections: {
    favorites: 'Favorites',
    resistors: 'Resistors', capacitors: 'Capacitors', inductors: 'Inductors',
    bms: 'BMS', transistors: 'Transistors', datasheets: 'Datasheets',
    pinouts: 'Pinouts',
    smd: 'SMD Codes',
    boardview: 'BoardView'
  },
  common: {
    comingSoon: 'Calculator coming soon',
    comingSoonHint: 'This section is under development. Switch to other tabs for now.',
    version: 'Version', language: 'Language', theme: 'Theme',
    themeDark: 'Dark', themeLight: 'Light',
    computed: 'computed', reset: 'Reset',
    pcs: 'pcs', result: 'Result', enterValues: 'Fill the fields', ready: 'Done'
  },
  buttons: {
    search: 'Search', copy: 'Copy', copied: 'Copied',
    reset: 'Reset', clear: 'Clear', openPdf: 'Open PDF',
    download: 'Download', back: 'Back', add: 'Add', remove: 'Remove',
    clearCache: 'Clear cache',
    confirm: 'Confirm',
    cancel: 'Cancel'
  },
  colors: {
    black: 'Black', brown: 'Brown', red: 'Red', orange: 'Orange', yellow: 'Yellow', green: 'Green', blue: 'Blue', violet: 'Violet',
    gray: 'Gray', white: 'White', gold: 'Gold', silver: 'Silver'
  },
  transistorTypes: { mosfet: 'MOSFET', bjt: 'Bipolar' },
  markingTabs: { code: 'Code', smd: 'SMD', color: 'Colors' },
  units: {
    hz: 'Hz', khz: 'kHz', mhz: 'MHz', pf: 'pF', nf: 'nF', uf: 'µF', mf: 'mF', f: 'F',
    ohm: 'Ω', kohm: 'kΩ', mohm: 'MΩ', mohm_short: 'mΩ',
    uh: 'µH', nh: 'nH', mh: 'mH', h: 'H',
    v: 'V', a: 'A', w: 'W', mm: 'mm',
    ah: 'Ah', wh: 'Wh', ma: 'mA', uv: 'µV', mv: 'mV',
    b: 'B', kb: 'KB', mb: 'MB', gb: 'GB'
  },
  boardview: {
    openFile: 'Open local file',
    noFile: 'Select a .fz or .cad file to view',
    loading: 'Loading...',
    error: 'Loading error',
    pinsCount: 'Pins',
    zoom: 'Zoom',
    showPins: 'Pins',
    showLines: 'Traces',
    showComponents: 'Components',
    showRoutes: 'Routing',
    helpTitle: 'BoardView Help',
    helpFormatsTitle: 'Supported formats',
    helpFormats: [
      { name: 'FZ / CAE', desc: 'ASUS / ASRock. Pins and components.' },
      { name: 'CAD (GenCAD)', desc: 'Pins, components, traces, pads.' },
      { name: 'BDV / ASC', desc: 'eM-Test Expert. Pins and components (native outlines).' }
    ],
    helpFeaturesTitle: 'Features',
    helpFeatures: [
      'Click a pin to highlight the entire net.',
      'Search by component (e.g., Q21).',
      'Layers: pins, traces, components, routing.',
      'Zoom and pan.',
      'Fullscreen mode.'
    ]
  }
};