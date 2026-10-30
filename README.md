# Canifol

**Cross-platform electronics repair toolkit: component calculators, datasheet search, pinouts, SMD code database and BoardView.**
**Кросс-платформенный инструмент ремонтника: калькуляторы компонентов, поиск даташитов, распиновки, база SMD-кодов и BoardView.**
**跨平台电子维修工具包：元器件计算器、数据表搜索、引脚图、SMD 代码数据库和 BoardView。**

[![Version](https://img.shields.io/badge/version-1.3.0-blue.svg)](../../releases)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20Android%20%7C%20Linux-0078D6.svg)](#install)
[![Tauri](https://img.shields.io/badge/tauri-2-24C8DB.svg)](https://tauri.app/)
[![Svelte](https://img.shields.io/badge/svelte-5-FF3E00.svg)](https://svelte.dev/)

[English](#english) · [Русский](#русский) · [中文](#中文)

🌐 **Landing page:** [invisiblelevel.github.io/canifol](https://invisiblelevel.github.io/canifol/)

---

## English

Canifol is a free tool for electronics repair technicians. Component calculators, datasheet search, pinouts, an SMD code database and a full-featured **BoardView** right inside the app. Works fully offline after the first launch.

**Available on Windows, Android and Linux.** The desktop build ships as a standard Windows installer (`.exe`), the mobile build as a standalone Android APK (`.apk`), and the Linux build as `.deb`, `.rpm` and `.AppImage` (auto-built via GitHub Actions). All files are attached to every release.

### What's new in 1.3.0

- **Favorites** — save SMD codes, pinouts, and datasheets. Access them all in a single tab.
- **Offline Datasheets** — download PDFs to a separate folder that is not cleared when you clear the regular cache. Works without internet.
- **SMD Favorites** — save a specific SMD component variant (uniquely identified by code + model + package) without marking all similar ones.
- **Pinouts Favorites** — save any connector, IC, or dev board and jump right back to it.
- **Cross-navigation** — open a favorite item and the app instantly switches to the correct tab with the item pre-selected.
- **UI Fixes** — improved mobile navigation for the pinouts list.

### Features

- **Resistors** — SMD code, color codes, Ohm's law, series/parallel, voltage divider, resistor picker.
- **Capacitors** — marking, SMD code, reactance (Xc), connections, ESR check.
- **Inductors** — marking, reactance (Xl), connections, Wheeler geometry.
- **BMS** — balancing resistor, S/P pack, current shunt, MOSFET protection.
- **Transistors** — analog search (DigiKey + LCSC + Soviet database), parametric search, base resistor calculator.
- **Datasheets** — search via DigiKey and LCSC, download PDFs with local caching, offline favorites, built-in PDF viewer.
- **Pinouts** — connectors, imported ICs, Soviet microchips, and development boards.
- **SMD Code Search** — database of 270,974 components. Search by the code printed on the package.
- **Favorites** — save SMD codes, pinouts, and datasheets. Offline PDF support.
- **BoardView** — open `.fz`, `.cae`, `.cad`, `.bdv` and `.asc` files, highlight nets, search components, toggle layers.
- **Localization** — English, Russian, and Chinese, with automatic system language detection.
- **Settings** — interface zoom (70–150%), datasheet cache size and cleanup, offline favorites cleanup.

### Tech Stack

Tauri 2 · Svelte 5 (runes) · Rust · Vite · `svelte-pdf-view` · `pcbrepair` · `gencad`.

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

Canifol — бесплатный инструмент для ремонтников электроники. Калькуляторы компонентов, поиск даташитов, распиновки, база SMD-кодов и полноценный **BoardView** прямо в приложении. После первого запуска работает полностью оффлайн.

**Доступно на Windows, Android и Linux.** Десктопная версия поставляется как обычный установщик (`.exe`), мобильная — как отдельный APK (`.apk`), Linux — как `.deb`, `.rpm` и `.AppImage`. Все файлы прикрепляются к каждому релизу.

### Что нового в 1.3.0

- **Избранное** — сохраняй SMD-коды, распиновки и даташиты. Всё в одной вкладке.
- **Офлайн-даташиты** — скачивай PDF в отдельную папку, которая не удаляется при очистке обычного кэша. Работает без интернета.
- **SMD-избранное** — сохраняется конкретный вариант компонента (уникальность по коду + модели + корпусу), а не все похожие.
- **Распиновки в избранном** — сохраняй любой разъём, микросхему или плату и мгновенно переходи к ним.
- **Кросс-навигация** — при открытии из избранного приложение переключается на нужную вкладку с уже выбранным элементом.
- **UI Fixes** — улучшена навигация по списку микросхем на мобильных устройствах.

### Что умеет

- **Резисторы** — SMD-код, цвета, закон Ома, соединения, делитель, подбор.
- **Конденсаторы** — маркировка, SMD, реактивное сопротивление (Xc), соединения, ESR.
- **Катушки** — маркировка, реактивное сопротивление (Xl), соединения, геометрия (Уилер).
- **BMS** — балансир, S/P сборка, шунт, защита на MOSFET.
- **Транзисторы** — поиск аналога (DigiKey + LCSC + советская база), подбор по параметрам, расчёт базового резистора.
- **Даташиты** — поиск через DigiKey и LCSC, скачивание PDF с локальным кэшем, офлайн-избранное, встроенный просмотрщик.
- **Распиновки** — разъёмы, импортные микросхемы, советские, платы разработчика.
- **Поиск по SMD-кодам** — база на 270 974 компонента.
- **Избранное** — сохранение SMD-кодов, распиновок и даташитов. Офлайн-доступ к PDF.
- **BoardView** — открытие файлов, подсветка цепей, поиск компонентов, слои.
- **Локализация** — русский, английский, китайский. Автоопределение языка системы.
- **Настройки** — масштаб интерфейса, размер кэша даташитов и его очистка, очистка офлайн-избранного.

### Стек

Tauri 2 · Svelte 5 (руны) · Rust · Vite · `svelte-pdf-view` · `pcbrepair` · `gencad`.

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

Canifol 是一款免费的电子维修技术人员工具。内置元器件计算器、数据表搜索、引脚图、SMD 代码数据库以及功能齐全的 **BoardView**。首次启动后完全支持离线使用。

**支持 Windows、Android 和 Linux。** 桌面版为标准 Windows 安装程序 (`.exe`)，移动版为独立的 Android APK (`.apk`)，Linux 版为 `.deb`、`.rpm` 和 `.AppImage`。所有文件均附在每次发布中。

### 1.3.0 新功能

- **收藏夹** — 保存 SMD 代码、引脚图和数据表。在一个选项卡中查看所有内容。
- **离线数据表** — 将 PDF 下载到单独的文件夹，清除普通缓存时不会删除该文件夹。无需网络即可使用。
- **SMD 收藏** — 保存特定的 SMD 元器件（按代码 + 型号 + 封装唯一标识），不会标记所有相似项。
- **引脚图收藏** — 保存任意连接器、IC 或开发板，并快速跳转回该页面。
- **交叉导航** — 打开收藏项时，应用会立即切换到正确的选项卡并预先选中该项。
- **UI 修复** — 改进移动端引脚图列表的导航。

### 功能

- **电阻器** — SMD 代码、色环、欧姆定律、串并联、分压器、电阻选择。
- **电容器** — 标识、SMD 代码、电抗 (Xc)、连接、ESR 检查。
- **电感器** — 标识、电抗 (Xl)、连接、Wheeler 几何。
- **BMS** — 均衡电阻、串/并联电池组、电流分流器、MOSFET 保护。
- **晶体管** — 模拟搜索（DigiKey + LCSC + 苏联数据库）、参数搜索、基极电阻计算器。
- **数据表** — 通过 DigiKey 和 LCSC 搜索，下载 PDF 并本地缓存，离线收藏，内置 PDF 查看器。
- **引脚图** — 连接器、进口 IC、苏联微芯片和开发板。
- **SMD 代码搜索** — 270,974 个元器件数据库。按封装上印刷的代码进行搜索。
- **收藏夹** — 保存 SMD 代码、引脚图和数据表。支持离线 PDF。
- **BoardView** — 打开 `.fz`、`.cae`、`.cad`、`.bdv` 和 `.asc` 文件，高亮网络，搜索元器件，切换图层。
- **本地化** — 英语、俄语和中文，自动检测系统语言。
- **设置** — 界面缩放 (70–150%)、数据表缓存大小和清理、离线收藏夹清理。

### 技术栈

Tauri 2 · Svelte 5 (runes) · Rust · Vite · `svelte-pdf-view` · `pcbrepair` · `gencad`。

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