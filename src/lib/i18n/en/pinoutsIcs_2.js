import { pinoutsIcs } from './pinoutsIcs.js';

export const pinoutsIcs2 = {
  // --- Interface & Specialized ICs ---
  'max485': {
    name: 'MAX485',
    desc: 'RS-485 / RS-422 Interface (DIP-8)',
    package: 'dip8',
    pins: {
      '1': { name: 'RO', desc: 'Receiver Output', side: 'left', num: 1 },
      '2': { name: '~RE', desc: 'Receiver Output Enable (Active Low)', side: 'left', num: 2 },
      '3': { name: 'DE', desc: 'Driver Output Enable (Active High)', side: 'left', num: 3 },
      '4': { name: 'DI', desc: 'Driver Input', side: 'left', num: 4 },
      '5': { name: 'GND', desc: 'Ground', side: 'right', num: 5 },
      '6': { name: 'A', desc: 'Non-inverting Receiver Input / Driver Output', side: 'right', num: 6 },
      '7': { name: 'B', desc: 'Inverting Receiver Input / Driver Output', side: 'right', num: 7 },
      '8': { name: 'VCC', desc: 'Supply Voltage (+5V)', side: 'right', num: 8 }
    }
  },
  'mcp2515': {
    name: 'MCP2515',
    desc: 'CAN Bus Controller with SPI (SOIC-18 / DIP-18)',
    package: 'dip18',
    pins: {
      '1': { name: 'TXCAN', desc: 'Transmit Output', side: 'left', num: 1 },
      '2': { name: 'RXCAN', desc: 'Receive Input', side: 'left', num: 2 },
      '3': { name: 'CLKOUT', desc: 'Clock Output', side: 'left', num: 3 },
      '4': { name: '~TX0RTS', desc: 'TX Buffer 0 Request-to-Send', side: 'left', num: 4 },
      '5': { name: '~TX1RTS', desc: 'TX Buffer 1 Request-to-Send', side: 'left', num: 5 },
      '6': { name: '~TX2RTS', desc: 'TX Buffer 2 Request-to-Send', side: 'left', num: 6 },
      '7': { name: 'OSC2', desc: 'Oscillator Output', side: 'left', num: 7 },
      '8': { name: 'OSC1', desc: 'Oscillator Input', side: 'left', num: 8 },
      '9': { name: 'VSS', desc: 'Ground', side: 'left', num: 9 },
      '10': { name: '~RX1BF', desc: 'Receive Buffer 1 Interrupt', side: 'right', num: 10 },
      '11': { name: '~RX0BF', desc: 'Receive Buffer 0 Interrupt', side: 'right', num: 11 },
      '12': { name: '~INT', desc: 'Interrupt Output', side: 'right', num: 12 },
      '13': { name: 'SCK', desc: 'SPI Clock', side: 'right', num: 13 },
      '14': { name: 'SI', desc: 'SPI Data In', side: 'right', num: 14 },
      '15': { name: 'SO', desc: 'SPI Data Out', side: 'right', num: 15 },
      '16': { name: '~CS', desc: 'SPI Chip Select', side: 'right', num: 16 },
      '17': { name: '~RESET', desc: 'Master Reset', side: 'right', num: 17 },
      '18': { name: 'VDD', desc: 'Supply Voltage (2.7V - 5.5V)', side: 'right', num: 18 }
    }
  },
  'mcp23017': {
    name: 'MCP23017',
    desc: '16-Bit I2C I/O Expander (DIP-28)',
    package: 'dip28',
    pins: {
      '1': { name: 'GPB0', desc: 'Port B0', side: 'left', num: 1 },
      '2': { name: 'GPB1', desc: 'Port B1', side: 'left', num: 2 },
      '3': { name: 'GPB2', desc: 'Port B2', side: 'left', num: 3 },
      '4': { name: 'GPB3', desc: 'Port B3', side: 'left', num: 4 },
      '5': { name: 'GPB4', desc: 'Port B4', side: 'left', num: 5 },
      '6': { name: 'GPB5', desc: 'Port B5', side: 'left', num: 6 },
      '7': { name: 'GPB6', desc: 'Port B6', side: 'left', num: 7 },
      '8': { name: 'GPB7', desc: 'Port B7', side: 'left', num: 8 },
      '9': { name: 'VDD', desc: 'Supply Voltage', side: 'left', num: 9 },
      '10': { name: 'VSS', desc: 'Ground', side: 'left', num: 10 },
      '11': { name: 'NC', desc: 'No Connect', side: 'left', num: 11 },
      '12': { name: 'SCL', desc: 'I2C Clock', side: 'left', num: 12 },
      '13': { name: 'SDA', desc: 'I2C Data', side: 'left', num: 13 },
      '14': { name: 'NC', desc: 'No Connect', side: 'left', num: 14 },
      '15': { name: 'A0', desc: 'Address Input 0', side: 'right', num: 15 },
      '16': { name: 'A1', desc: 'Address Input 1', side: 'right', num: 16 },
      '17': { name: 'A2', desc: 'Address Input 2', side: 'right', num: 17 },
      '18': { name: '~RESET', desc: 'Hardware Reset', side: 'right', num: 18 },
      '19': { name: 'INTA', desc: 'Port A Interrupt', side: 'right', num: 19 },
      '20': { name: 'INTB', desc: 'Port B Interrupt', side: 'right', num: 20 },
      '21': { name: 'GPA0', desc: 'Port A0', side: 'right', num: 21 },
      '22': { name: 'GPA1', desc: 'Port A1', side: 'right', num: 22 },
      '23': { name: 'GPA2', desc: 'Port A2', side: 'right', num: 23 },
      '24': { name: 'GPA3', desc: 'Port A3', side: 'right', num: 24 },
      '25': { name: 'GPA4', desc: 'Port A4', side: 'right', num: 25 },
      '26': { name: 'GPA5', desc: 'Port A5', side: 'right', num: 26 },
      '27': { name: 'GPA6', desc: 'Port A6', side: 'right', num: 27 },
      '28': { name: 'GPA7', desc: 'Port A7', side: 'right', num: 28 }
    }
  },
  'pcf8574': {
    name: 'PCF8574',
    desc: '8-Bit I2C I/O Expander (DIP-16)',
    package: 'dip16',
    pins: {
      '1': { name: 'A0', desc: 'Address 0', side: 'left', num: 1 },
      '2': { name: 'A1', desc: 'Address 1', side: 'left', num: 2 },
      '3': { name: 'A2', desc: 'Address 2', side: 'left', num: 3 },
      '4': { name: 'P0', desc: 'I/O Pin 0', side: 'left', num: 4 },
      '5': { name: 'P1', desc: 'I/O Pin 1', side: 'left', num: 5 },
      '6': { name: 'P2', desc: 'I/O Pin 2', side: 'left', num: 6 },
      '7': { name: 'P3', desc: 'I/O Pin 3', side: 'left', num: 7 },
      '8': { name: 'VSS', desc: 'Ground', side: 'left', num: 8 },
      '9': { name: 'P4', desc: 'I/O Pin 4', side: 'right', num: 9 },
      '10': { name: 'P5', desc: 'I/O Pin 5', side: 'right', num: 10 },
      '11': { name: 'P6', desc: 'I/O Pin 6', side: 'right', num: 11 },
      '12': { name: 'P7', desc: 'I/O Pin 7', side: 'right', num: 12 },
      '13': { name: '~INT', desc: 'Interrupt Output', side: 'right', num: 13 },
      '14': { name: 'SCL', desc: 'I2C Clock', side: 'right', num: 14 },
      '15': { name: 'SDA', desc: 'I2C Data', side: 'right', num: 15 },
      '16': { name: 'VDD', desc: 'Supply Voltage', side: 'right', num: 16 }
    }
  },

  // --- Communication Modules ---
  'esp32-cam': {
    name: 'ESP32-CAM',
    desc: 'ESP32 Module with OV2640 Camera',
    package: 'module',
    pins: {
      '1': { name: '5V', desc: 'Power 5V', side: 'left', num: 1 },
      '2': { name: 'GND', desc: 'Ground', side: 'left', num: 2 },
      '3': { name: 'U0R', desc: 'UART0 RX', side: 'left', num: 3 },
      '4': { name: 'U0T', desc: 'UART0 TX', side: 'left', num: 4 },
      '5': { name: 'GPIO0', desc: 'Flash Mode (Connect to GND)', side: 'left', num: 5 },
      '6': { name: 'GND', desc: 'Ground', side: 'right', num: 6 },
      '7': { name: 'IO12', desc: 'GPIO 12', side: 'right', num: 7 },
      '8': { name: 'IO13', desc: 'GPIO 13', side: 'right', num: 8 },
      '9': { name: 'VCC 3V3', desc: 'Power 3.3V', side: 'right', num: 9 },
      '10': { name: 'GND', desc: 'Ground', side: 'right', num: 10 }
    }
  },
  'nrf24l01': {
    name: 'NRF24L01+',
    desc: '2.4 GHz RF Transceiver (SPI)',
    package: 'module2x4',
    pins: {
      '1': { name: 'GND', desc: 'Ground', side: 'left', num: 1 },
      '2': { name: 'VCC', desc: 'Power 3.3V', side: 'left', num: 2 },
      '3': { name: 'CE', desc: 'Chip Enable', side: 'left', num: 3 },
      '4': { name: 'CSN', desc: 'SPI Chip Select', side: 'left', num: 4 },
      '5': { name: 'SCK', desc: 'SPI Clock', side: 'right', num: 5 },
      '6': { name: 'MOSI', desc: 'SPI Data In', side: 'right', num: 6 },
      '7': { name: 'MISO', desc: 'SPI Data Out', side: 'right', num: 7 },
      '8': { name: 'IRQ', desc: 'Interrupt', side: 'right', num: 8 }
    }
  },
  'hm10': {
    name: 'HM-10 (BLE 4.0)',
    desc: 'Bluetooth 4.0 Low Energy Module',
    package: 'module',
    pins: {
      '1': { name: 'VCC', desc: 'Power 3.3V', side: 'left', num: 1 },
      '2': { name: 'GND', desc: 'Ground', side: 'left', num: 2 },
      '3': { name: 'TXD', desc: 'UART TX', side: 'right', num: 3 },
      '4': { name: 'RXD', desc: 'UART RX', side: 'right', num: 4 }
    }
  },
  'rc522': {
    name: 'RFID-RC522',
    desc: '13.56 MHz RFID Reader Module',
    package: 'module',
    pins: {
      '1': { name: 'VCC', desc: 'Power 3.3V', side: 'left', num: 1 },
      '2': { name: 'RST', desc: 'Reset', side: 'left', num: 2 },
      '3': { name: 'GND', desc: 'Ground', side: 'left', num: 3 },
      '4': { name: 'IRQ', desc: 'Interrupt', side: 'left', num: 4 },
      '5': { name: 'MISO', desc: 'SPI MISO', side: 'right', num: 5 },
      '6': { name: 'MOSI', desc: 'SPI MOSI', side: 'right', num: 6 },
      '7': { name: 'SCK', desc: 'SPI SCK', side: 'right', num: 7 },
      '8': { name: 'SS/SDA', desc: 'SPI Chip Select', side: 'right', num: 8 }
    }
  }
};