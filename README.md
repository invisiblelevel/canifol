# Canifol

**Инструмент ремонтника: калькуляторы компонентов и поиск даташитов.**

[![Version](https://img.shields.io/badge/version-1.0.0-blue.svg)](../../releases)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20Android-0078D6.svg)](#install)
[![Tauri](https://img.shields.io/badge/tauri-2-24C8DB.svg)](https://tauri.app/)
[![Svelte](https://img.shields.io/badge/svelte-5-FF3E00.svg)](https://svelte.dev/)

[English](#english) · [Русский](#русский)

---

## English

Canifol is a free tool for electronics repair technicians. Component calculators, datasheet search and PDF viewer right inside the app. Works offline after the first launch.

### Features

- **Resistors** — SMD code, color codes, Ohm's law, series/parallel, voltage divider, resistor picker.
- **Capacitors** — marking, SMD code, reactance (Xc), connections.
- **Inductors** — marking, reactance (Xl), connections, Wheeler geometry.
- **BMS** — balancing resistor, S/P pack, current shunt, MOSFET protection.
- **Transistors** — analog search (DigiKey + LCSC + Soviet database), parametric search, base resistor calculator.
- **Datasheets** — search via DigiKey and LCSC, download PDFs with local caching, built-in PDF viewer.
- **Localization** — English and Russian, with automatic system language detection.
- **Settings** — interface zoom (70–150%), datasheet cache size and cleanup.
- **Dark theme** by default.

### Tech Stack

Tauri 2 · Svelte 5 (runes) · Rust · Vite · `svelte-pdf-view`.

### Install

**Windows:** Download the installer from [Releases](../../releases), run it, install.

**Android:** Download the `.apk` from [Releases](../../releases), allow installation from unknown sources, install.

> **Note:** On first launch the app needs internet to fetch datasheets and analog data. After that, calculators work fully offline.

### Build from source

Requirements: [Node.js](https://nodejs.org/) 18+, [Rust](https://rustup.rs/) 1.75+, Windows 10/11 (WebView2) or Android SDK + NDK.

```bash
git clone https://github.com/invisiblelevel/canifol.git
cd canifol
npm install
npm run tauri dev          # development (desktop)
npm run tauri build        # release build (Windows)
npm run tauri android build -- --apk   # release build (Android APK)
The Windows installer will appear in src-tauri/target/release/bundle/nsis/. The Android APK will appear in src-tauri/gen/android/app/build/outputs/apk/universal/release/.

License
MIT — see LICENSE.

Author
INV.LVL · @invisiblelevel

Русский
Canifol — бесплатный инструмент для ремонтников электроники. Калькуляторы компонентов, поиск даташитов и просмотр PDF прямо в приложении. После первого запуска работает полностью оффлайн.

Что умеет
Резисторы — SMD-код, цвета, закон Ома, соединения, делитель, подбор.

Конденсаторы — маркировка, SMD, реактивное сопротивление (Xc), соединения.

Катушки — маркировка, реактивное сопротивление (Xl), соединения, геометрия (Уилер).

BMS — балансир, S/P сборка, шунт, защита на MOSFET.

Транзисторы — поиск аналога (DigiKey + LCSC + советская база), подбор по параметрам, расчёт базового резистора.

Даташиты — поиск через DigiKey и LCSC, скачивание PDF с локальным кэшем, встроенный просмотрщик.

Локализация — русский и английский, с автоопределением языка системы.

Настройки — масштаб интерфейса (70–150%), размер кэша даташитов и его очистка.

Тёмная тема по умолчанию.

Стек
Tauri 2 · Svelte 5 (руны) · Rust · Vite · svelte-pdf-view.

Установка
Windows: Скачай установщик из Releases, запусти, установи.

Android: Скачай .apk из Releases, разреши установку из неизвестных источников, установи.

Важно: При первом запуске нужен интернет, чтобы загрузить даташиты и базу аналогов. После этого калькуляторы работают полностью оффлайн.

Сборка из исходников
Требования: Node.js 18+, Rust 1.75+, Windows 10/11 (WebView2) или Android SDK + NDK.

bash
git clone https://github.com/invisiblelevel/canifol.git
cd canifol
npm install
npm run tauri dev          # dev-режим (десктоп)
npm run tauri build        # релиз (Windows)
npm run tauri android build -- --apk   # релиз (Android APK)
Windows-установщик появится в src-tauri/target/release/bundle/nsis/. Android APK — в src-tauri/gen/android/app/build/outputs/apk/universal/release/.

Лицензия
MIT — см. LICENSE.

Автор
INV.LVL · @invisiblelevel