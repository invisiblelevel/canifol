export const help = {
  "help": {
    "resistors": {
      "title": "电阻 — 帮助",
      "smdTitle": "SMD 代码",
      "smdText": "输入电阻本体上的代码。支持 3 位（103）、4 位（1002）、EIA-96（01C）和 R 记法（4R7）。",
      "colorTitle": "色环",
      "colorText": "按顺序选择色环颜色 — 从第一个到最后一个。支持 3、4、5、6 色环。结果会立即显示容差和温度系数（若为 6 色环）。",
      "ohmTitle": "欧姆定律",
      "ohmText": "输入 U、I、R、P 中的任意两个值 — 程序会自动计算其余值。计算出的字段会高亮并标记为「已计算」。",
      "connTitle": "连接",
      "connText": "在串联和并联之间切换。使用按钮添加/移除电阻。结果会立即计算。",
      "dividerTitle": "分压器",
      "dividerText": "输入 Uin、Uout、R1、R2 中的任意三个值 — 程序会自动计算第四个值。此外还会显示分压器电流和分压比。",
      "pickTitle": "选型",
      "pickText": "输入电压和电流 — 程序会选择合适的电阻，显示耗散功率、推荐的额定功率以及最接近的 E24 标准值。"
    },
    "capacitors": {
      "title": "电容 — 帮助",
      "markingTitle": "标记",
      "markingText": "三种模式：代码（陶瓷和薄膜电容：104、4R7、3n3）、SMD（钽电容：J3、A0）、色环。结果以 pF、nF、µF 或 mF 为单位。",
      "reactanceTitle": "容抗 (Xc)",
      "reactanceText": "输入 f、C、Xc 中的任意两个值 — 程序会自动计算第三个值。Xc 是电容对交流电的「阻力」。",
      "connTitle": "连接",
      "connText": "串联和并联。注意：电容的行为与电阻相反。串联时容量减小，并联时容量增加。",
      "esrTitle": "ESR（等效串联电阻）",
      "esrText": "通过 ESR 检查电解电容。选择容量、电压和类型（通用 / 低 ESR / 音频 / 105 °C）。程序会显示正常的 ESR 范围和「更换阈值」。输入万用表测量值即可获得判定：正常、老化或干涸。对于低 ESR 电容（开关电源用）要求更严格 — ESR 值要低数倍。数据为平均值，精确检测请参考具体系列的数据表。"
    },
    "inductors": {
      "title": "电感 — 帮助",
      "markingTitle": "标记",
      "markingText": "两种模式：SMD（3-4 位数字：100、4R7）和色环（MIL 标准）。单位为 µH、nH、mH。",
      "reactanceTitle": "感抗 (Xl)",
      "reactanceText": "输入 f、L、Xl 中的任意两个值 — 程序会自动计算第三个值。Xl 是电感对交流电的「阻力」。",
      "connTitle": "连接",
      "connText": "串联时电感相加，并联时减小。与电阻类似。",
      "geometryTitle": "几何",
      "geometryText": "使用惠勒公式计算电感：单层和多层绕组。需要线圈直径、绕线长度和匝数。在重绕扼流圈时很有用。"
    },
    "bms": {
      "title": "BMS — 帮助",
      "balancingTitle": "均衡器",
      "balancingText": "根据 Texas Instruments 公式计算被动均衡电阻。输入电芯电压、目标均衡电流和开关的 R_ds(on)。可视化图会显示电路。",
      "configTitle": "串/并组装",
      "configText": "使用 N 个串联和 M 个并联电芯组成电池组。输入单个电芯的参数 — 程序会计算电压、容量、能量和最大电流。可视化图会显示如何焊接：并联母线、串联跳线、均衡线 B1...Bn、端子 B+/B-。",
      "shuntTitle": "分流器",
      "shuntText": "用于电流测量的分流器计算。输入最大电流、最小电流和放大器的 Vos。程序会显示标称值、功率、电压降，并在灵敏度不足时发出警告。",
      "mosfetTitle": "MOSFET",
      "mosfetText": "为 BMS 保护选择 MOSFET。从数据库中选择型号（大功率 TO-220 和小型 SOT-23），输入最大电流 — 程序会告诉你需要并联多少个（50% 电流裕量）。"
    },
    "transistors": {
      "title": "晶体管 — 帮助",
      "analogTitle": "寻找替代型号",
      "analogText": "输入晶体管型号，应用会同时在三个地方搜索替代型号：",
      "analogItem1": "苏联替代型号 — 本地数据库，可离线使用。",
      "analogItem2": "DigiKey — 现代替代型号。",
      "analogItem3": "LCSC — 现代替代型号。",
      "analogWarning": "重要提示：苏联替代型号在通用搜索中查找 — 没有单独的选项卡。只需输入型号（例如 KT315 或 BC547）即可看到三个结果块。",
      "paramTitle": "参数选型",
      "paramText": "如果不方便查看型号但知道参数 — 请使用此模式。对于 MOSFET：Vds、Id、RdsOn。对于双极型：Vceo、Ic、hFE。",
      "calcTitle": "计算器",
      "calcText": "晶体管相关计算（基极电阻、工作模式等）。",
      "tipsTitle": "提示",
      "tip1": "如果搜索不到 — 请检查拼写。BC547 和 BC547A 是不同的查询。",
      "tip2": "苏联数据库可能不包含非常罕见的型号，但常见的（KT315、KT361、GT402 等）都有。"
    },
    "datasheets": {
      "title": "数据表 — 帮助",
      "sourcesTitle": "来源",
      "sourcesText": "如果其中一个找不到，请在 DigiKey 和 LCSC 之间切换。每个来源都有自己的数据库，覆盖范围不同。",
      "cacheTitle": "缓存",
      "cacheText": "所有下载的 PDF 都会缓存到磁盘上（在 %APPDATA%/Canifol/datasheets 文件夹中）。再次打开时会即时加载，即使没有网络。",
      "viewerTitle": "查看",
      "viewerText": "PDF 在应用内打开。支持缩放、翻页、文本搜索和全屏模式。「下载」按钮可将缓存中的文件保存到指定的文件夹。",
      "tipsTitle": "提示",
      "tip1": "如果找不到数据表 — 请尝试另一个来源。",
      "tip2": "部分制造商（如 TI）会延迟提供 PDF，请稍等几秒钟。",
      "tip3": "如果链接指向 HTML 而不是 PDF — 请尝试 LCSC（DigiKey 有时会有重定向）。"
    }
  }
};