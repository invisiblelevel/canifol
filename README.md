# Canifol

> ### 📚 **KNOWLEDGE BASE: 6,000+ FILES (10+ GB) OF SCHEMATICS AND BOARDVIEWS FOR LAPTOPS, GPUS AND CONSOLES — RIGHT INSIDE THE APP.**
> ### 💾 **NO NEED TO DOWNLOAD ARCHIVES — EVERYTHING OPENS IN ONE CLICK.**

**Cross-platform electronics repair toolkit: component calculators, datasheet search, pinouts, SMD code database, BoardView, RSS feed and Knowledge Base.**
**Кросс-платформенный инструмент ремонтника: калькуляторы компонентов, поиск даташитов, распиновки, база SMD-кодов, BoardView, RSS-лента и База знаний.**
**跨平台电子维修工具包：元器件计算器、数据表搜索、引脚图、SMD 代码数据库、BoardView、RSS 订阅和知识库。**

[![Version](https://img.shields.io/badge/version-1.5.2-blue.svg)](../../releases)
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

### What's new in 1.5.2

- **New BoardView format: TVW (Teleplan / TestView)** — binary and ASCII-obfuscated variants. Pins, components and board outline are parsed. Layer switching (TOP / BOTTOM) now works for double-sided boards.
- **FZ / CAE parser improved** — dynamic column reading from the `A!SECTION!COL1!COL2!` header (no more shifted indices), board outline (`BOARD OUTLINE`) and traces (`ETCH`) are parsed with garbage protection.
- **Apple BRD parser** — real refdes (`U1`, `C2`) and components instead of `P1`, `P2`. Pins are grouped into components automatically.
- **XZZ (.pcb) parser** — coordinates are in mils now (the `* 250` hack is gone), layer detection for pins (`TOP` / `BOTTOM`), net table parsing.
- **Unified BoardView layer:** auto-format detection by content (not by extension), `catch_unwind` protection, auto-generated outline when the format has none, arc tessellation.
- **BoardView UI:** TOP / BOTTOM / Both switch, mirror mode for the bottom side, layer colors (TOP — blue, BOTTOM — orange), smart search that jumps to the component's layer.
- **Android BoardView:** `read_boardview_uri` uses the same parser as desktop, no temp files.
- **Help updated:** `.tvw` added to the supported formats (RU / EN / ZH), empty state and file filter.

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
- **BoardView** — open `.fz`, `.cae`, `.cad`, `.bdv`, `.asc`, `.pcb`, `.tvw`, and `.brd` files, highlight nets, search components, toggle layers (TOP / BOTTOM / Both).
- **RSS feed** — YouTube videos (RU/EN) and Bilibili videos (ZH) from repair channels. Native fullscreen on Android, iframe on desktop.
- **Knowledge Base** — 6,000+ schematics and BoardViews (10+ GB) for laptops, GPUs and consoles. Synced from Google Drive with MEGA fallback, smart search (by model, category and file name), built-in PDF and BoardView viewers.
- **Localization** — English, Russian, and Chinese, with automatic system language detection.
- **Settings** — interface zoom (70–150%), unified cache size and cleanup, offline favorites cleanup, RSS cache cleanup.

### Tech Stack

Tauri 2 · Svelte 5 (runes) · Rust · Vite · `svelte-pdf-view` · `pcbrepair` · `gencad` · `des` (for XZZ) · `encoding_rs` (for RSS) · `megalib` (for MEGA fallback).

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

> ### 📚 **БАЗА ЗНАНИЙ: 6 000+ ФАЙЛОВ (10+ ГБ) СХЕМ И BOARDVIEW ДЛЯ НОУТБУКОВ, ВИДЕОКАРТ И КОНСОЛЕЙ — ПРЯМО В ПРИЛОЖЕНИИ.**
> ### 💾 **НЕ НУЖНО КАЧАТЬ АРХИВЫ — ВСЁ ОТКРЫВАЕТСЯ В ОДИН КЛИК.**

Canifol — бесплатный инструмент для ремонтников электроники. Калькуляторы компонентов, поиск даташитов, распиновки, база SMD-кодов, полноценный **BoardView**, **RSS-лента** с видео по ремонту и **База знаний** прямо в приложении. После первого запуска работает полностью оффлайн.

**Доступно на Windows, Android и Linux.** Десктопная версия поставляется как обычный установщик (`.exe`), мобильная — как отдельный APK (`.apk`), Linux — как `.deb`, `.rpm` и `.AppImage`. Все файлы прикрепляются к каждому релизу.

### Что нового в 1.5.2

- **Новый формат BoardView: TVW (Teleplan / TestView)** — бинарный и ASCII-обфусцированный варианты. Парсятся пины, компоненты и контур платы. Переключение слоёв (TOP / BOTTOM) теперь работает для двусторонних плат.
- **Улучшен FZ / CAE-парсер** — динамическое чтение колонок из заголовка `A!SECTION!COL1!COL2!` (больше не сдвигаются индексы), читается контур платы (`BOARD OUTLINE`) и дорожки (`ETCH`) с защитой от мусора.
- **Apple BRD-парсер** — реальные refdes (`U1`, `C2`) и компоненты вместо `P1`, `P2`. Пины автоматически группируются в компоненты.
- **XZZ (.pcb)-парсер** — координаты теперь в милах (убран костыль `* 250`), разделение слоёв пинов (`TOP` / `BOTTOM`), парсинг net-таблицы.
- **Унификация BoardView:** автоопределение формата по содержимому (а не по расширению), защита через `catch_unwind`, авто-генерация контура, если формат его не даёт, разбиение дуг на отрезки.
- **UI BoardView:** переключатель TOP / BOTTOM / Оба, зеркалирование нижней стороны, цвета по слоям (TOP — голубой, BOTTOM — оранжевый), умный поиск, который переключает слой на нужный компонент.
- **Android BoardView:** `read_boardview_uri` использует тот же парсер, что и десктоп, без временных файлов.
- **Обновлена справка:** `.tvw` добавлен в список поддерживаемых форматов (RU / EN / ZH), пустое состояние и фильтр выбора файла.

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
- **BoardView** — открытие `.fz`, `.cae`, `.cad`, `.bdv`, `.asc`, `.pcb`, `.tvw` и `.brd` файлов, подсветка цепей, поиск компонентов, слои (TOP / BOTTOM / Оба).
- **RSS-лента** — видео с YouTube (RU/EN) и Bilibili (ZH) каналов по ремонту. Нативный фуллскрин на Android, iframe на десктопе.
- **База знаний** — 6 000+ схем и BoardView (10+ ГБ) для ноутбуков, видеокарт и консолей. Синхронизируется с Google Drive (с фоллбэком на MEGA), умный поиск (по модели, категории и имени файла), встроенный просмотр PDF и BoardView.
- **Локализация** — русский, английский, китайский. Автоопределение языка системы.
- **Настройки** — масштаб интерфейса, размер единого кэша и его очистка, очистка офлайн-избранного, очистка кэша RSS.

### Стек

Tauri 2 · Svelte 5 (руны) · Rust · Vite · `svelte-pdf-view` · `pcbrepair` · `gencad` · `des` (для XZZ) · `encoding_rs` (для RSS) · `megalib` (для фоллбэка на MEGA).

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

> ### 📚 **知识库：6000+ 个文件和 10+ GB 的笔记本电脑、显卡和游戏机原理图与 BoardView — 全部内置在应用中。**
> ### 💾 **无需下载压缩包 — 一键即可打开。**

Canifol 是一款免费的电子维修技术人员工具。内置元器件计算器、数据表搜索、引脚图、SMD 代码数据库、功能齐全的 **BoardView**、**RSS 订阅**（维修视频）以及 **知识库**。首次启动后完全支持离线使用。

**支持 Windows、Android 和 Linux。** 桌面版为标准 Windows 安装程序 (`.exe`)，移动版为独立的 Android APK (`.apk`)，Linux 版为 `.deb`、`.rpm` 和 `.AppImage`。所有文件均附在每次发布中。

### 1.5.2 新功能

- **全新 BoardView 格式：TVW (Teleplan / TestView)** — 支持二进制和 ASCII 混淆两种变体。解析引脚、元件和电路板轮廓。双面板现在支持图层切换 (TOP / BOTTOM)。
- **改进 FZ / CAE 解析器** — 从 `A!SECTION!COL1!COL2!` 头部动态读取列（不再有索引偏移），解析电路板轮廓（`BOARD OUTLINE`）和走线（`ETCH`）并进行垃圾防护。
- **Apple BRD 解析器** — 使用真实 refdes（`U1`、`C2`）和元件，而不是 `P1`、`P2`。引脚自动分组成元件。
- **XZZ (.pcb) 解析器** — 坐标现在使用 mil（移除了 `* 250` 的 hack），支持引脚图层检测（`TOP` / `BOTTOM`），解析网络表。
- **统一 BoardView：** 按内容自动检测格式（而不是按扩展名），使用 `catch_unwind` 保护，当格式没有轮廓时自动生成轮廓，将圆弧分割为线段。
- **BoardView UI：** TOP / BOTTOM / 两者切换、底部镜像模式、图层颜色（TOP — 蓝色，BOTTOM — 橙色）、智能搜索自动切换到元件所在图层。
- **Android BoardView：** `read_boardview_uri` 与桌面使用相同的解析器，无需临时文件。
- **更新帮助：** `.tvw` 已添加到支持的格式列表（俄语 / 英语 / 中文）、空状态和文件过滤器。

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
- **BoardView** — 打开 `.fz`、`.cae`、`.cad`、`.bdv`、`.asc`、`.pcb`、`.tvw` 和 `.brd` 文件，高亮网络，搜索元器件，切换图层（TOP / BOTTOM / 两者）。
- **RSS 订阅** — 来自 YouTube（俄语/英语）和 Bilibili（中文）维修频道的视频。Android 上原生全屏，桌面端使用 iframe。
- **知识库** — 6000+ 个原理图和 BoardView（10+ GB），适用于笔记本电脑、显卡和游戏机。从 Google Drive 同步（带 MEGA 备用源），智能搜索（按型号、类别和文件名），内置查看 PDF 和 BoardView 文件。
- **本地化** — 英语、俄语和中文，自动检测系统语言。
- **设置** — 界面缩放 (70–150%)、统一缓存大小和清理、离线收藏夹清理、RSS 缓存清理。

### 技术栈

Tauri 2 · Svelte 5 (runes) · Rust · Vite · `svelte-pdf-view` · `pcbrepair` · `gencad` · `des`（用于 XZZ）· `encoding_rs`（用于 RSS）· `megalib`（用于 MEGA 备用源）。

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