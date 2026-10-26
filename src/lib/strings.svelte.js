import { localeStore } from './stores/locale.svelte.js';

const ru = {
  app: { name: 'Canifol', tagline: 'Инструмент ремонтника' },
  nav: { menu: 'Меню', close: 'Закрыть', sections: 'Разделы', settings: 'Настройки' },
  sections: {
    resistors: 'Резисторы', capacitors: 'Конденсаторы', inductors: 'Катушки',
    bms: 'BMS', transistors: 'Транзисторы', datasheets: 'Даташиты'
  },
  common: {
    comingSoon: 'Скоро здесь будет калькулятор',
    comingSoonHint: 'Раздел в разработке. Пока переключайся на другие вкладки.',
    version: 'Версия', language: 'Язык', theme: 'Тема',
    themeDark: 'Тёмная', themeLight: 'Светлая',
    computed: 'вычислено', reset: 'Сброс',
    pcs: 'шт.', result: 'Результат', enterValues: 'Заполни поля', ready: 'Готово'
  },
  buttons: {
    search: 'Искать', copy: 'Копировать', copied: 'Скопировано',
    reset: 'Сброс', clear: 'Очистить', openPdf: 'Открыть PDF',
    download: 'Скачать', back: 'Назад', add: 'Добавить', remove: 'Удалить',
    clearCache: 'Очистить кэш',
    confirm: 'Подтвердить',
    cancel: 'Отмена'
  },
  colors: {
    black: 'Чёрный', brown: 'Коричневый', red: 'Красный', orange: 'Оранжевый',
    yellow: 'Жёлтый', green: 'Зелёный', blue: 'Синий', violet: 'Фиолетовый',
    gray: 'Серый', white: 'Белый', gold: 'Золотой', silver: 'Серебряный'
  },
  transistorTypes: { mosfet: 'MOSFET', bjt: 'Биполярные' },
  markingTabs: { code: 'Код', smd: 'SMD', color: 'Цвета' },
  units: {
    hz: 'Гц', khz: 'кГц', mhz: 'МГц',
    pf: 'пФ', nf: 'нФ', uf: 'мкФ', mf: 'мФ', f: 'Ф',
    ohm: 'Ом', kohm: 'кОм', mohm: 'МОм', mohm_short: 'мОм',
    uh: 'мкГн', nh: 'нГн', mh: 'мГн', h: 'Гн',
    v: 'В', a: 'А', w: 'Вт', mm: 'мм',
    ah: 'А·ч', wh: 'Вт·ч', ma: 'мА', uv: 'мкВ', mv: 'мВ',
    b: 'Б', kb: 'КБ', mb: 'МБ', gb: 'ГБ'
  },

  settings: {
    title: 'Настройки',
    scaleTitle: 'Масштаб интерфейса',
    scaleHint: 'Увеличь, если у тебя 4K-монитор или плохое зрение.',
    cacheTitle: 'Кэш даташитов',
    cacheSize: 'Размер кэша',
    cacheHint: 'Скачанные PDF хранятся локально и открываются мгновенно при повторном запросе. Очистка освободит место, но при следующем поиске PDF придётся скачивать заново.',
    cacheCleared: 'Кэш очищен',
    cacheEmpty: 'Кэш пуст',
    confirmClear: 'Ты уверен, что хочешь удалить все скачанные PDF? Это действие нельзя отменить.'
  },

  resistors: {
    title: 'Калькуляторы резисторов',
    subtitle: 'Цветовая маркировка, SMD-коды, соединения, закон Ома',
    sub: { smd: 'SMD-код', color: 'Цвета', ohm: 'Закон Ома', connections: 'Соединения', divider: 'Делитель', pick: 'Подбор' },
    labels: {
      smd: 'SMD-код', smdPlaceholder: '103, 1002, 4R7, 01C', smdHint: '3, 4 знака, R-нотация или EIA-96', smdError: 'Код не распознан',
      standard3: '3-значный код', standard4: '4-значный код', eia96: 'EIA-96 (1%)',
      bands: 'Количество полос', tolerance: 'Допуск', multiplier: 'Множитель', digit1: 'Цифра 1', digit2: 'Цифра 2', digit3: 'Цифра 3', tcr: 'ТКС',
      band1: 'Полоса 1', band2: 'Полоса 2', invalidColors: 'Выбранная комбинация цветов некорректна',
      voltage: 'Напряжение (U)', current: 'Ток (I)', resistance: 'Сопротивление (R)', power: 'Мощность (P)',
      enterTwo: 'Введи любые две величины', enterOneMore: 'Введи ещё одну величину', ready: 'Готово',
      calcFailed: 'Не удалось рассчитать — проверь значения', enterExactlyTwo: 'Введи ровно две величины для расчёта',
      result: 'Результат', series: 'Последовательное', parallel: 'Параллельное',
      addResistor: '+ Добавить резистор', formula: 'соединение',
      uin: 'Uвх', uout: 'Uвых', dividerRatio: 'Коэффициент деления', dividerCurrent: 'Ток через делитель',
      pickVoltage: 'Напряжение', pickCurrent: 'Ток', pickResistance: 'Сопротивление',
      recommendedPower: 'Рекомендуемая мощность', nearestE24: 'Ближайшие E24', about: 'Описание'
    }
  },

  capacitors: {
    title: 'Калькуляторы конденсаторов',
    subtitle: 'Маркировка, SMD-коды, реактивное сопротивление',
    sub: { marking: 'Маркировка', reactance: 'Xc', connections: 'Соединения' },
    labels: {
      marking: 'Маркировка', code: 'Код', smd: 'SMD', colors: 'Цвета',
      codeLabel: 'Код маркировки',
      codePlaceholder: '104, 4R7, 3n3, 223K, 104 50V',
      codeHint: 'Цифровой код, R-нотация, буквы p/n/u, допуск, напряжение',
      smdCapPlaceholder: 'J3, A0, 105',
      smdCapHint: 'Буква + цифра (тантал) или 3 знака (керамика)',
      tant: 'Танталовый', ceramic: 'Керамика',
      invalidColorsCap: 'Комбинация цветов некорректна',
      frequency: 'Частота (f)', capacitance: 'Ёмкость (C)', reactanceC: 'Сопротивление (Xc)',
      enterTwo: 'Введи любые две величины',
      enterOneMore: 'Введи ещё одну величину',
      enterOneCapacity: 'Введи хотя бы одну ёмкость',
      calcFailed: 'Не удалось рассчитать — проверь значения',
      enterExactlyTwo: 'Введи ровно две величины для расчёта',
      ready: 'Готово',
      result: 'Результат', series: 'Последовательное', parallel: 'Параллельное',
      addCapacitor: '+ Добавить',
      bandCount: 'Количество полос', digit1: 'Цифра 1', digit2: 'Цифра 2',
      multiplier: 'Множитель', tolerance: 'Допуск', voltageLabel: 'Напряжение',
      color_interpretation: 'Стандартная интерпретация — у разных производителей может отличаться',
      smd_ceramic_warning: 'Возможно, значение в мкФ — проверь корпус и даташит'
    }
  },

  inductors: {
    title: 'Калькуляторы катушек индуктивности',
    subtitle: 'Расчёт по геометрии, цветовая маркировка',
    sub: { marking: 'Маркировка', reactance: 'Xl', connections: 'Соединения', geometry: 'Геометрия' },
    labels: {
      marking: 'Маркировка',
      smdIndPlaceholder: '100, 101, 4R7, R47',
      smdIndHint: '3-4-значный или R-нотация. Значение в мкГн',
      inductance: 'Индуктивность (L)', reactanceL: 'Сопротивление (Xl)',
      frequency: 'Частота (f)',
      enterTwo: 'Введи любые две величины',
      enterOneMore: 'Введи ещё одну величину',
      enterOneInductance: 'Введи хотя бы одну индуктивность',
      calcFailed: 'Не удалось рассчитать — проверь значения',
      enterExactlyTwo: 'Введи ровно две величины для расчёта',
      ready: 'Готово',
      result: 'Результат', series: 'Последовательное', parallel: 'Параллельное',
      single: 'Однослойная', multi: 'Многослойная',
      diameter: 'Диаметр каркаса', length: 'Длина намотки',
      turns: 'Количество витков', layers: 'Количество слоёв',
      wheelerFormula: 'Формула Уилера', fillAllFields: 'Заполни все поля',
      addInductor: '+ Добавить'
    }
  },

  bms: {
    title: 'BMS — Battery Management System',
    sub: { balancing: 'Балансир', config: 'S/P сборка', shunt: 'Шунт', mosfet: 'MOSFET' },
    labels: {
      cellVoltage: 'Напряжение банки (V_CELL)', balancingCurrent: 'Ток балансировки (I_CB)',
      rdsOn: 'R_DS(on) ключа BMS', rdsOnHint: '0, если внешний MOSFET', nominalRcb: 'Номинал R_CB',
      powerAtLeast: 'Мощность: ≥', worstCurrent: 'Худший ток',
      seriesBms: 'Последовательно (S)', parallelCount: 'Параллельно (P)', cellCapacity: 'Ёмкость банки',
      cRate: 'C-rate (для тока)', cRateHint: '1 = 1C, 2 = 2C',
      voltageBms: 'Напряжение', capacity: 'Ёмкость', energy: 'Энергия', maxCurrent: 'Макс. ток',
      enterCorrectValues: 'Введи корректные значения (S, P, V, ёмкость больше 0).',
      maxCurrentLabel: 'Максимальный ток', minCurrentLabel: 'Минимальный ток', offsetVoltage: 'Напряжение смещения (Vos)',
      offsetHint: 'Типично 10-100 мкВ', recommendedNominal: 'Рекомендуемый номинал',
      powerLabel: 'Мощность', vSense: 'V_sense', minCurrentBadge: 'Мин. ток',
      notSensitive: 'Чувствительности не хватает, чтобы различить',
      notSensitiveHint: 'А. Попробуй увеличить номинал шунта или использовать усилитель с меньшим Vos.',
      enterCorrectCurrent: 'Введи корректные значения (ток больше 0).',
      maxCurrentField: 'Максимальный ток (А)', packVoltage: 'Напряжение сборки (В)',
      suitableFor: 'Подходящие (Id ≥', suitableFor2: 'А, Vds ≥', suitableFor3: 'В):',
      needPieces: 'нужно', noMosfets: 'Нет MOSFET, которые держат', noMosfets2: 'Попробуй уменьшить ток/напряжение или добавить модель в базу.',
      enterCurrentVoltage: 'Введи ток и напряжение, чтобы увидеть подходящие модели.',
      mosfetModel: 'Модель MOSFET', parallelCountNeeded: 'Параллельно штук',
      calcFailed: 'Не удалось рассчитать. Проверь значения (ток слишком большой или R_DS(on) слишком высокое).',
      visualStrongHeat: 'Сильный нагрев',
      visualCurrent: 'Ток'
    }
  },

  transistors: {
    title: 'Транзисторы',
    sub: { analog: 'Поиск аналога', param: 'Подбор по параметрам', calc: 'Калькулятор' },
    labels: {
      searchAnalog: 'Искать аналоги', searchParam: 'Найти по параметрам', model: 'Модель транзистора',
      modelPlaceholder: 'IRFZ44N или КТ315', sovietAnalogs: 'Советские / отечественные аналоги',
      digikeyAnalogs: 'Аналоги DigiKey', lcscAnalogs: 'Аналоги LCSC',
      nothingFound: 'Ничего не найдено. Попробуй другую модель.', enterModel: 'Введи модель и нажми «Искать аналоги».',
      searching: 'Поиск...', vdsMin: 'Vds (мин)', idMin: 'Id (мин)', rdsOnMax: 'RdsOn (макс)',
      vceoMin: 'Vceo (мин)', icMin: 'Ic (мин)', hfeMin: 'hFE (мин)', type: 'Тип транзистора',
      enterParams: 'Введи параметры и нажми «Найти по параметрам».', calcInDev: 'Калькулятор в разработке.', noAnalog: 'Аналогов не найдено',
      calcTitle: 'Расчёт базового резистора',
      uIn: 'Напряжение управления (Uвх)',
      uLoad: 'Напряжение нагрузки (Uнагр)',
      rLoad: 'Сопротивление нагрузки (Rнагр)',
      hfe: 'Коэффициент hFE (мин)',
      vBe: 'Напряжение база-эмиттер (Uбэ)',
      baseResistor: 'Базовый резистор (Rb)',
      baseCurrent: 'Ток базы (Ib)',
      collectorCurrent: 'Ток коллектора (Ic)',
      powerOnResistor: 'Мощность на резисторе',
      note: 'Расчёт для режима насыщения (ключа) с коэффициентом 2.5. Подходит для реле и ламп.'
    }
  },

  datasheets: {
    title: 'Поиск даташитов', subtitle: 'Найдём PDF и откроем прямо в приложении',
    digikey: 'DigiKey', lcsc: 'LCSC', openPdf: 'Открыть PDF', download: 'Скачать', back: 'Назад к поиску', searchBtn: 'Искать',
    labels: {
      searchLabel: 'Поиск даташита', searchPlaceholder: 'Например, IRFZ44N',
      enterQuery: 'Введите запрос и нажмите «Искать»', searching: 'Поиск...', nothingFound: 'Ничего не найдено',
      downloading: 'Скачивание и подготовка PDF...', downloadError: 'Не удалось скачать PDF',
      results: 'Результаты', source: 'Источник', manufacturer: 'Производитель',
      saveError: 'Ошибка сохранения: ',
      fullscreen: 'Полный экран'
    }
  },

  about: {
    title: 'Canifol', version: 'Версия',
    description: 'Canifol — бесплатный инструмент для ремонтников электроники. Калькуляторы компонентов, поиск даташитов и просмотр PDF прямо в приложении.',
    supportButton: 'Поддержать проект', license: 'Лицензия MIT', copyright: '© 2026 Canifol'
  },
  support: {
    title: 'Поддержать проект', subtitle: 'Разрабу на еду',
    copy: 'Копировать', copied: 'Скопировано', thanks: 'Спасибо за поддержку ♥'
  },

  help: {
    resistors: {
      title: 'Резисторы — справка', smdTitle: 'SMD-код', smdText: 'Введи код с корпуса резистора. Поддерживаются 3-значные (103), 4-значные (1002), EIA-96 (01C) и R-нотация (4R7).',
      colorTitle: 'Цвета', colorText: 'Выбери цвета полос по порядку — от первой к последней. Поддерживается 3, 4, 5 и 6 полос. Результат показывается сразу с допуском и ТКС (если 6 полос).',
      ohmTitle: 'Закон Ома', ohmText: 'Введи любые две величины из U, I, R, P — программа рассчитает остальные. Вычисленные поля подсвечены и помечены плашкой «вычислено».',
      connTitle: 'Соединения', connText: 'Переключайся между последовательным и параллельным соединением. Добавляй/удаляй резисторы кнопками. Итог считается сразу.',
      dividerTitle: 'Делитель напряжения', dividerText: 'Введи любые три величины из Uin, Uout, R1, R2 — программа рассчитает четвёртую. Дополнительно показывает ток через делитель и коэффициент деления.',
      pickTitle: 'Подбор', pickText: 'Введи напряжение и ток — программа подберёт сопротивление, покажет выделяемую мощность, рекомендуемый номинал по мощности и ближайшие стандартные номиналы E24.'
    },
    capacitors: {
      title: 'Конденсаторы — справка', markingTitle: 'Маркировка', markingText: 'Три режима: Код (для керамики и плёнки: 104, 4R7, 3n3), SMD (танталовые: J3, A0), Цвета (по полосам). Результат в пФ, нФ, мкФ или мФ.',
      reactanceTitle: 'Реактивное сопротивление (Xc)', reactanceText: 'Введи любые две величины из f, C, Xc — программа рассчитает третью. Xc — это «сопротивление» конденсатора переменному току.',
      connTitle: 'Соединения', connText: 'Последовательное и параллельное соединение. Обрати внимание: у конденсаторов всё наоборот по сравнению с резисторами. Последовательно — ёмкость уменьшается, параллельно — увеличивается.'
    },
    inductors: {
      title: 'Катушки — справка', markingTitle: 'Маркировка', markingText: 'Два режима: SMD (3-4 знака: 100, 4R7) и Цвета (MIL-стандарт). Значения в мкГн, нГн, мГн.',
      reactanceTitle: 'Реактивное сопротивление (Xl)', reactanceText: 'Введи любые две величины из f, L, Xl — программа рассчитает третью. Xl — «сопротивление» катушки переменному току.',
      connTitle: 'Соединения', connText: 'Последовательное — индуктивность складывается. Параллельное — уменьшается. Как у резисторов.',
      geometryTitle: 'Геометрия', geometryText: 'Расчёт индуктивности по формуле Уилера: для однослойной и многослойной намотки. Нужны диаметр, длина, количество витков. Полезно при перемотке дросселя.'
    },
    bms: {
      title: 'BMS — справка', balancingTitle: 'Балансир', balancingText: 'Расчёт резистора для пассивного балансира по формуле Texas Instruments. Введи напряжение банки, желаемый ток балансировки и R_ds(on) ключа. Визуал покажет схему.',
      configTitle: 'S/P сборка', configText: 'Собери батарею из N последовательных и M параллельных банок. Введи параметры одной банки — программа посчитает напряжение, ёмкость, энергию и максимальный ток. Визуал показывает, как паять: параллельные шины, последовательные перемычки, балансировочные провода B1...Bn, клеммы B+/B-.',
      shuntTitle: 'Шунт', shuntText: 'Расчёт шунта для измерения тока. Введи максимальный ток, минимальный ток и Vos усилителя. Программа покажет номинал, мощность, падение напряжения и предупредит, если чувствительности не хватает.',
      mosfetTitle: 'MOSFET', mosfetText: 'Подбор MOSFET для защиты BMS. Выбери модель из базы (мощные TO-220 и мелкие SOT-23), введи максимальный ток — программа скажет, сколько штук нужно параллельно с запасом 50% по току.'
    },
    transistors: {
      title: 'Транзисторы — справка', analogTitle: 'Поиск аналога', analogText: 'Введи модель транзистора, и приложение ищет аналоги сразу в трёх местах:',
      analogItem1: 'Советские аналоги — локальная база, работает офлайн.', analogItem2: 'DigiKey — современные аналоги.', analogItem3: 'LCSC — современные аналоги.',
      analogWarning: 'Важно: советские аналоги ищутся в общем поиске — отдельной вкладки для них нет. Просто введи модель (например, КТ315 или BC547) — и увидишь все три блока результатов.',
      paramTitle: 'Подбор по параметрам', paramText: 'Если модели нет под рукой, но известны параметры — используй этот режим. Для MOSFET: Vds, Id, RdsOn. Для биполярных: Vceo, Ic, hFE.',
      calcTitle: 'Калькулятор', calcText: 'Раздел с расчётами для транзисторов (базовый резистор, режимы работы и т.д.).',
      tipsTitle: 'Советы', tip1: 'Если поиск ничего не находит — проверь написание. BC547 и BC547A — разные запросы.', tip2: 'Советская база может не знать очень редких моделей, но популярные (КТ315, КТ361, ГТ402 и т.д.) там есть.'
    },
    datasheets: {
      title: 'Даташиты — справка', sourcesTitle: 'Источники', sourcesText: 'Переключайся между DigiKey и LCSC, если в одном не нашлось. У каждого свои базы, покрытие отличается.',
      cacheTitle: 'Кэш', cacheText: 'Все скачанные PDF кэшируются на диск (в папке %APPDATA%/Canifol/datasheets). При повторном открытии они загружаются мгновенно, даже без интернета.',
      viewerTitle: 'Просмотр', viewerText: 'PDF открывается внутри приложения. Работает зум, листание страниц, поиск по тексту и полноэкранный режим. Кнопка «Скачать» сохраняет файл из кэша в выбранную папку.',
      tipsTitle: 'Советы', tip1: 'Если даташит не находится — попробуй другой источник.', tip2: 'Некоторые производители (например, TI) отдают PDF с задержкой, подожди пару секунд.', tip3: 'Если ссылка ведёт на HTML вместо PDF — попробуй LCSC (у DigiKey иногда бывают редиректы).'
    }
  }
};

const en = {
  app: { name: 'Canifol', tagline: "Repairman's toolkit" },
  nav: { menu: 'Menu', close: 'Close', sections: 'Sections', settings: 'Settings' },
  sections: {
    resistors: 'Resistors', capacitors: 'Capacitors', inductors: 'Inductors',
    bms: 'BMS', transistors: 'Transistors', datasheets: 'Datasheets'
  },
  common: {
    comingSoon: 'Calculator coming soon', comingSoonHint: 'This section is under development. Switch to other tabs for now.',
    version: 'Version', language: 'Language', theme: 'Theme', themeDark: 'Dark', themeLight: 'Light', computed: 'computed', reset: 'Reset',
    pcs: 'pcs', result: 'Result', enterValues: 'Fill the fields', ready: 'Done'
  },
  buttons: {
    search: 'Search', copy: 'Copy', copied: 'Copied', reset: 'Reset', clear: 'Clear', openPdf: 'Open PDF',
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

  settings: {
    title: 'Settings',
    scaleTitle: 'Interface scale',
    scaleHint: 'Increase if you have a 4K monitor or poor eyesight.',
    cacheTitle: 'Datasheet cache',
    cacheSize: 'Cache size',
    cacheHint: 'Downloaded PDFs are stored locally and open instantly on re-request. Clearing frees space, but the next search will re-download the PDF.',
    cacheCleared: 'Cache cleared',
    cacheEmpty: 'Cache is empty',
    confirmClear: 'Are you sure you want to delete all downloaded PDFs? This cannot be undone.'
  },

  resistors: {
    title: 'Resistor calculators', subtitle: "Color codes, SMD codes, connections, Ohm's law",
    sub: { smd: 'SMD code', color: 'Colors', ohm: "Ohm's law", connections: 'Connections', divider: 'Divider', pick: 'Pick resistor' },
    labels: {
      smd: 'SMD code', smdPlaceholder: '103, 1002, 4R7, 01C', smdHint: '3, 4 digits, R-notation or EIA-96', smdError: 'Code not recognized',
      standard3: '3-digit code', standard4: '4-digit code', eia96: 'EIA-96 (1%)',
      bands: 'Number of bands', tolerance: 'Tolerance', multiplier: 'Multiplier', digit1: 'Digit 1', digit2: 'Digit 2', digit3: 'Digit 3', tcr: 'TCR',
      band1: 'Band 1', band2: 'Band 2', invalidColors: 'Invalid color combination',
      voltage: 'Voltage (U)', current: 'Current (I)', resistance: 'Resistance (R)', power: 'Power (P)',
      enterTwo: 'Enter any two values', enterOneMore: 'Enter one more value', ready: 'Done',
      calcFailed: 'Calculation failed — check values', enterExactlyTwo: 'Enter exactly two values',
      result: 'Result', series: 'Series', parallel: 'Parallel',
      addResistor: '+ Add resistor', formula: 'connection',
      uin: 'Vin', uout: 'Vout', dividerRatio: 'Divider ratio', dividerCurrent: 'Divider current',
      pickVoltage: 'Voltage', pickCurrent: 'Current', pickResistance: 'Resistance',
      recommendedPower: 'Recommended power', nearestE24: 'Nearest E24', about: 'Description'
    }
  },

  capacitors: {
    title: 'Capacitor calculators', subtitle: 'Marking, SMD codes, reactance',
    sub: { marking: 'Marking', reactance: 'Xc', connections: 'Connections' },
    labels: {
      marking: 'Marking', code: 'Code', smd: 'SMD', colors: 'Colors',
      codeLabel: 'Marking code', codePlaceholder: '104, 4R7, 3n3, 223K, 104 50V', codeHint: 'Numeric code, R-notation, p/n/u letters, tolerance, voltage',
      smdCapPlaceholder: 'J3, A0, 105', smdCapHint: 'Letter + digit (tantalum) or 3 digits (ceramic)',
      tant: 'Tantalum', ceramic: 'Ceramic', invalidColorsCap: 'Invalid color combination',
      frequency: 'Frequency (f)', capacitance: 'Capacitance (C)', reactanceC: 'Reactance (Xc)',
      enterTwo: 'Enter any two values', enterOneMore: 'Enter one more value', enterOneCapacity: 'Enter at least one capacitance',
      calcFailed: 'Calculation failed — check values', enterExactlyTwo: 'Enter exactly two values', ready: 'Done',
      result: 'Result', series: 'Series', parallel: 'Parallel', addCapacitor: '+ Add',
      bandCount: 'Number of bands', digit1: 'Digit 1', digit2: 'Digit 2', multiplier: 'Multiplier', tolerance: 'Tolerance', voltageLabel: 'Voltage',
      color_interpretation: 'Standard interpretation — may vary between manufacturers',
      smd_ceramic_warning: 'Value may be in µF — check package and datasheet'
    }
  },

  inductors: {
    title: 'Inductor calculators', subtitle: 'Geometry, color codes',
    sub: { marking: 'Marking', reactance: 'Xl', connections: 'Connections', geometry: 'Geometry' },
    labels: {
      marking: 'Marking', smdIndPlaceholder: '100, 101, 4R7, R47', smdIndHint: '3-4 digit or R-notation. Value in µH',
      inductance: 'Inductance (L)', reactanceL: 'Reactance (Xl)', frequency: 'Frequency (f)',
      enterTwo: 'Enter any two values', enterOneMore: 'Enter one more value', enterOneInductance: 'Enter at least one inductance',
      calcFailed: 'Calculation failed — check values', enterExactlyTwo: 'Enter exactly two values', ready: 'Done',
      result: 'Result', series: 'Series', parallel: 'Parallel', single: 'Single-layer', multi: 'Multi-layer',
      diameter: 'Core diameter', length: 'Winding length', turns: 'Number of turns', layers: 'Number of layers',
      wheelerFormula: "Wheeler's formula", fillAllFields: 'Fill all fields', addInductor: '+ Add'
    }
  },

  bms: {
    title: 'BMS — Battery Management System',
    sub: { balancing: 'Balancer', config: 'S/P pack', shunt: 'Shunt', mosfet: 'MOSFET' },
    labels: {
      cellVoltage: 'Cell voltage (V_CELL)', balancingCurrent: 'Balancing current (I_CB)',
      rdsOn: 'R_DS(on) of BMS switch', rdsOnHint: '0 if external MOSFET', nominalRcb: 'Nominal R_CB',
      powerAtLeast: 'Power: ≥', worstCurrent: 'Worst current',
      seriesBms: 'Series (S)', parallelCount: 'Parallel (P)', cellCapacity: 'Cell capacity',
      cRate: 'C-rate (for current)', cRateHint: '1 = 1C, 2 = 2C',
      voltageBms: 'Voltage', capacity: 'Capacity', energy: 'Energy', maxCurrent: 'Max current',
      enterCorrectValues: 'Enter valid values (S, P, V, capacity > 0).',
      maxCurrentLabel: 'Max current', minCurrentLabel: 'Min current', offsetVoltage: 'Offset voltage (Vos)',
      offsetHint: 'Typically 10-100 µV', recommendedNominal: 'Recommended nominal',
      powerLabel: 'Power', vSense: 'V_sense', minCurrentBadge: 'Min current',
      notSensitive: 'Sensitivity is not enough to distinguish', notSensitiveHint: 'A. Try increasing shunt value or using an amplifier with lower Vos.',
      enterCorrectCurrent: 'Enter valid values (current > 0).', maxCurrentField: 'Max current (A)', packVoltage: 'Pack voltage (V)',
      suitableFor: 'Suitable (Id ≥', suitableFor2: 'A, Vds ≥', suitableFor3: 'V):',
      needPieces: 'needed', noMosfets: 'No MOSFETs that hold', noMosfets2: 'Try reducing current/voltage or adding a model to the database.',
      enterCurrentVoltage: 'Enter current and voltage to see suitable models.',
      mosfetModel: 'MOSFET model', parallelCountNeeded: 'Parallel needed',
      calcFailed: 'Calculation failed. Check values (current too high or R_DS(on) too high).',
      visualStrongHeat: 'Strong heating',
      visualCurrent: 'Current'
    }
  },

  transistors: {
    title: 'Transistors', sub: { analog: 'Analog search', param: 'Parametric search', calc: 'Calculator' },
    labels: {
      searchAnalog: 'Search analogs', searchParam: 'Find by params', model: 'Transistor model', modelPlaceholder: 'IRFZ44N or KT315',
      sovietAnalogs: 'Soviet / domestic analogs', digikeyAnalogs: 'DigiKey analogs', lcscAnalogs: 'LCSC analogs',
      nothingFound: 'Nothing found. Try another model.', enterModel: 'Enter a model and press «Search analogs».', searching: 'Searching...',
      vdsMin: 'Vds (min)', idMin: 'Id (min)', rdsOnMax: 'RdsOn (max)', vceoMin: 'Vceo (min)', icMin: 'Ic (min)', hfeMin: 'hFE (min)',
      type: 'Transistor type', enterParams: 'Enter params and press «Find by params».', calcInDev: 'Calculator in development.', noAnalog: 'No analogs found',
      calcTitle: 'Base resistor calculation',
      uIn: 'Control voltage (Uin)',
      uLoad: 'Load voltage (Uload)',
      rLoad: 'Load resistance (Rload)',
      hfe: 'hFE coefficient (min)',
      vBe: 'Base-emitter voltage (Ube)',
      baseResistor: 'Base resistor (Rb)',
      baseCurrent: 'Base current (Ib)',
      collectorCurrent: 'Collector current (Ic)',
      powerOnResistor: 'Power on resistor',
      note: 'Calculation for saturation mode (switch) with factor 2.5. Suitable for relays and lamps.'
    }
  },

  datasheets: {
    title: 'Datasheet search', subtitle: 'Find PDF and open right in the app',
    digikey: 'DigiKey', lcsc: 'LCSC', openPdf: 'Open PDF', download: 'Download', back: 'Back to search', searchBtn: 'Search',
    labels: {
      searchLabel: 'Datasheet search', searchPlaceholder: 'e.g. IRFZ44N', enterQuery: 'Enter a query and press «Search»',
      searching: 'Searching...', nothingFound: 'Nothing found', downloading: 'Downloading and preparing PDF...',
      downloadError: 'Failed to download PDF', results: 'Results', source: 'Source', manufacturer: 'Manufacturer',
      saveError: 'Save error: ',
      fullscreen: 'Fullscreen'
    }
  },

  about: {
    title: 'Canifol', version: 'Version',
    description: 'Canifol is a free tool for electronics repair technicians. Component calculators, datasheet search and PDF viewer right inside the app.',
    supportButton: 'Support the project', license: 'MIT License', copyright: '© 2026 Canifol'
  },
  support: {
    title: 'Support the project', subtitle: 'Food for the developer',
    copy: 'Copy', copied: 'Copied', thanks: 'Thanks for your support ♥'
  },

  help: {
    resistors: {
      title: 'Resistors — help', smdTitle: 'SMD code', smdText: 'Enter the code from the resistor body. Supports 3-digit (103), 4-digit (1002), EIA-96 (01C) and R-notation (4R7).',
      colorTitle: 'Colors', colorText: 'Pick band colors in order — from first to last. Supports 3, 4, 5 and 6 bands. Result shows tolerance and TCR (if 6 bands).',
      ohmTitle: "Ohm's law", ohmText: 'Enter any two of U, I, R, P — the app calculates the rest. Calculated fields are highlighted with a "computed" badge.',
      connTitle: 'Connections', connText: 'Switch between series and parallel. Add/remove resistors with buttons. Result is updated instantly.',
      dividerTitle: 'Voltage divider', dividerText: 'Enter any three of Uin, Uout, R1, R2 — the app calculates the fourth. Also shows divider current and ratio.',
      pickTitle: 'Pick resistor', pickText: 'Enter voltage and current — the app picks resistance, shows dissipated power, recommended power rating and nearest E24 values.'
    },
    capacitors: {
      title: 'Capacitors — help', markingTitle: 'Marking', markingText: 'Three modes: Code (ceramic/film: 104, 4R7, 3n3), SMD (tantalum: J3, A0), Colors (bands). Result in pF, nF, µF or mF.',
      reactanceTitle: 'Reactance (Xc)', reactanceText: 'Enter any two of f, C, Xc — the app calculates the third. Xc is the capacitor\'s "resistance" to AC.',
      connTitle: 'Connections', connText: 'Series and parallel. Note: capacitors behave opposite to resistors. Series — capacitance decreases, parallel — increases.'
    },
    inductors: {
      title: 'Inductors — help', markingTitle: 'Marking', markingText: 'Two modes: SMD (3-4 digits: 100, 4R7) and Colors (MIL standard). Values in µH, nH, mH.',
      reactanceTitle: 'Reactance (Xl)', reactanceText: 'Enter any two of f, L, Xl — the app calculates the third. Xl is the inductor\'s "resistance" to AC.',
      connTitle: 'Connections', connText: 'Series — inductance adds up. Parallel — decreases. Like resistors.',
      geometryTitle: 'Geometry', geometryText: 'Inductance by Wheeler\'s formula: single-layer and multi-layer. Needs diameter, length, number of turns. Useful when rewinding a choke.'
    },
    bms: {
      title: 'BMS — help', balancingTitle: 'Balancer', balancingText: 'Balancing resistor calculation per Texas Instruments formula. Enter cell voltage, target balancing current and R_ds(on). Visual shows the circuit.',
      configTitle: 'S/P pack', configText: 'Build a pack from N series and M parallel cells. Enter one cell\'s parameters — the app calculates voltage, capacity, energy and max current. Visual shows wiring: parallel buses, series jumpers, balance wires B1...Bn, terminals B+/B-.',
      shuntTitle: 'Shunt', shuntText: 'Current-sense shunt calculation. Enter max current, min current and amplifier Vos. The app shows resistance, power, voltage drop and warns if sensitivity is not enough.',
      mosfetTitle: 'MOSFET', mosfetText: 'MOSFET selection for BMS protection. Pick a model from the database (power TO-220 and small SOT-23), enter max current — the app tells how many in parallel with 50% margin.'
    },
    transistors: {
      title: 'Transistors — help', analogTitle: 'Analog search', analogText: 'Enter a transistor model, and the app searches analogs in three places at once:',
      analogItem1: 'Soviet analogs — local database, works offline.', analogItem2: 'DigiKey — modern analogs.', analogItem3: 'LCSC — modern analogs.',
      analogWarning: 'Important: Soviet analogs are searched in the general search — there is no separate tab for them. Just enter a model (e.g. KT315 or BC547) and see all three result blocks.',
      paramTitle: 'Parametric search', paramText: 'If you don\'t know the model but know the specs — use this mode. For MOSFET: Vds, Id, RdsOn. For BJT: Vceo, Ic, hFE.',
      calcTitle: 'Calculator', calcText: 'Transistor calculations (base resistor, operating modes, etc.).',
      tipsTitle: 'Tips', tip1: 'If nothing is found — check spelling. BC547 and BC547A are different queries.', tip2: 'The Soviet database may not know very rare models, but popular ones (KT315, KT361, GT402, etc.) are there.'
    },
    datasheets: {
      title: 'Datasheets — help', sourcesTitle: 'Sources', sourcesText: 'Switch between DigiKey and LCSC if one doesn\'t find it. Each has its own database, coverage differs.',
      cacheTitle: 'Cache', cacheText: 'All downloaded PDFs are cached on disk (in %APPDATA%/Canifol/datasheets). Reopening is instant, even offline.',
      viewerTitle: 'Viewer', viewerText: 'PDF opens inside the app. Zoom, page navigation, text search and fullscreen mode work. The "Save" button saves the file from cache to a chosen folder.',
      tipsTitle: 'Tips', tip1: 'If the datasheet is not found — try another source.', tip2: 'Some manufacturers (e.g. TI) serve PDFs with a delay, wait a few seconds.', tip3: 'If the link leads to HTML instead of PDF — try LCSC (DigiKey sometimes has redirects).'
    }
  }
};

export function getStrings() {
  return localeStore.current === 'en' ? en : ru;
}