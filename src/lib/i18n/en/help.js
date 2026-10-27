export const help = {
  "help": {
    "resistors": {
      "title": "Resistors — help",
      "smdTitle": "SMD code",
      "smdText": "Enter the code from the resistor body. Supports 3-digit (103), 4-digit (1002), EIA-96 (01C) and R-notation (4R7).",
      "colorTitle": "Colors",
      "colorText": "Pick band colors in order — from first to last. Supports 3, 4, 5 and 6 bands. Result shows tolerance and TCR (if 6 bands).",
      "ohmTitle": "Ohm's law",
      "ohmText": "Enter any two of U, I, R, P — the app calculates the rest. Calculated fields are highlighted with a \"computed\" badge.",
      "connTitle": "Connections",
      "connText": "Switch between series and parallel. Add/remove resistors with buttons. Result is updated instantly.",
      "dividerTitle": "Voltage divider",
      "dividerText": "Enter any three of Uin, Uout, R1, R2 — the app calculates the fourth. Also shows divider current and ratio.",
      "pickTitle": "Pick resistor",
      "pickText": "Enter voltage and current — the app picks resistance, shows dissipated power, recommended power rating and nearest E24 values."
    },
    "capacitors": {
      "title": "Capacitors — help",
      "markingTitle": "Marking",
      "markingText": "Three modes: Code (ceramic/film: 104, 4R7, 3n3), SMD (tantalum: J3, A0), Colors (bands). Result in pF, nF, µF or mF.",
      "reactanceTitle": "Reactance (Xc)",
      "reactanceText": "Enter any two of f, C, Xc — the app calculates the third. Xc is the capacitor's \"resistance\" to AC.",
      "connTitle": "Connections",
      "connText": "Series and parallel. Note: capacitors behave opposite to resistors. Series — capacitance decreases, parallel — increases."
    },
    "inductors": {
      "title": "Inductors — help",
      "markingTitle": "Marking",
      "markingText": "Two modes: SMD (3-4 digits: 100, 4R7) and Colors (MIL standard). Values in µH, nH, mH.",
      "reactanceTitle": "Reactance (Xl)",
      "reactanceText": "Enter any two of f, L, Xl — the app calculates the third. Xl is the inductor's \"resistance\" to AC.",
      "connTitle": "Connections",
      "connText": "Series — inductance adds up. Parallel — decreases. Like resistors.",
      "geometryTitle": "Geometry",
      "geometryText": "Inductance by Wheeler's formula: single-layer and multi-layer. Needs diameter, length, number of turns. Useful when rewinding a choke."
    },
    "bms": {
      "title": "BMS — help",
      "balancingTitle": "Balancer",
      "balancingText": "Balancing resistor calculation per Texas Instruments formula. Enter cell voltage, target balancing current and R_ds(on). Visual shows the circuit.",
      "configTitle": "S/P pack",
      "configText": "Build a pack from N series and M parallel cells. Enter one cell's parameters — the app calculates voltage, capacity, energy and max current. Visual shows wiring: parallel buses, series jumpers, balance wires B1...Bn, terminals B+/B-.",
      "shuntTitle": "Shunt",
      "shuntText": "Current-sense shunt calculation. Enter max current, min current and amplifier Vos. The app shows resistance, power, voltage drop and warns if sensitivity is not enough.",
      "mosfetTitle": "MOSFET",
      "mosfetText": "MOSFET selection for BMS protection. Pick a model from the database (power TO-220 and small SOT-23), enter max current — the app tells how many in parallel with 50% margin."
    },
    "transistors": {
      "title": "Transistors — help",
      "analogTitle": "Analog search",
      "analogText": "Enter a transistor model, and the app searches analogs in three places at once:",
      "analogItem1": "Soviet analogs — local database, works offline.",
      "analogItem2": "DigiKey — modern analogs.",
      "analogItem3": "LCSC — modern analogs.",
      "analogWarning": "Important: Soviet analogs are searched in the general search — there is no separate tab for them. Just enter a model (e.g. KT315 or BC547) and see all three result blocks.",
      "paramTitle": "Parametric search",
      "paramText": "If you don't know the model but know the specs — use this mode. For MOSFET: Vds, Id, RdsOn. For BJT: Vceo, Ic, hFE.",
      "calcTitle": "Calculator",
      "calcText": "Transistor calculations (base resistor, operating modes, etc.).",
      "tipsTitle": "Tips",
      "tip1": "If nothing is found — check spelling. BC547 and BC547A are different queries.",
      "tip2": "The Soviet database may not know very rare models, but popular ones (KT315, KT361, GT402, etc.) are there."
    },
    "datasheets": {
      "title": "Datasheets — help",
      "sourcesTitle": "Sources",
      "sourcesText": "Switch between DigiKey and LCSC if one doesn't find it. Each has its own database, coverage differs.",
      "cacheTitle": "Cache",
      "cacheText": "All downloaded PDFs are cached on disk (in %APPDATA%/Canifol/datasheets). Reopening is instant, even offline.",
      "viewerTitle": "Viewer",
      "viewerText": "PDF opens inside the app. Zoom, page navigation, text search and fullscreen mode work. The \"Save\" button saves the file from cache to a chosen folder.",
      "tipsTitle": "Tips",
      "tip1": "If the datasheet is not found — try another source.",
      "tip2": "Some manufacturers (e.g. TI) serve PDFs with a delay, wait a few seconds.",
      "tip3": "If the link leads to HTML instead of PDF — try LCSC (DigiKey sometimes has redirects)."
    }
  }
};
