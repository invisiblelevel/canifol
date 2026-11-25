import { pinoutsIcs } from './pinoutsIcs.js';

// 所有 25 系列 SPI BIOS 闪存都使用这种 8 引脚封装。
const spiFlashPins = {
  '1': { name: '~CS', desc: '片选' },
  '2': { name: 'DO / IO1', desc: '数据输出' },
  '3': { name: '~WP / IO2', desc: '写保护' },
  '4': { name: 'GND', desc: '接地' },
  '5': { name: 'DI / IO0', desc: '数据输入' },
  '6': { name: 'CLK', desc: '时钟' },
  '7': { name: '~HOLD / ~RESET', desc: '保持 / 复位' },
  '8': { name: 'VCC', desc: '3.3V 电源' }
};

export const pinoutsIcs2 = {
  // --- 接口与专用集成电路 ---
  'max485': {
    name: 'MAX485',
    desc: 'RS-485 / RS-422 接口芯片 (DIP-8)',
    package: 'dip8',
    pins: {
      '1': { name: 'RO', desc: '接收器输出', side: 'left', num: 1 },
      '2': { name: '~RE', desc: '接收器使能 (低电平有效)', side: 'left', num: 2 },
      '3': { name: 'DE', desc: '驱动器使能 (高电平有效)', side: 'left', num: 3 },
      '4': { name: 'DI', desc: '驱动器输入', side: 'left', num: 4 },
      '5': { name: 'GND', desc: '接地', side: 'right', num: 5 },
      '6': { name: 'A', desc: '同相 A 线', side: 'right', num: 6 },
      '7': { name: 'B', desc: '反相 B 线', side: 'right', num: 7 },
      '8': { name: 'VCC', desc: '电源 (+5V)', side: 'right', num: 8 }
    }
  },
  'mcp2515': {
    name: 'MCP2515',
    desc: 'SPI 接口 CAN 总线控制器 (DIP-18)',
    package: 'dip18',
    pins: {
      '1': { name: 'TXCAN', desc: 'CAN 发送输出', side: 'left', num: 1 },
      '2': { name: 'RXCAN', desc: 'CAN 接收输入', side: 'left', num: 2 },
      '3': { name: 'CLKOUT', desc: '时钟输出', side: 'left', num: 3 },
      '4': { name: '~TX0RTS', desc: '发送缓冲区 0 请求', side: 'left', num: 4 },
      '5': { name: '~TX1RTS', desc: '发送缓冲区 1 请求', side: 'left', num: 5 },
      '6': { name: '~TX2RTS', desc: '发送缓冲区 2 请求', side: 'left', num: 6 },
      '7': { name: 'OSC2', desc: '晶振输出', side: 'left', num: 7 },
      '8': { name: 'OSC1', desc: '晶振输入', side: 'left', num: 8 },
      '9': { name: 'VSS', desc: '接地', side: 'left', num: 9 },
      '10': { name: '~RX1BF', desc: '接收缓冲区 1 中断', side: 'right', num: 10 },
      '11': { name: '~RX0BF', desc: '接收缓冲区 0 中断', side: 'right', num: 11 },
      '12': { name: '~INT', desc: '中断输出', side: 'right', num: 12 },
      '13': { name: 'SCK', desc: 'SPI 时钟', side: 'right', num: 13 },
      '14': { name: 'SI', desc: 'SPI 数据输入', side: 'right', num: 14 },
      '15': { name: 'SO', desc: 'SPI 数据输出', side: 'right', num: 15 },
      '16': { name: '~CS', desc: 'SPI 片选', side: 'right', num: 16 },
      '17': { name: '~RESET', desc: '复位', side: 'right', num: 17 },
      '18': { name: 'VDD', desc: '电源 (2.7V - 5.5V)', side: 'right', num: 18 }
    }
  },
  'mcp23017': {
    name: 'MCP23017',
    desc: '16位 I2C 扩展 I/O 芯片 (DIP-28)',
    package: 'dip28',
    pins: {
      '1': { name: 'GPB0', desc: '端口 B0', side: 'left', num: 1 },
      '2': { name: 'GPB1', desc: '端口 B1', side: 'left', num: 2 },
      '3': { name: 'GPB2', desc: '端口 B2', side: 'left', num: 3 },
      '4': { name: 'GPB3', desc: '端口 B3', side: 'left', num: 4 },
      '5': { name: 'GPB4', desc: '端口 B4', side: 'left', num: 5 },
      '6': { name: 'GPB5', desc: '端口 B5', side: 'left', num: 6 },
      '7': { name: 'GPB6', desc: '端口 B6', side: 'left', num: 7 },
      '8': { name: 'GPB7', desc: '端口 B7', side: 'left', num: 8 },
      '9': { name: 'VDD', desc: '电源', side: 'left', num: 9 },
      '10': { name: 'VSS', desc: '接地', side: 'left', num: 10 },
      '11': { name: 'NC', desc: '悬空', side: 'left', num: 11 },
      '12': { name: 'SCL', desc: 'I2C 时钟', side: 'left', num: 12 },
      '13': { name: 'SDA', desc: 'I2C 数据', side: 'left', num: 13 },
      '14': { name: 'NC', desc: '悬空', side: 'left', num: 14 },
      '15': { name: 'A0', desc: '地址输入 0', side: 'right', num: 15 },
      '16': { name: 'A1', desc: '地址输入 1', side: 'right', num: 16 },
      '17': { name: 'A2', desc: '地址输入 2', side: 'right', num: 17 },
      '18': { name: '~RESET', desc: '复位', side: 'right', num: 18 },
      '19': { name: 'INTA', desc: 'A口中断', side: 'right', num: 19 },
      '20': { name: 'INTB', desc: 'B口中断', side: 'right', num: 20 },
      '21': { name: 'GPA0', desc: '端口 A0', side: 'right', num: 21 },
      '22': { name: 'GPA1', desc: '端口 A1', side: 'right', num: 22 },
      '23': { name: 'GPA2', desc: '端口 A2', side: 'right', num: 23 },
      '24': { name: 'GPA3', desc: '端口 A3', side: 'right', num: 24 },
      '25': { name: 'GPA4', desc: '端口 A4', side: 'right', num: 25 },
      '26': { name: 'GPA5', desc: '端口 A5', side: 'right', num: 26 },
      '27': { name: 'GPA6', desc: '端口 A6', side: 'right', num: 27 },
      '28': { name: 'GPA7', desc: '端口 A7', side: 'right', num: 28 }
    }
  },
  'pcf8574': {
    name: 'PCF8574',
    desc: '8位 I2C 扩展 I/O 芯片 (DIP-16)',
    package: 'dip16',
    pins: {
      '1': { name: 'A0', desc: '地址 0', side: 'left', num: 1 },
      '2': { name: 'A1', desc: '地址 1', side: 'left', num: 2 },
      '3': { name: 'A2', desc: '地址 2', side: 'left', num: 3 },
      '4': { name: 'P0', desc: 'I/O 引脚 0', side: 'left', num: 4 },
      '5': { name: 'P1', desc: 'I/O 引脚 1', side: 'left', num: 5 },
      '6': { name: 'P2', desc: 'I/O 引脚 2', side: 'left', num: 6 },
      '7': { name: 'P3', desc: 'I/O 引脚 3', side: 'left', num: 7 },
      '8': { name: 'VSS', desc: '接地', side: 'left', num: 8 },
      '9': { name: 'P4', desc: 'I/O 引脚 4', side: 'right', num: 9 },
      '10': { name: 'P5', desc: 'I/O 引脚 5', side: 'right', num: 10 },
      '11': { name: 'P6', desc: 'I/O 引脚 6', side: 'right', num: 11 },
      '12': { name: 'P7', desc: 'I/O 引脚 7', side: 'right', num: 12 },
      '13': { name: '~INT', desc: '中断输出', side: 'right', num: 13 },
      '14': { name: 'SCL', desc: 'I2C 时钟', side: 'right', num: 14 },
      '15': { name: 'SDA', desc: 'I2C 数据', side: 'right', num: 15 },
      '16': { name: 'VDD', desc: '电源', side: 'right', num: 16 }
    }
  },

  // --- 通信模块 ---
  'esp32-cam': {
    name: 'ESP32-CAM',
    desc: '带 OV2640 摄像头的 ESP32 模块',
    package: 'module',
    pins: {
      '1': { name: '5V', desc: '5V 电源', side: 'left', num: 1 },
      '2': { name: 'GND', desc: '接地', side: 'left', num: 2 },
      '3': { name: 'U0R', desc: 'UART0 RX', side: 'left', num: 3 },
      '4': { name: 'U0T', desc: 'UART0 TX', side: 'left', num: 4 },
      '5': { name: 'GPIO0', desc: '烧录模式 (拉低)', side: 'left', num: 5 },
      '6': { name: 'GND', desc: '接地', side: 'right', num: 6 },
      '7': { name: 'IO12', desc: 'GPIO 12', side: 'right', num: 7 },
      '8': { name: 'IO13', desc: 'GPIO 13', side: 'right', num: 8 },
      '9': { name: 'VCC 3V3', desc: '3.3V 电源', side: 'right', num: 9 },
      '10': { name: 'GND', desc: '接地', side: 'right', num: 10 }
    }
  },
  'nrf24l01': {
    name: 'NRF24L01+',
    desc: '2.4 GHz 无线收发模块 (SPI)',
    package: 'module2x4',
    pins: {
      '1': { name: 'GND', desc: '接地', side: 'left', num: 1 },
      '2': { name: 'VCC', desc: '3.3V 电源', side: 'left', num: 2 },
      '3': { name: 'CE', desc: '1芯片使能', side: 'left', num: 3 },
      '4': { name: 'CSN', desc: 'SPI 片选', side: 'left', num: 4 },
      '5': { name: 'SCK', desc: 'SPI 时钟', side: 'right', num: 5 },
      '6': { name: 'MOSI', desc: 'SPI 数据输入', side: 'right', num: 6 },
      '7': { name: 'MISO', desc: 'SPI 数据输出', side: 'right', num: 7 },
      '8': { name: 'IRQ', desc: '中断', side: 'right', num: 8 }
    }
  },
  'hm10': {
    name: 'HM-10 (BLE 4.0)',
    desc: '蓝牙 4.0 低功耗模块',
    package: 'module',
    pins: {
      '1': { name: 'VCC', desc: '3.3V 电源', side: 'left', num: 1 },
      '2': { name: 'GND', desc: '接地', side: 'left', num: 2 },
      '3': { name: 'TXD', desc: 'UART TX', side: 'right', num: 3 },
      '4': { name: 'RXD', desc: 'UART RX', side: 'right', num: 4 }
    }
  },
  'rc522': {
    name: 'RFID-RC522',
    desc: '13.56 MHz 射频识别模块',
    package: 'module',
    pins: {
      '1': { name: 'VCC', desc: '3.3V 电源', side: 'left', num: 1 },
      '2': { name: 'RST', desc: '复位', side: 'left', num: 2 },
      '3': { name: 'GND', desc: '接地', side: 'left', num: 3 },
      '4': { name: 'IRQ', desc: '中断', side: 'left', num: 4 },
      '5': { name: 'MISO', desc: 'SPI MISO', side: 'right', num: 5 },
      '6': { name: 'MOSI', desc: 'SPI MOSI', side: 'right', num: 6 },
      '7': { name: 'SCK', desc: 'SPI SCK', side: 'right', num: 7 },
      '8': { name: 'SS/SDA', desc: 'SPI 片选', side: 'right', num: 8 }
    }
  },
  // --- 其他进口芯片 ---
  'max7219': {
    name: 'MAX7219', desc: '8位数码管驱动 (DIP-24)', package: 'dip24',
    pins: {
      '1': { name: 'DIN', desc: '串行数据输入' }, '2': { name: 'DIG0', desc: '位选 0' },
      '3': { name: 'DIG4', desc: '位选 4' }, '4': { name: 'GND', desc: '接地' },
      '5': { name: 'DIG6', desc: '位选 6' }, '6': { name: 'DIG2', desc: '位选 2' },
      '7': { name: 'DIG3', desc: '位选 3' }, '8': { name: 'DIG7', desc: '位选 7' },
      '9': { name: 'DIG5', desc: '位选 5' }, '10': { name: 'DIG1', desc: '位选 1' },
      '11': { name: 'LOAD/~CS', desc: '锁存 / 片选' }, '12': { name: 'CLK', desc: '时钟' },
      '13': { name: 'NC', desc: '未连接' }, '14': { name: 'DOUT', desc: '串行数据输出' },
      '15': { name: 'SEG D', desc: '段 D' }, '16': { name: 'SEG DP', desc: '小数点' },
      '17': { name: 'SEG E', desc: '段 E' }, '18': { name: 'SEG C', desc: '段 C' },
      '19': { name: 'V+', desc: '+5V 电源' }, '20': { name: 'SEG B', desc: '段 B' },
      '21': { name: 'SEG F', desc: '段 F' }, '22': { name: 'SEG A', desc: '段 A' },
      '23': { name: 'SEG G', desc: '段 G' }, '24': { name: 'NC', desc: '未连接' }
    }
  },
  'ds1307': {
    name: 'DS1307', desc: 'I2C 实时时钟 (DIP-8)', package: 'dip8',
    pins: {
      '1': { name: 'X1', desc: '32.768 kHz 晶振' }, '2': { name: 'X2', desc: '32.768 kHz 晶振' },
      '3': { name: 'VBAT', desc: '电池备份' }, '4': { name: 'GND', desc: '接地' },
      '5': { name: 'SDA', desc: 'I2C 数据' }, '6': { name: 'SCL', desc: 'I2C 时钟' },
      '7': { name: 'SQW/OUT', desc: '方波输出' }, '8': { name: 'VCC', desc: '+5V 电源' }
    }
  },
  'pcf8591': {
    name: 'PCF8591', desc: '8位 I2C ADC/DAC (DIP-16)', package: 'dip16',
    pins: {
      '1': { name: 'AIN0', desc: '模拟输入 0' }, '2': { name: 'AIN1', desc: '模拟输入 1' },
      '3': { name: 'AIN2', desc: '模拟输入 2' }, '4': { name: 'AIN3', desc: '模拟输入 3' },
      '5': { name: 'A0', desc: 'I2C 地址 0' }, '6': { name: 'A1', desc: 'I2C 地址 1' },
      '7': { name: 'A2', desc: 'I2C 地址 2' }, '8': { name: 'VSS', desc: '接地' },
      '9': { name: 'SDA', desc: 'I2C 数据' }, '10': { name: 'SCL', desc: 'I2C 时钟' },
      '11': { name: 'OSC', desc: '振荡器' }, '12': { name: 'EXT', desc: '外部时钟' },
      '13': { name: 'AGND', desc: '模拟地' }, '14': { name: 'VREF', desc: '基准电压' },
      '15': { name: 'AOUT', desc: '模拟输出' }, '16': { name: 'VDD', desc: '电源' }
    }
  },
  'max3232': {
    name: 'MAX3232', desc: '3.3V RS-232 收发器 (DIP-16)', package: 'dip16',
    pins: {
      '1': { name: 'C1+', desc: '电荷泵电容 1+' }, '2': { name: 'V+', desc: '正电压' },
      '3': { name: 'C1-', desc: '电荷泵电容 1-' }, '4': { name: 'C2+', desc: '电荷泵电容 2+' },
      '5': { name: 'C2-', desc: '电荷泵电容 2-' }, '6': { name: 'V-', desc: '负电压' },
      '7': { name: 'T2OUT', desc: '发送器 2 输出' }, '8': { name: 'R2IN', desc: '接收器 2 输入' },
      '9': { name: 'R2OUT', desc: '接收器 2 输出' }, '10': { name: 'T2IN', desc: '发送器 2 输入' },
      '11': { name: 'T1IN', desc: '发送器 1 输入' }, '12': { name: 'R1OUT', desc: '接收器 1 输出' },
      '13': { name: 'R1IN', desc: '接收器 1 输入' }, '14': { name: 'T1OUT', desc: '发送器 1 输出' },
      '15': { name: 'GND', desc: '接地' }, '16': { name: 'VCC', desc: '3.3V 电源' }
    }
  },
  '74hc165': {
    name: '74HC165', desc: '8位并转串移位寄存器 (DIP-16)', package: 'dip16',
    pins: {
      '1': { name: '~PL', desc: '并行加载' }, '2': { name: 'CP', desc: '时钟' },
      '3': { name: 'D4', desc: '并行输入 D4' }, '4': { name: 'D5', desc: '并行输入 D5' },
      '5': { name: 'D6', desc: '并行输入 D6' }, '6': { name: 'D7', desc: '并行输入 D7' },
      '7': { name: '~Q7', desc: '反相串行输出' }, '8': { name: 'GND', desc: '接地' },
      '9': { name: 'Q7', desc: '串行输出' }, '10': { name: 'DS', desc: '串行输入' },
      '11': { name: 'D0', desc: '并行输入 D0' }, '12': { name: 'D1', desc: '并行输入 D1' },
      '13': { name: 'D2', desc: '并行输入 D2' }, '14': { name: 'D3', desc: '并行输入 D3' },
      '15': { name: '~CE', desc: '时钟禁止' }, '16': { name: 'VCC', desc: '电源' }
    }
  },
  'acs712': {
    name: 'ACS712', desc: '霍尔电流传感器 (SOIC-8)', package: 'sop8',
    pins: {
      '1': { name: 'IP+', desc: '电流输入 +' }, '2': { name: 'IP+', desc: '电流输入 +' },
      '3': { name: 'GND', desc: '接地' }, '4': { name: 'NC', desc: '未连接' },
      '5': { name: 'VCC', desc: '+5V 电源' }, '6': { name: 'FILTER', desc: '滤波' },
      '7': { name: 'VOUT', desc: '模拟输出' }, '8': { name: 'IP-', desc: '电流输入 -' }
    }
  },
  'max6675': {
    name: 'MAX6675', desc: 'K型热电偶控制器 (SOIC-8)', package: 'sop8',
    pins: {
      '1': { name: 'GND', desc: '接地' }, '2': { name: 'T-', desc: '热电偶 -' },
      '3': { name: 'T+', desc: '热电偶 +' }, '4': { name: 'VCC', desc: '3-5V 电源' },
      '5': { name: 'SCK', desc: 'SPI 时钟' }, '6': { name: '~CS', desc: 'SPI 片选' },
      '7': { name: 'SO', desc: 'SPI 数据输出' }, '8': { name: 'NC', desc: '未连接' }
    }
  },
  'tp4056': {
    name: 'TP4056', desc: '1A 锂电池充电管理 (SOP-8)', package: 'sop8',
    pins: {
      '1': { name: 'TEMP', desc: '电池温度' }, '2': { name: 'PROG', desc: '充电电流设置' },
      '3': { name: 'GND', desc: '接地' }, '4': { name: 'VCC', desc: '电源输入' },
      '5': { name: 'BAT', desc: '电池' }, '6': { name: 'STDBY', desc: '待机指示' },
      '7': { name: 'CHRG', desc: '充电指示' }, '8': { name: 'CE', desc: '使能' }
    }
  },
  'ncp1117': {
    name: 'NCP1117', desc: '低压差线性稳压器 (SOT-223)', package: 'sot223',
    pins: {
      '1': { name: 'ADJ/GND', desc: '调整 / 接地' }, '2': { name: 'OUT', desc: '输出' },
      '3': { name: 'IN', desc: '输入' }
    }
  },
  'lm2576': {
    name: 'LM2576', desc: '3A 降压稳压器 (TO-220-5)', package: 'to220_5',
    pins: {
      '1': { name: 'VIN', desc: '输入电压' }, '2': { name: 'OUT', desc: '开关输出' },
      '3': { name: 'GND', desc: '接地' }, '4': { name: 'FB', desc: '反馈' },
      '5': { name: 'ON/~OFF', desc: '使能' }
    }
  },
  // --- BIOS / 固件 SPI 闪存 (SOIC-8) ---
  'w25q32jv': { name: 'W25Q32JV', desc: 'SPI BIOS 闪存 32 Mbit (SOIC-8)', package: 'sop8', pins: spiFlashPins },
  'w25q64jv': { name: 'W25Q64JV', desc: 'SPI BIOS 闪存 64 Mbit (SOIC-8)', package: 'sop8', pins: spiFlashPins },
  'w25q128jv': { name: 'W25Q128JV', desc: 'SPI BIOS 闪存 128 Mbit (SOIC-8)', package: 'sop8', pins: spiFlashPins },
  'w25q256jv': { name: 'W25Q256JV', desc: 'SPI BIOS 闪存 256 Mbit (SOIC-8/WSON-8)', package: 'sop8', pins: spiFlashPins },
  'mx25l6406e': { name: 'MX25L6406E', desc: 'SPI BIOS 闪存 64 Mbit (SOIC-8)', package: 'sop8', pins: spiFlashPins },
  'mx25l12835f': { name: 'MX25L12835F', desc: 'SPI BIOS 闪存 128 Mbit (SOIC-8)', package: 'sop8', pins: spiFlashPins },
  'gd25q64c': { name: 'GD25Q64C', desc: 'SPI BIOS 闪存 64 Mbit (SOIC-8)', package: 'sop8', pins: spiFlashPins },
  'gd25q128c': { name: 'GD25Q128C', desc: 'SPI BIOS 闪存 128 Mbit (SOIC-8)', package: 'sop8', pins: spiFlashPins },
  'sst25vf016b': { name: 'SST25VF016B', desc: 'SPI BIOS 闪存 16 Mbit (SOIC-8)', package: 'sop8', pins: spiFlashPins },
  'en25q128': { name: 'EN25Q128', desc: 'SPI BIOS 闪存 128 Mbit (SOIC-8)', package: 'sop8', pins: spiFlashPins }
};