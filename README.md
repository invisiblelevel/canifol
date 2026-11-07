# Canifol

**Cross-platform electronics repair toolkit: component calculators, datasheet search, pinouts, SMD code database, BoardView, RSS feed and Knowledge Base.**
**Кросс-платформенный инструмент ремонтника: калькуляторы компонентов, поиск даташитов, распиновки, база SMD-кодов, BoardView, RSS-лента и База знаний.**
**跨平台电子维修工具包：元器件计算器、数据表搜索、引脚图、SMD 代码数据库、BoardView、RSS 订阅和知识库。**

[![Version](https://img.shields.io/badge/version-1.5.1-blue.svg)](../../releases)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20Android%20%7C%20Linux-0078D6.svg)](#install)
[![Tauri](https://img.shields.io/badge/tauri-2-24C8DB.svg)](https://tauri.app/)
[![Svelte](https://img.shields.io/badge/svelte-5-FF3E00.svg)](https://svelte.dev/)

[English](#english) · [Русский](#русский) · [中文](#中文)

🌐 **Landing page:** [invisiblelevel.github.io/canifol](https://invisiblelevel.github.io/canifol/)

---

## English

Canifol is a free tool for electronics repair technicians. Component calculators, datasheet search, pinouts, an SMD code database, a full-featured **BoardView**, an **RSS feed** with repair videos, and a **Knowledge Base** right inside the app. Works fully offline after the first launch.

**Available on Windows, Android and Linux.** The desktop build ships as a standard Windows installer (`.exe`), the mobile build as a standalone Android APK (`.apk`), and the Linux build as `.deb`, `.rpm` and `.AppImage` (auto-built via GitHub Actions). All files are attached to every release.

### What's new in 1.5.1

- **Bilibili for Chinese locale** — ZH RSS feed now shows videos from Bilibili repair channels. 6-hour cache, pagination ("Load more" button), and risk-control (HTTP 412) handling.
- **Improved Knowledge Base search** — now searches not only by category and model name, but also by **file names** inside models. Matching text is highlighted.
- **Bilibili/YouTube badges** — cards now show a platform badge in the bottom-right corner.
- **Native Android fullscreen** (from 1.5.0) — video now expands to fullscreen on Android via a custom `WebChromeClient` in `MainActivity.kt`.

### Features

- **Resistors** — SMD code, color codes, Ohm's law, series/parallel, voltage divider, resistor picker.
- **Capacitors** — marking, SMD code, reactance (Xc), connections, ESR check.
- **Inductors** — marking, reactance (Xl), connections, Wheeler geometry.
- **BMS** — balancing resistor, S/P pack, current shunt, MOSFET protection.
- **Transistors** — analog search (DigiKey + LCSC + Soviet database), parametric search, base resistor calculator.
- **Datasheets** — search via DigiKey, LCSC and Farnell, download PDFs with local caching, offline favorites, built-in PDF viewer.
- **Pinouts** — connectors, imported ICs, Soviet microchips, and development boards.
- **SMD Code Search** — database of 270,974 components. Search by the code printed on the package.
- **Favorites** — save SMD codes, pinouts, and datasheets. Offline PDF support.
- **BoardView** — open `.fz`, `.cae`, `.cad`, `.bdv`, `.asc`, `.pcb`, and `.brd` files, highlight nets, search components, toggle layers.
- **RSS feed** — YouTube videos (RU/EN) and Bilibili videos (ZH) from repair channels. Native fullscreen on Android, iframe on desktop.
- **Knowledge Base** — synced from Google Drive, with auto-updating catalog, smart search (by model, category and file name), and built-in viewing of PDFs and BoardView files.
- **Localization** — English, Russian, and Chinese, with automatic system language detection.
- **Settings** — interface zoom (70–150%), unified cache size and cleanup, offline favorites cleanup, RSS cache cleanup.

### Tech Stack

Tauri 2 · Svelte 5 (runes) · Rust · Vite · `svelte-pdf-view` · `pcbrepair` · `gencad` · `des` (for XZZ) · `encoding_rs` (for RSS).

### Install

**Windows:** Download the installer (`Canifol Setup ...exe`) from [Releases](../../releases), run it, install.
**Android:** Download the `.apk` from [Releases](../../releases), allow installation from unknown sources, install.
**Linux:** Download `.deb`, `.rpm` or `.AppImage` from [Releases](../../releases).

### Build from Source

1. `git clone https://github.com/invisiblelevel/canifol.git`
2. `cd canifol`
3. `npm install`
4. `npm run tauri dev` — development
5. `npm run tauri build` — release (Windows)
6. `npm run tauri android build -- --apk` — release (Android APK)

### License

MIT — see [LICENSE](LICENSE).

### Author

INV.LVL · [@invisiblelevel](https://github.com/invisiblelevel)

---

## Русский

Canifol — бесплатный инструмент для ремонтников электроники. Калькуляторы компонентов, поиск даташитов, распиновки, база SMD-кодов, полноценный **BoardView**, **RSS-лента** с видео по ремонту и **База знаний** прямо в приложении. После первого запуска работает полностью оффлайн.

**Доступно на Windows, Android и Linux.** Десктопная версия поставляется как обычный установщик (`.exe`), мобильная — как отдельный APK (`.apk`), Linux — как `.deb`, `.rpm` и `.AppImage`. Все файлы прикрепляются к каждому релизу.

### Что нового в 1.5.1

- **Bilibili для китайской локали** — ZH-лента теперь показывает видео с Bilibili-каналов по ремонту. Кэш на 6 часов, пагинация («Загрузить ещё»), обработка риск-контроля (HTTP 412).
- **Улучшенный поиск в Базе знаний** — теперь ищет не только по категории и названию модели, но и по **именам файлов** внутри модели. Совпадения подсвечиваются.
- **Бейджи Bilibili/YouTube** — на карточках появился бейдж платформы в правом нижнем углу.
- **Нативный фуллскрин на Android** (из 1.5.0) — видео разворачивается на весь экран через кастомный `WebChromeClient` в `MainActivity.kt`.

### Что умеет

- **Резисторы** — SMD-код, цвета, закон Ома, соединения, делитель, подбор.
- **Конденсаторы** — маркировка, SMD, реактивное сопротивление (Xc), соединения, ESR.
- **Катушки** — маркировка, реактивное сопротивление (Xl), соединения, геометрия (Уилер).
- **BMS** — балансир, S/P сборка, шунт, защита на MOSFET.
- **Транзисторы** — поиск аналога (DigiKey + LCSC + советская база), подбор по параметрам, расчёт базового резистора.
- **Даташиты** — поиск через DigiKey, LCSC и Farnell, скачивание PDF с локальным кэшем, офлайн-избранное, встроенный просмотрщик.
- **Распиновки** — разъёмы, импортные микросхемы, советские, платы разработчика.
- **Поиск по SMD-кодам** — база на 270 974 компонента.
- **Избранное** — сохранение SMD-кодов, распиновок и даташитов. Офлайн-доступ к PDF.
- **BoardView** — открытие `.fz`, `.cae`, `.cad`, `.bdv`, `.asc`, `.pcb` и `.brd` файлов, подсветка цепей, поиск компонентов, слои.
- **RSS-лента** — видео с YouTube (RU/EN) и Bilibili (ZH) каналов по ремонту. Нативный фуллскрин на Android, iframe на десктопе.
- **База знаний** — синхронизируется с Google Drive, автообновление каталога, умный поиск (по модели, категории и имени файла), встроенный просмотр PDF и BoardView.
- **Локализация** — русский, английский, китайский. Автоопределение языка системы.
- **Настройки** — масштаб интерфейса, размер единого кэша и его очистка, очистка офлайн-избранного, очистка кэша RSS.

### Стек

Tauri 2 · Svelte 5 (руны) · Rust · Vite · `svelte-pdf-view` · `pcbrepair` · `gencad` · `des` (для XZZ) · `encoding_rs` (для RSS).

### Установка

**Windows:** Скачай установщик (`Canifol Setup ...exe`) из [Releases](../../releases).
**Android:** Скачай `.apk` из [Releases](../../releases).
**Linux:** Скачай `.deb`, `.rpm` или `.AppImage` из [Releases](../../releases).

### Сборка из исходников

1. `git clone https://github.com/invisiblelevel/canifol.git`
2. `cd canifol`
3. `npm install`
4. `npm run tauri dev` — dev-режим
5. `npm run tauri build` — релиз (Windows)
6. `npm run tauri android build -- --apk` — релиз (Android APK)

### Лицензия

MIT — см. [LICENSE](LICENSE).

### Автор

INV.LVL · [@invisiblelevel](https://github.com/invisiblelevel)

---

## 中文

Canifol 是一款免费的电子维修技术人员工具。内置元器件计算器、数据表搜索、引脚图、SMD 代码数据库、功能齐全的 **BoardView**、**RSS 订阅**（维修视频）以及 **知识库**。首次启动后完全支持离线使用。

**支持 Windows、Android 和 Linux。** 桌面版为标准 Windows 安装程序 (`.exe`)，移动版为独立的 Android APK (`.apk`)，Linux 版为 `.deb`、`.rpm` 和 `.AppImage`。所有文件均附在每次发布中。

### 1.5.1 新功能

- **Bilibili 中文频道** — 中文 RSS 订阅现在显示来自 Bilibili 维修频道的视频。6 小时缓存，分页（“加载更多”按钮），以及风险控制 (HTTP 412) 处理。
- **知识库搜索改进** — 现在不仅按类别和型号名称搜索，还按型号内的**文件名**搜索。匹配的文本会高亮显示。
- **Bilibili/YouTube 徽章** — 卡片右下角现在显示平台徽章。
- **Android 原生全屏**（自 1.5.0 起）— 通过 `MainActivity.kt` 中的自定义 `WebChromeClient`，视频现在可以在 Android 上全屏播放。

### 功能

- **电阻器** — SMD 代码、色环、欧姆定律、串并联、分压器、电阻选择。
- **电容器** — 标识、SMD 代码、电抗 (Xc)、连接、ESR 检查。
- **电感器** — 标识、电抗 (Xl)、连接、Wheeler 几何。
- **BMS** — 均衡电阻、串/并联电池组、电流分流器、MOSFET 保护。
- **晶体管** — 模拟搜索（DigiKey + LCSC + 苏联数据库）、参数搜索、基极电阻计算器。
- **数据表** — 通过 DigiKey、LCSC 和 Farnell 搜索，下载 PDF 并本地缓存，离线收藏，内置 PDF 查看器。
- **引脚图** — 连接器、进口 IC、苏联微芯片和开发板。
- **SMD 代码搜索** — 270,974 个元器件数据库。按封装上印刷的代码进行搜索。
- **收藏夹** — 保存 SMD 代码、引脚图和数据表。支持离线 PDF。
- **BoardView** — 打开 `.fz`、`.cae`、`.cad`、`.bdv`、`.asc`、`.pcb` 和 `.brd` 文件，高亮网络，搜索元器件，切换图层。
- **RSS 订阅** — 来自 YouTube（俄语/英语）和 Bilibili（中文）维修频道的视频。Android 上原生全屏，桌面端使用 iframe。
- **知识库** — 从 Google Drive 同步，自动更新目录，智能搜索（按型号、类别和文件名），内置查看 PDF 和 BoardView 文件。
- **本地化** — 英语、俄语和中文，自动检测系统语言。
- **设置** — 界面缩放 (70–150%)、统一缓存大小和清理、离线收藏夹清理、RSS 缓存清理。

### 技术栈

Tauri 2 · Svelte 5 (runes) · Rust · Vite · `svelte-pdf-view` · `pcbrepair` · `gencad` · `des`（用于 XZZ）· `encoding_rs`（用于 RSS）。

### 安装

**Windows:** 从 [Releases](../../releases) 下载安装程序 (`Canifol Setup ...exe`) 并运行。
**Android:** 从 [Releases](../../releases) 下载 `.apk`，允许从未知来源安装。
**Linux:** 从 [Releases](../../releases) 下载 `.deb`、`.rpm` 或 `.AppImage`。

### 从源码构建

1. `git clone https://github.com/invisiblelevel/canifol.git`
2. `cd canifol`
3. `npm install`
4. `npm run tauri dev` — 开发模式
5. `npm run tauri build` — 发布 (Windows)
6. `npm run tauri android build -- --apk` — 发布 (Android APK)

### 许可证

MIT — 见 [LICENSE](LICENSE)。

### 作者

INV.LVL · [@invisiblelevel](https://github.com/invisiblelevel)