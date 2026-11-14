import { pinoutsIcs } from './pinoutsIcs.js';

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
  }
};