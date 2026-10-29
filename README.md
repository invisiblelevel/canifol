# Canifol

**Cross-platform electronics repair toolkit: component calculators, datasheet search, pinouts, SMD code database and BoardView.**
**Кросс-платформенный инструмент ремонтника: калькуляторы компонентов, поиск даташитов, распиновки, база SMD-кодов и BoardView.**

[![Version](https://img.shields.io/badge/version-1.2.0-blue.svg)](../../releases)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20Android%20%7C%20Linux-0078D6.svg)](#install)
[![Tauri](https://img.shields.io/badge/tauri-2-24C8DB.svg)](https://tauri.app/)
[![Svelte](https://img.shields.io/badge/svelte-5-FF3E00.svg)](https://svelte.dev/)

[English](#english) · [Русский](#русский)

🌐 **Landing page:** [invisiblelevel.github.io/canifol](https://invisiblelevel.github.io/canifol/)

---

## English

Canifol is a free tool for electronics repair technicians. Component calculators, datasheet search, pinouts, an SMD code database and a full-featured **BoardView** right inside the app. Works fully offline after the first launch.

**Available on Windows, Android and Linux.** The desktop build ships as a standard Windows installer (`.exe`), the mobile build as a standalone Android APK (`.apk`), and the Linux build as `.deb`, `.rpm` and `.AppImage` (auto-built via GitHub Actions). All files are attached to every release.

### What's new in 1.2.0

- **BoardView** — open `.fz`, `.cae`, `.cad`, `.bdv` and `.asc` files directly in Canifol.
- **Net highlighting** — click any pin to highlight the entire net (green = selected, purple = connected). This is the killer feature for finding shorts.
- **Component search** — type `Q21`, `R123`, or `U5` to instantly locate and highlight a component.
- **Layers** — toggle pins, traces, components, and routing on/off.
- **Fullscreen** — view the board without the UI.
- **Touch support** — pinch-to-zoom and pan on Android.
- **Supported formats:**
  - **FZ** (ASUS / ASRock) — pins + components.
  - **CAD** (GenCAD) — pins + components + traces + pads.
  - **BDV** (eM-Test Expert / Honhan) — pins + components with native outlines.

### Features

- **Resistors** — SMD code, color codes, Ohm's law, series/parallel, voltage divider, resistor picker.
- **Capacitors** — marking, SMD code, reactance (Xc), connections.
- **Inductors** — marking, reactance (Xl), connections, Wheeler geometry.
- **BMS** — balancing resistor, S/P pack, current shunt, MOSFET protection.
- **Transistors** — analog search (DigiKey + LCSC + Soviet database), parametric search, base resistor calculator.
- **Datasheets** — search via DigiKey and LCSC, download PDFs with local caching, built-in PDF viewer with fullscreen mode.
- **Pinouts** — connectors (USB Type-A/B/C, USB 3.0, Mini-B, Micro-B, HDMI, DisplayPort, VGA, Jack 3.5 TRS/TRRS, ATX 24-pin, SATA Power, RJ45, PS/2), imported ICs (LM358, NE555, LM7805, LM317, LM7812, IRFZ44N, TL494, NE5532, 74HC595, ATmega328P, TDA2030, TDA7294, TDA2822, LM386, PC817, MC34063, 74HC14, CH340G, MAX232, ULN2003, L293D, DS3231, LM2596, AMS1117, ESP-01), Soviet microchips (К155, К561, КР142, К140, КР1006, КР1533, КР580, and more).
- **SMD Code Search** — database of 270 974 components with 119 290 unique codes. Search by the code printed on the package and get the model, type, package and parameters instantly.
- **BoardView** — see above.
- **Localization** — English and Russian, with automatic system language detection.
- **Settings** — interface zoom (70–150%), datasheet cache size and cleanup.
- **Dark theme** by default.

### Tech Stack

Tauri 2 · Svelte 5 (runes) · Rust · Vite · `svelte-pdf-view` · `pcbrepair` · `gencad`.

### Install

**Windows:** Download the installer (`Canifol Setup ...exe`) from [Releases](../../releases), run it, install.

**Android:** Download the `.apk` from [Releases](../../releases), allow installation from unknown sources, install.

**Linux:** Download `.deb`, `.rpm` or `.AppImage` from [Releases](../../releases).

> **Note:** On first launch the app needs internet to fetch datasheets and analog data. After that, calculators work fully offline.

### Build from Source

Requirements: [Node.js](https://nodejs.org/) 18+, [Rust](https://rustup.rs/) 1.75+, Windows 10/11 (WebView2) or Android SDK + NDK or Linux dependencies.

1. `git clone https://github.com/invisiblelevel/canifol.git`
2. `cd canifol`
3. `npm install`
4. `npm run tauri dev` — development (desktop)
5. `npm run tauri build` — release build (Windows)
6. `npm run tauri android build -- --apk` — release build (Android APK)

The Windows installer will appear in `src-tauri/target/release/bundle/nsis/`. The Android APK will appear in `src-tauri/gen/android/app/build/outputs/apk/universal/release/`.

### License

MIT — see [LICENSE](LICENSE).

### Author

INV.LVL · [@invisiblelevel](https://github.com/invisiblelevel)

🌐 [Landing page](https://invisiblelevel.github.io/canifol/)

---

## Русский

Canifol — бесплатный инструмент для ремонтников электроники. Калькуляторы компонентов, поиск даташитов, распиновки, база SMD-кодов и полноценный **BoardView** прямо в приложении. После первого запуска работает полностью оффлайн.

**Доступно на Windows, Android и Linux.** Десктопная версия поставляется как обычный установщик (`.exe`), мобильная — как отдельный APK (`.apk`), Linux — как `.deb`, `.rpm` и `.AppImage` (собираются автоматически через GitHub Actions). Все файлы прикрепляются к каждому релизу.

### Что нового в 1.2.0

- **BoardView** — открывай `.fz`, `.cae`, `.cad`, `.bdv` и `.asc` файлы прямо в Canifol.
- **Подсветка цепей** — клик по пину подсвечивает всю цепь (зелёный = выбранный, фиолетовые = связанные). Это главная фича для поиска короткого замыкания.
- **Поиск по компонентам** — вводишь `Q21`, `R123` или `U5` — компонент мгновенно находится и подсвечивается.
- **Слои** — включение/выключение пинов, дорожек, компонентов и трассировки.
- **Fullscreen** — просмотр платы без интерфейса.
- **Тач-управление** — pinch-to-zoom и панорамирование на Android.
- **Поддерживаемые форматы:**
  - **FZ** (ASUS / ASRock) — пины + компоненты.
  - **CAD** (GenCAD) — пины + компоненты + дорожки + пады.
  - **BDV** (eM-Test Expert / Honhan) — пины + компоненты с родными контурами.

### Что умеет

- **Резисторы** — SMD-код, цвета, закон Ома, соединения, делитель, подбор.
- **Конденсаторы** — маркировка, SMD, реактивное сопротивление (Xc), соединения.
- **Катушки** — маркировка, реактивное сопротивление (Xl), соединения, геометрия (Уилер).
- **BMS** — балансир, S/P сборка, шунт, защита на MOSFET.
- **Транзисторы** — поиск аналога (DigiKey + LCSC + советская база), подбор по параметрам, расчёт базового резистора.
- **Даташиты** — поиск через DigiKey и LCSC, скачивание PDF с локальным кэшем, встроенный просмотрщик с полноэкранным режимом.
- **Распиновки** — разъёмы (USB Type-A/B/C, USB 3.0, Mini-B, Micro-B, HDMI, DisplayPort, VGA, Jack 3.5 TRS/TRRS, ATX 24-pin, SATA Power, RJ45, PS/2), импортные микросхемы (LM358, NE555, LM7805, LM317, LM7812, IRFZ44N, TL494, NE5532, 74HC595, ATmega328P, TDA2030, TDA7294, TDA2822, LM386, PC817, MC34063, 74HC14, CH340G, MAX232, ULN2003, L293D, DS3231, LM2596, AMS1117, ESP-01), советские микросхемы (К155, К561, КР142, К140, КР1006, КР1533, КР580 и др.).
- **Поиск по SMD-кодам** — база на 270 974 компонента с 119 290 уникальными кодами. Мгновенный поиск по коду с корпуса, с моделью, типом, корпусом и параметрами.
- **BoardView** — см. выше.
- **Локализация** — русский и английский, с автоопределением языка системы.
- **Настройки** — масштаб интерфейса (70–150%), размер кэша даташитов и его очистка.
- **Тёмная тема** по умолчанию.

### Стек

Tauri 2 · Svelte 5 (руны) · Rust · Vite · `svelte-pdf-view` · `pcbrepair` · `gencad`.

### Установка

**Windows:** Скачай установщик (`Canifol Setup ...exe`) из [Releases](../../releases), запусти, установи.

**Android:** Скачай `.apk` из [Releases](../../releases), разреши установку из неизвестных источников, установи.

**Linux:** Скачай `.deb`, `.rpm` или `.AppImage` из [Releases](../../releases).

> **Важно:** При первом запуске нужен интернет, чтобы загрузить даташиты и базу аналогов. После этого калькуляторы работают полностью оффлайн.

### Сборка из исходников

Требования: [Node.js](https://nodejs.org/) 18+, [Rust](https://rustup.rs/) 1.75+, Windows 10/11 (WebView2) или Android SDK + NDK или Linux-зависимости.

1. `git clone https://github.com/invisiblelevel/canifol.git`
2. `cd canifol`
3. `npm install`
4. `npm run tauri dev` — dev-режим (десктоп)
5. `npm run tauri build` — релиз (Windows)
6. `npm run tauri android build -- --apk` — релиз (Android APK)

Windows-установщик появится в `src-tauri/target/release/bundle/nsis/`. Android APK — в `src-tauri/gen/android/app/build/outputs/apk/universal/release/`.

### Лицензия

MIT — см. [LICENSE](LICENSE).

### Автор

INV.LVL · [@invisiblelevel](https://github.com/invisiblelevel)

🌐 [Лендинг](https://invisiblelevel.github.io/canifol/)