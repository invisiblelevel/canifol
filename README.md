# Canifol

**Cross-platform electronics repair toolkit: component calculators, datasheet search, pinouts and SMD code database.**
**Кросс-платформенный инструмент ремонтника: калькуляторы компонентов, поиск даташитов, распиновки и база SMD-кодов.**

[![Version](https://img.shields.io/badge/version-1.1.0-blue.svg)](../../releases)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20Android-0078D6.svg)](#install)
[![Tauri](https://img.shields.io/badge/tauri-2-24C8DB.svg)](https://tauri.app/)
[![Svelte](https://img.shields.io/badge/svelte-5-FF3E00.svg)](https://svelte.dev/)

[English](#english) · [Русский](#русский)

---

## English

Canifol is a free tool for electronics repair technicians. Component calculators, datasheet search, pinouts and an SMD code database right inside the app. Works fully offline after the first launch.

**Available on Windows and Android.** The desktop build ships as a standard Windows installer (`.exe`), and the mobile build ships as a standalone Android APK (`.apk`). Both are attached to every release.

### What's new in 1.1.0

- **Pinouts** — visual SVG diagrams for connectors (USB, HDMI, DisplayPort, VGA, ATX, SATA, Jack 3.5), imported ICs (LM, NE, TDA, 74HC, etc.) and Soviet microchips (K155, K561, KR142, etc.).
- **SMD Code Search** — database of **270 974 components** with **119 290 unique codes**. Search by the code printed on the package and get the model, type, package and parameters instantly.
- **Extended SMD database** — parsed from the SMD-CODES DATABOOK 2023 EDITION.

### Features

- **Resistors** — SMD code, color codes, Ohm's law, series/parallel, voltage divider, resistor picker.
- **Capacitors** — marking, SMD code, reactance (Xc), connections.
- **Inductors** — marking, reactance (Xl), connections, Wheeler geometry.
- **BMS** — balancing resistor, S/P pack, current shunt, MOSFET protection.
- **Transistors** — analog search (DigiKey + LCSC + Soviet database), parametric search, base resistor calculator.
- **Datasheets** — search via DigiKey and LCSC, download PDFs with local caching, built-in PDF viewer with fullscreen mode.
- **Pinouts** — connectors (USB Type-A, USB Type-C, USB 3.0, HDMI, DisplayPort, VGA, Jack 3.5 TRS/TRRS, ATX 24-pin, SATA Power), imported ICs (LM358, NE555, LM7805, LM317, LM7812, IRFZ44N, TL494, NE5532, 74HC595, ATmega328P, TDA2030, TDA7294, TDA2822, LM386, PC817, MC34063, 74HC14), Soviet microchips (К155ЛА3, К155ТМ2, К155ИД3, К155ИР1, К155АГ3, К155КП1, К155ИЕ7, К561ЛА7, К561ТМ2, К561ИЕ10, КР142ЕН5А, КР142ЕН8Б, К140УД6, К140УД7, КР1006ВИ1, КР1533ИР22, КР580ВМ80А, КР580ВВ55А, КР580ВИ53).
- **SMD Code Search** — database of 270 974 components, instant search by code, with model, type, package and parameters.
- **Localization** — English and Russian, with automatic system language detection.
- **Settings** — interface zoom (70–150%), datasheet cache size and cleanup.
- **Dark theme** by default.

### Tech Stack

Tauri 2 · Svelte 5 (runes) · Rust · Vite · `svelte-pdf-view`.

### Install

**Windows:** Download the installer (`Canifol Setup ...exe`) from [Releases](../../releases), run it, install.

**Android:** Download the `.apk` from [Releases](../../releases), allow installation from unknown sources, install.

> **Note:** On first launch the app needs internet to fetch datasheets and analog data. After that, calculators work fully offline.

### Build from Source

Requirements: [Node.js](https://nodejs.org/) 18+, [Rust](https://rustup.rs/) 1.75+, Windows 10/11 (WebView2) or Android SDK + NDK.

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

---

## Русский

Canifol — бесплатный инструмент для ремонтников электроники. Калькуляторы компонентов, поиск даташитов, распиновки и база SMD-кодов прямо в приложении. После первого запуска работает полностью оффлайн.

**Доступно на Windows и Android.** Десктопная версия поставляется как обычный установщик (`.exe`), мобильная — как отдельный APK (`.apk`). Оба файла прикрепляются к каждому релизу.

### Что нового в 1.1.0

- **Распиновки** — визуальные SVG-схемы для разъёмов (USB, HDMI, DisplayPort, VGA, ATX, SATA, Jack 3.5), импортных микросхем (LM, NE, TDA, 74HC и т.д.) и советских микросхем (К155, К561, КР142 и т.д.).
- **Поиск по SMD-кодам** — база на **270 974 компонента** с **119 290 уникальными кодами**. Ищешь по коду с корпуса — получаешь модель, тип, корпус и параметры мгновенно.
- **Расширенная SMD-база** — распарсена из SMD-CODES DATABOOK 2023 EDITION.

### Что умеет

- **Резисторы** — SMD-код, цвета, закон Ома, соединения, делитель, подбор.
- **Конденсаторы** — маркировка, SMD, реактивное сопротивление (Xc), соединения.
- **Катушки** — маркировка, реактивное сопротивление (Xl), соединения, геометрия (Уилер).
- **BMS** — балансир, S/P сборка, шунт, защита на MOSFET.
- **Транзисторы** — поиск аналога (DigiKey + LCSC + советская база), подбор по параметрам, расчёт базового резистора.
- **Даташиты** — поиск через DigiKey и LCSC, скачивание PDF с локальным кэшем, встроенный просмотрщик с полноэкранным режимом.
- **Распиновки** — разъёмы (USB Type-A, USB Type-C, USB 3.0, HDMI, DisplayPort, VGA, Jack 3.5 TRS/TRRS, ATX 24-pin, SATA Power), импортные микросхемы (LM358, NE555, LM7805, LM317, LM7812, IRFZ44N, TL494, NE5532, 74HC595, ATmega328P, TDA2030, TDA7294, TDA2822, LM386, PC817, MC34063, 74HC14), советские микросхемы (К155ЛА3, К155ТМ2, К155ИД3, К155ИР1, К155АГ3, К155КП1, К155ИЕ7, К561ЛА7, К561ТМ2, К561ИЕ10, КР142ЕН5А, КР142ЕН8Б, К140УД6, К140УД7, КР1006ВИ1, КР1533ИР22, КР580ВМ80А, КР580ВВ55А, КР580ВИ53).
- **Поиск по SMD-кодам** — база на 270 974 компонента, мгновенный поиск по коду, с моделью, типом, корпусом и параметрами.
- **Локализация** — русский и английский, с автоопределением языка системы.
- **Настройки** — масштаб интерфейса (70–150%), размер кэша даташитов и его очистка.
- **Тёмная тема** по умолчанию.

### Стек

Tauri 2 · Svelte 5 (руны) · Rust · Vite · `svelte-pdf-view`.

### Установка

**Windows:** Скачай установщик (`Canifol Setup ...exe`) из [Releases](../../releases), запусти, установи.

**Android:** Скачай `.apk` из [Releases](../../releases), разреши установку из неизвестных источников, установи.

> **Важно:** При первом запуске нужен интернет, чтобы загрузить даташиты и базу аналогов. После этого калькуляторы работают полностью оффлайн.

### Сборка из исходников

Требования: [Node.js](https://nodejs.org/) 18+, [Rust](https://rustup.rs/) 1.75+, Windows 10/11 (WebView2) или Android SDK + NDK.

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