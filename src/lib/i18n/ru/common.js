export const common = {
  app: { name: 'Canifol', tagline: 'Инструмент ремонтника' },
  nav: { menu: 'Меню', close: 'Закрыть', sections: 'Разделы', settings: 'Настройки' },
  sections: {
    favorites: 'Избранное',
    resistors: 'Резисторы', capacitors: 'Конденсаторы', inductors: 'Катушки',
    bms: 'BMS', transistors: 'Транзисторы', datasheets: 'Даташиты',
    pinouts: 'Распиновки',
    smd: 'SMD-коды',
    boardview: 'BoardView'
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
  boardview: {
    openFile: 'Открыть свой файл',
    noFile: 'Выберите файл .fz или .cad для просмотра',
    loading: 'Загрузка...',
    error: 'Ошибка загрузки',
    pinsCount: 'Пинов',
    zoom: 'Масштаб',
    showPins: 'Пины',
    showLines: 'Дорожки',
    showComponents: 'Компоненты',
    showRoutes: 'Трассировка',
    helpTitle: 'Справка по BoardView',
    helpFormatsTitle: 'Поддерживаемые форматы',
    helpFormats: [
      { name: 'FZ / CAE', desc: 'ASUS / ASRock. Пины и компоненты.' },
      { name: 'CAD (GenCAD)', desc: 'Пины, компоненты, дорожки, пады.' },
      { name: 'BDV / ASC', desc: 'eM-Test Expert. Пины и компоненты (родные контуры).' }
    ],
    helpFeaturesTitle: 'Возможности',
    helpFeatures: [
      'Клик по пину — подсветка всей цепи (нет-листа).',
      'Поиск по компонентам (например, Q21).',
      'Слои: пины, дорожки, компоненты, трассировка.',
      'Масштабирование и панорамирование.',
      'Полноэкранный режим.'
    ]
  }
};