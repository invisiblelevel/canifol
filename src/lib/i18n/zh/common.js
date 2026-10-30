export const common = {
  app: { name: 'Canifol', tagline: '维修工具' },
  nav: { menu: '菜单', close: '关闭', sections: '部分', settings: '设置' },
  sections: {
    favorites: '收藏',
    resistors: '电阻器', capacitors: '电容器', inductors: '电感器',
    bms: 'BMS', transistors: '晶体管', datasheets: '数据表',
    pinouts: '引脚图', smd: 'SMD 代码', boardview: '主板视图'
  },
  common: {
    comingSoon: '计算器即将推出',
    comingSoonHint: '此部分正在开发中。请切换到其他选项卡。',
    version: '版本', language: '语言', theme: '主题',
    themeDark: '深色', themeLight: '浅色',
    computed: '已计算', reset: '重置',
    pcs: '个', result: '结果', enterValues: '填写字段', ready: '就绪'
  },
  buttons: {
    search: '搜索', copy: '复制', copied: '已复制',
    reset: '重置', clear: '清除', openPdf: '打开 PDF',
    download: '下载', back: '返回', add: '添加', remove: '删除',
    clearCache: '清除缓存', confirm: '确认', cancel: '取消'
  },
  colors: {
    black: '黑色', brown: '棕色', red: '红色', orange: '橙色',
    yellow: '黄色', green: '绿色', blue: '蓝色', violet: '紫色',
    gray: '灰色', white: '白色', gold: '金色', silver: '银色'
  },
  transistorTypes: { mosfet: 'MOSFET', bjt: '双极型' },
  markingTabs: { code: '代码', smd: 'SMD', color: '颜色' },
  units: {
    hz: '赫兹', khz: '千赫', mhz: '兆赫',
    pf: '皮法', nf: '纳法', uf: '微法', mf: '毫法', f: '法拉',
    ohm: '欧姆', kohm: '千欧', mohm: '兆欧', mohm_short: '毫欧',
    uh: '微亨', nh: '纳亨', mh: '毫亨', h: '亨利',
    v: '伏特', a: '安培', w: '瓦特', mm: '毫米',
    ah: '安时', wh: '瓦时', ma: '毫安', uv: '微伏', mv: '毫伏',
    b: '字节', kb: 'KB', mb: 'MB', gb: 'GB'
  },
  boardview: {
    openFile: '打开本地文件',
    noFile: '选择一个 .fz 或 .cad 文件进行查看',
    loading: '加载中...',
    error: '加载错误',
    pinsCount: '引脚',
    zoom: '缩放',
    showPins: '引脚',
    showLines: '走线',
    showComponents: '元件',
    showRoutes: '布线',
    helpTitle: 'BoardView 帮助',
    helpFormatsTitle: '支持的格式',
    helpFormats: [
      { name: 'FZ / CAE', desc: 'ASUS / ASRock。引脚和元件。' },
      { name: 'CAD (GenCAD)', desc: '引脚、元件、走线、焊盘。' },
      { name: 'BDV / ASC', desc: 'eM-Test Expert。引脚和元件（原生轮廓）。' }
    ],
    helpFeaturesTitle: '功能',
    helpFeatures: [
      '点击引脚以高亮整个网络。',
      '按元件搜索（例如，Q21）。',
      '图层：引脚、走线、元件、布线。',
      '缩放和平移。',
      '全屏模式。'
    ]
  }
};