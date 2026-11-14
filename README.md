# Canifol

> ### 📚 **KNOWLEDGE BASE: 6,000+ FILES (10+ GB) OF SCHEMATICS AND BOARDVIEWS FOR LAPTOPS, GPUS AND CONSOLES — RIGHT INSIDE THE APP.**
> ### 💾 **NO NEED TO DOWNLOAD ARCHIVES — EVERYTHING OPENS IN ONE CLICK.**

**Cross-platform electronics repair toolkit: component calculators, datasheet search, pinouts, SMD code database, BoardView, sketch pad, RSS feed and Knowledge Base.**
**Кросс-платформенный инструмент ремонтника: калькуляторы компонентов, поиск даташитов, распиновки, база SMD-кодов, BoardView, набросок схем, RSS-лента и База знаний.**
**跨平台电子维修工具包：元器件计算器、数据表搜索、引脚图、SMD 代码数据库、BoardView、草图板、RSS 订阅和知识库。**

[![Version](https://img.shields.io/badge/version-1.6.0-blue.svg)](../../releases)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20Android%20%7C%20Linux-0078D6.svg)](#install)
[![Tauri](https://img.shields.io/badge/tauri-2-24C8DB.svg)](https://tauri.app/)
[![Svelte](https://img.shields.io/badge/svelte-5-FF3E00.svg)](https://svelte.dev/)

[English](#english) · [Русский](#русский) · [中文](#中文)

🌐 **Landing page:** [invisiblelevel.github.io/canifol](https://invisiblelevel.github.io/canifol/)

---

## English

Canifol is a free tool for electronics repair technicians. Component calculators, datasheet search, pinouts, an SMD code database, a full-featured **BoardView**, a **sketch pad** for drawing schematics, an **RSS feed** with repair videos, and a **Knowledge Base** right inside the app. Works fully offline after the first launch.

**Available on Windows, Android and Linux.** The desktop build ships as a standard Windows installer (`.exe`), the mobile build as a standalone Android APK (`.apk`), and the Linux build as `.deb`, `.rpm` and `.AppImage` (auto-built via GitHub Actions). All files are attached to every release.

### What's new in 1.6.0

- **Sketch pad (Набросок)** — draw schematics with a pencil, wires, points, and text. Shape recognition (resistor, capacitor, inductor, diode, ground, VCC, transistor, microchip) based on geometric features. **Stamps** let you place components from a palette with one click. **Background image**: load a photo of a PCB and annotate it. Zoom, pan, pinch-zoom on mobile. Undo/redo, export to PNG. All strings localized (RU/EN/ZH).
- **Offline database** — download the entire Knowledge Base (10+ GB) to your local disk. Choose any folder on desktop. On Android, files are saved to the app's internal storage. **Live progress bar** with percentage, speed, ETA, and the name of the current file.
- **Fourth datasheet source: TME** — search now works through **DigiKey, LCSC, Farnell and TME**. TME uses OAuth 2.0. Cloudflare-protected TME links open in the system browser.
- **Expanded pinouts** — added Arduino Mega 2560, Raspberry Pi Pico, MAX485, MCP2515, MCP23017, PCF8574, ESP32-CAM, NRF24L01+, HM-10, RFID-RC522.
- **BoardView light theme** — auto-syncs with the app's light/dark theme. Manual palette toggle cycles through Canifol → OpenBoardView → Light.
- **Light theme for the whole app** — switch in Settings or via the sun/moon icon in the header.
- **Localization** — English, Russian, Chinese. Automatic system language detection. All sketch and database strings are localized.
- **Hardcoded labels fixed** — all BoardView layer labels, sidebar theme toggle, and database controls now use i18n keys.

### Features

- **Resistors** — SMD code, color codes, Ohm's law, series/parallel, voltage divider, resistor picker.
- **Capacitors** — marking, SMD code, reactance (Xc), connections, ESR check.
- **Inductors** — marking, reactance (Xl), connections, Wheeler geometry.
- **BMS** — balancing resistor, S/P pack, current shunt, MOSFET protection.
- **Transistors** — analog search (DigiKey + LCSC + Soviet database), parametric search, base resistor calculator.
- **Datasheets** — search via **DigiKey, LCSC, Farnell and TME**, download PDFs with local caching, offline favorites, built-in PDF viewer.
- **Pinouts** — connectors, imported ICs (including Arduino Mega, Raspberry Pi Pico, MCP2515, MCP23017, etc.), Soviet microchips, and development boards.
- **SMD Code Search** — database of 270,974 components. Search by the code printed on the package.
- **Favorites** — save SMD codes, pinouts, and datasheets. Offline PDF support.
- **BoardView** — open `.fz`, `.cae`, `.cad`, `.bdv`, `.asc`, `.pcb`, `.tvw`, and `.brd` files, highlight nets, search components, toggle layers (TOP / BOTTOM / Both), three color themes (Canifol, OpenBoardView, Light).
- **Sketch pad** — draw schematics with pencil/wire/point/text, stamp components, load a PCB photo as background, pan/zoom/pinch, export to PNG.
- **RSS feed** — YouTube videos (RU/EN) and Bilibili videos (ZH) from repair channels. Native fullscreen on Android, iframe on desktop.
- **Knowledge Base** — 6,000+ schematics and BoardViews (10+ GB) for laptops, GPUs and consoles. Synced from Google Drive with MEGA fallback, smart search (by model, category and file name), built-in PDF and BoardView viewers. **Download the entire base offline** with progress, speed and ETA.
- **Localization** — English, Russian, and Chinese, with automatic system language detection.
- **Settings** — interface zoom (70–150%), light/dark theme, unified cache size and cleanup, offline favorites cleanup, RSS cache cleanup, local database size and cleanup.

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

Canifol — бесплатный инструмент для ремонтников электроники. Калькуляторы компонентов, поиск даташитов, распиновки, база SMD-кодов, полноценный **BoardView**, **набросок схем**, **RSS-лента** с видео по ремонту и **База знаний** прямо в приложении. После первого запуска работает полностью оффлайн.

**Доступно на Windows, Android и Linux.** Десктопная версия поставляется как обычный установщик (`.exe`), мобильная — как отдельный APK (`.apk`), Linux — как `.deb`, `.rpm` и `.AppImage`. Все файлы прикрепляются к каждому релизу.

### Что нового в 1.6.0

- **Набросок (Sketch)** — рисуй схемы карандашом, проводами, точками и текстом. Распознавание фигур (резистор, конденсатор, катушка, диод, земля, VCC, транзистор, микросхема) на основе геометрических признаков. **Штампы** позволяют ставить элементы из палитры одним кликом. **Фоновое изображение**: загрузи фото платы и делай пометки поверх. Зум, панорамирование, пинч-зум на мобилке. Undo/redo, экспорт в PNG. Все строки локализованы (RU/EN/ZH).
- **Офлайн-база** — скачай всю Базу знаний (10+ ГБ) на локальный диск. Выбор папки на десктопе. На Android — во внутреннюю память приложения. **Живой прогресс-бар** с процентами, скоростью, ETA и именем текущего файла.
- **Четвёртый источник даташитов: TME** — поиск теперь идёт через **DigiKey, LCSC, Farnell и TME**. TME использует OAuth 2.0. Ссылки, защищённые Cloudflare, открываются в системном браузере.
- **Расширение распиновок** — добавлены Arduino Mega 2560, Raspberry Pi Pico, MAX485, MCP2515, MCP23017, PCF8574, ESP32-CAM, NRF24L01+, HM-10, RFID-RC522.
- **Светлая тема для BoardView** — автосинхронизация с общей темой приложения. Ручной переключатель палитры циклически меняет Canifol → OpenBoardView → Light.
- **Светлая тема для всего приложения** — переключатель в Настройках или иконка солнца/луны в шапке.
- **Локализация** — русский, английский, китайский. Автоопределение языка системы. Все строки наброска и базы локализованы.
- **Убраны хардкоды** — все подписи слоёв BoardView, переключатель темы в сайдбаре и элементы управления базой теперь берутся из локализаций.

### Что умеет

- **Резисторы** — SMD-код, цвета, закон Ома, соединения, делитель, подбор.
- **Конденсаторы** — маркировка, SMD, реактивное сопротивление (Xc), соединения, ESR.
- **Катушки** — маркировка, реактивное сопротивление (Xl), соединения, геометрия (Уилер).
- **BMS** — балансир, S/P сборка, шунт, защита на MOSFET.
- **Транзисторы** — поиск аналога (DigiKey + LCSC + советская база), подбор по параметрам, расчёт базового резистора.
- **Даташиты** — поиск через **DigiKey, LCSC, Farnell и TME**, скачивание PDF с локальным кэшем, офлайн-избранное, встроенный просмотрщик.
- **Распиновки** — разъёмы, импортные микросхемы (включая Arduino Mega, Raspberry Pi Pico, MCP2515, MCP23017 и др.), советские, платы разработчика.
- **Поиск по SMD-кодам** — база на 270 974 компонента.
- **Избранное** — сохранение SMD-кодов, распиновок и даташитов. Офлайн-доступ к PDF.
- **BoardView** — открытие `.fz`, `.cae`, `.cad`, `.bdv`, `.asc`, `.pcb`, `.tvw` и `.brd` файлов, подсветка цепей, поиск компонентов, слои (TOP / BOTTOM / Оба), три цветовые темы (Canifol, OpenBoardView, Light).
- **Набросок** — рисование схем карандашом/проводом/точкой/текстом, штампы компонентов, загрузка фото платы как фона, зум/панорамирование/пинч, экспорт в PNG.
- **RSS-лента** — видео с YouTube (RU/EN) и Bilibili (ZH) каналов по ремонту. Нативный фуллскрин на Android, iframe на десктопе.
- **База знаний** — 6 000+ схем и BoardView (10+ ГБ) для ноутбуков, видеокарт и консолей. Синхронизируется с Google Drive (с фоллбэком на MEGA), умный поиск (по модели, категории и имени файла), встроенный просмотр PDF и BoardView. **Скачивание всей базы офлайн** с прогрессом, скоростью и ETA.
- **Локализация** — русский, английский, китайский. Автоопределение языка системы.
- **Настройки** — масштаб интерфейса, светлая/тёмная тема, размер единого кэша и его очистка, очистка офлайн-избранного, очистка кэша RSS, размер и очистка локальной базы.

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

Canifol 是一款免费的电子维修技术人员工具。内置元器件计算器、数据表搜索、引脚图、SMD 代码数据库、功能齐全的 **BoardView**、**草图板**（绘制原理图）、**RSS 订阅**（维修视频）以及 **知识库**。首次启动后完全支持离线使用。

**支持 Windows、Android 和 Linux。** 桌面版为标准 Windows 安装程序 (`.exe`)，移动版为独立的 Android APK (`.apk`)，Linux 版为 `.deb`、`.rpm` 和 `.AppImage`。所有文件均附在每次发布中。

### 1.6.0 新功能

- **草图板 (Sketch)** — 用铅笔、导线、点和文本绘制原理图。基于几何特征识别图形（电阻器、电容器、电感器、二极管、接地、VCC、晶体管、微芯片）。**图章**可从调色板一键放置组件。**背景图像**：加载 PCB 照片并在其上做标注。支持缩放、平移、移动端双指缩放。支持撤销/重做、导出为 PNG。所有字符串均已本地化（俄语/英语/中文）。
- **离线数据库** — 将整个知识库（10+ GB）下载到本地磁盘。桌面端可选择任意文件夹。Android 上文件保存在应用内部存储中。**实时进度条**显示百分比、速度、预计剩余时间和当前文件名。
- **第四个数据表源：TME** — 搜索现在支持 **DigiKey、LCSC、Farnell 和 TME**。TME 使用 OAuth 2.0。受 Cloudflare 保护的 TME 链接会在系统浏览器中打开。
- **扩展引脚图** — 添加了 Arduino Mega 2560、Raspberry Pi Pico、MAX485、MCP2515、MCP23017、PCF8574、ESP32-CAM、NRF24L01+、HM-10、RFID-RC522。
- **BoardView 浅色主题** — 与应用的浅色/深色主题自动同步。手动调色板切换循环切换 Canifol → OpenBoardView → Light。
- **整个应用支持浅色主题** — 可在设置中切换，或通过标题栏中的太阳/月亮图标切换。
- **本地化** — 英语、俄语、中文。自动检测系统语言。所有草图板和数据库字符串均已本地化。
- **移除硬编码** — 所有 BoardView 图层标签、侧边栏主题切换和数据库控件现在使用 i18n 键。

### 功能

- **电阻器** — SMD 代码、色环、欧姆定律、串并联、分压器、电阻选择。
- **电容器** — 标识、SMD 代码、电抗 (Xc)、连接、ESR 检查。
- **电感器** — 标识、电抗 (Xl)、连接、Wheeler 几何。
- **BMS** — 均衡电阻、串/并联电池组、电流分流器、MOSFET 保护。
- **晶体管** — 模拟搜索（DigiKey + LCSC + 苏联数据库）、参数搜索、基极电阻计算器。
- **数据表** — 通过 **DigiKey、LCSC、Farnell 和 TME** 搜索，下载 PDF 并本地缓存，离线收藏，内置 PDF 查看器。
- **引脚图** — 连接器、进口 IC（包括 Arduino Mega、Raspberry Pi Pico、MCP2515、MCP23017 等）、苏联微芯片和开发板。
- **SMD 代码搜索** — 270,974 个元器件数据库。按封装上印刷的代码进行搜索。
- **收藏夹** — 保存 SMD 代码、引脚图和数据表。支持离线 PDF。
- **BoardView** — 打开 `.fz`、`.cae`、`.cad`、`.bdv`、`.asc`、`.pcb`、`.tvw` 和 `.brd` 文件，高亮网络，搜索元器件，切换图层（TOP / BOTTOM / 两者），三种配色主题（Canifol、OpenBoardView、Light）。
- **草图板** — 用铅笔/导线/点/文本绘制原理图，图章组件，加载 PCB 照片作为背景，缩放/平移/双指缩放，导出为 PNG。
- **RSS 订阅** — 来自 YouTube（俄语/英语）和 Bilibili（中文）维修频道的视频。Android 上原生全屏，桌面端使用 iframe。
- **知识库** — 6000+ 个原理图和 BoardView（10+ GB），适用于笔记本电脑、显卡和游戏机。从 Google Drive 同步（带 MEGA 备用源），智能搜索（按型号、类别和文件名），内置查看 PDF 和 BoardView 文件。**可离线下载整个数据库**，显示进度、速度和预计剩余时间。
- **本地化** — 英语、俄语和中文，自动检测系统语言。
- **设置** — 界面缩放 (70–150%)、浅色/深色主题、统一缓存大小和清理、离线收藏夹清理、RSS 缓存清理、本地数据库大小和清理。

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