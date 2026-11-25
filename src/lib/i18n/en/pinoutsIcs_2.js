import { pinoutsIcs } from './pinoutsIcs.js';

// Every 25-series SPI BIOS flash chip uses this 8-pin package pinout.
const spiFlashPins = {
  '1': { name: '~CS', desc: 'Chip select' },
  '2': { name: 'DO / IO1', desc: 'Data out' },
  '3': { name: '~WP / IO2', desc: 'Write protect' },
  '4': { name: 'GND', desc: 'Ground' },
  '5': { name: 'DI / IO0', desc: 'Data in' },
  '6': { name: 'CLK', desc: 'Clock' },
  '7': { name: '~HOLD / ~RESET', desc: 'Hold or reset' },
  '8': { name: 'VCC', desc: 'Power 3.3 V' }
};

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
  },
  // --- Additional import ICs ---
  'max7219': {
    name: 'MAX7219', desc: '8-digit LED display driver (DIP-24)', package: 'dip24',
    pins: {
      '1': { name: 'DIN', desc: 'Serial data in' }, '2': { name: 'DIG0', desc: 'Digit 0' },
      '3': { name: 'DIG4', desc: 'Digit 4' }, '4': { name: 'GND', desc: 'Ground' },
      '5': { name: 'DIG6', desc: 'Digit 6' }, '6': { name: 'DIG2', desc: 'Digit 2' },
      '7': { name: 'DIG3', desc: 'Digit 3' }, '8': { name: 'DIG7', desc: 'Digit 7' },
      '9': { name: 'DIG5', desc: 'Digit 5' }, '10': { name: 'DIG1', desc: 'Digit 1' },
      '11': { name: 'LOAD/~CS', desc: 'Load / chip select' }, '12': { name: 'CLK', desc: 'Clock' },
      '13': { name: 'NC', desc: 'Not connected' }, '14': { name: 'DOUT', desc: 'Serial data out' },
      '15': { name: 'SEG D', desc: 'Segment D' }, '16': { name: 'SEG DP', desc: 'Decimal point' },
      '17': { name: 'SEG E', desc: 'Segment E' }, '18': { name: 'SEG C', desc: 'Segment C' },
      '19': { name: 'V+', desc: 'Power +5 V' }, '20': { name: 'SEG B', desc: 'Segment B' },
      '21': { name: 'SEG F', desc: 'Segment F' }, '22': { name: 'SEG A', desc: 'Segment A' },
      '23': { name: 'SEG G', desc: 'Segment G' }, '24': { name: 'NC', desc: 'Not connected' }
    }
  },
  'ds1307': {
    name: 'DS1307', desc: 'I2C real-time clock (DIP-8)', package: 'dip8',
    pins: {
      '1': { name: 'X1', desc: '32.768 kHz crystal' }, '2': { name: 'X2', desc: '32.768 kHz crystal' },
      '3': { name: 'VBAT', desc: 'Battery backup' }, '4': { name: 'GND', desc: 'Ground' },
      '5': { name: 'SDA', desc: 'I2C data' }, '6': { name: 'SCL', desc: 'I2C clock' },
      '7': { name: 'SQW/OUT', desc: 'Square wave output' }, '8': { name: 'VCC', desc: 'Power +5 V' }
    }
  },
  'pcf8591': {
    name: 'PCF8591', desc: '8-bit I2C ADC/DAC (DIP-16)', package: 'dip16',
    pins: {
      '1': { name: 'AIN0', desc: 'Analog input 0' }, '2': { name: 'AIN1', desc: 'Analog input 1' },
      '3': { name: 'AIN2', desc: 'Analog input 2' }, '4': { name: 'AIN3', desc: 'Analog input 3' },
      '5': { name: 'A0', desc: 'I2C address 0' }, '6': { name: 'A1', desc: 'I2C address 1' },
      '7': { name: 'A2', desc: 'I2C address 2' }, '8': { name: 'VSS', desc: 'Ground' },
      '9': { name: 'SDA', desc: 'I2C data' }, '10': { name: 'SCL', desc: 'I2C clock' },
      '11': { name: 'OSC', desc: 'Oscillator' }, '12': { name: 'EXT', desc: 'External clock' },
      '13': { name: 'AGND', desc: 'Analog ground' }, '14': { name: 'VREF', desc: 'Reference voltage' },
      '15': { name: 'AOUT', desc: 'Analog output' }, '16': { name: 'VDD', desc: 'Power' }
    }
  },
  'max3232': {
    name: 'MAX3232', desc: '3.3 V RS-232 transceiver (DIP-16)', package: 'dip16',
    pins: {
      '1': { name: 'C1+', desc: 'Charge pump capacitor 1+' }, '2': { name: 'V+', desc: 'Positive voltage' },
      '3': { name: 'C1-', desc: 'Charge pump capacitor 1-' }, '4': { name: 'C2+', desc: 'Charge pump capacitor 2+' },
      '5': { name: 'C2-', desc: 'Charge pump capacitor 2-' }, '6': { name: 'V-', desc: 'Negative voltage' },
      '7': { name: 'T2OUT', desc: 'Transmitter 2 output' }, '8': { name: 'R2IN', desc: 'Receiver 2 input' },
      '9': { name: 'R2OUT', desc: 'Receiver 2 output' }, '10': { name: 'T2IN', desc: 'Transmitter 2 input' },
      '11': { name: 'T1IN', desc: 'Transmitter 1 input' }, '12': { name: 'R1OUT', desc: 'Receiver 1 output' },
      '13': { name: 'R1IN', desc: 'Receiver 1 input' }, '14': { name: 'T1OUT', desc: 'Transmitter 1 output' },
      '15': { name: 'GND', desc: 'Ground' }, '16': { name: 'VCC', desc: 'Power 3.3 V' }
    }
  },
  '74hc165': {
    name: '74HC165', desc: '8-bit parallel-in/serial-out shift register (DIP-16)', package: 'dip16',
    pins: {
      '1': { name: '~PL', desc: 'Parallel load' }, '2': { name: 'CP', desc: 'Clock' },
      '3': { name: 'D4', desc: 'Parallel input D4' }, '4': { name: 'D5', desc: 'Parallel input D5' },
      '5': { name: 'D6', desc: 'Parallel input D6' }, '6': { name: 'D7', desc: 'Parallel input D7' },
      '7': { name: '~Q7', desc: 'Inverted serial output' }, '8': { name: 'GND', desc: 'Ground' },
      '9': { name: 'Q7', desc: 'Serial output' }, '10': { name: 'DS', desc: 'Serial input' },
      '11': { name: 'D0', desc: 'Parallel input D0' }, '12': { name: 'D1', desc: 'Parallel input D1' },
      '13': { name: 'D2', desc: 'Parallel input D2' }, '14': { name: 'D3', desc: 'Parallel input D3' },
      '15': { name: '~CE', desc: 'Clock inhibit' }, '16': { name: 'VCC', desc: 'Power' }
    }
  },
  'acs712': {
    name: 'ACS712', desc: 'Hall-effect current sensor (SOIC-8)', package: 'sop8',
    pins: {
      '1': { name: 'IP+', desc: 'Current input +' }, '2': { name: 'IP+', desc: 'Current input +' },
      '3': { name: 'GND', desc: 'Ground' }, '4': { name: 'NC', desc: 'Not connected' },
      '5': { name: 'VCC', desc: 'Power +5 V' }, '6': { name: 'FILTER', desc: 'Filter capacitor' },
      '7': { name: 'VOUT', desc: 'Analog output' }, '8': { name: 'IP-', desc: 'Current input -' }
    }
  },
  'max6675': {
    name: 'MAX6675', desc: 'K-type thermocouple controller (SOIC-8)', package: 'sop8',
    pins: {
      '1': { name: 'GND', desc: 'Ground' }, '2': { name: 'T-', desc: 'Thermocouple -' },
      '3': { name: 'T+', desc: 'Thermocouple +' }, '4': { name: 'VCC', desc: 'Power 3-5 V' },
      '5': { name: 'SCK', desc: 'SPI clock' }, '6': { name: '~CS', desc: 'SPI chip select' },
      '7': { name: 'SO', desc: 'SPI data out' }, '8': { name: 'NC', desc: 'Not connected' }
    }
  },
  'tp4056': {
    name: 'TP4056', desc: '1 A Li-ion battery charger (SOP-8)', package: 'sop8',
    pins: {
      '1': { name: 'TEMP', desc: 'Battery temperature' }, '2': { name: 'PROG', desc: 'Charge current set' },
      '3': { name: 'GND', desc: 'Ground' }, '4': { name: 'VCC', desc: 'Power input' },
      '5': { name: 'BAT', desc: 'Battery' }, '6': { name: 'STDBY', desc: 'Standby indicator' },
      '7': { name: 'CHRG', desc: 'Charging indicator' }, '8': { name: 'CE', desc: 'Chip enable' }
    }
  },
  'ncp1117': {
    name: 'NCP1117', desc: 'Low-dropout linear regulator (SOT-223)', package: 'sot223',
    pins: {
      '1': { name: 'ADJ/GND', desc: 'Adjust / ground' }, '2': { name: 'OUT', desc: 'Regulated output' },
      '3': { name: 'IN', desc: 'Input voltage' }
    }
  },
  'lm2576': {
    name: 'LM2576', desc: '3 A step-down regulator (TO-220-5)', package: 'to220_5',
    pins: {
      '1': { name: 'VIN', desc: 'Input voltage' }, '2': { name: 'OUT', desc: 'Switching output' },
      '3': { name: 'GND', desc: 'Ground' }, '4': { name: 'FB', desc: 'Feedback' },
      '5': { name: 'ON/~OFF', desc: 'Enable' }
    }
  },
  // --- BIOS / firmware SPI flash (SOIC-8) ---
  'w25q32jv': { name: 'W25Q32JV', desc: '32 Mbit SPI BIOS flash (SOIC-8)', package: 'sop8', pins: spiFlashPins },
  'w25q64jv': { name: 'W25Q64JV', desc: '64 Mbit SPI BIOS flash (SOIC-8)', package: 'sop8', pins: spiFlashPins },
  'w25q128jv': { name: 'W25Q128JV', desc: '128 Mbit SPI BIOS flash (SOIC-8)', package: 'sop8', pins: spiFlashPins },
  'w25q256jv': { name: 'W25Q256JV', desc: '256 Mbit SPI BIOS flash (SOIC-8/WSON-8)', package: 'sop8', pins: spiFlashPins },
  'mx25l6406e': { name: 'MX25L6406E', desc: '64 Mbit SPI BIOS flash (SOIC-8)', package: 'sop8', pins: spiFlashPins },
  'mx25l12835f': { name: 'MX25L12835F', desc: '128 Mbit SPI BIOS flash (SOIC-8)', package: 'sop8', pins: spiFlashPins },
  'gd25q64c': { name: 'GD25Q64C', desc: '64 Mbit SPI BIOS flash (SOIC-8)', package: 'sop8', pins: spiFlashPins },
  'gd25q128c': { name: 'GD25Q128C', desc: '128 Mbit SPI BIOS flash (SOIC-8)', package: 'sop8', pins: spiFlashPins },
  'sst25vf016b': { name: 'SST25VF016B', desc: '16 Mbit SPI BIOS flash (SOIC-8)', package: 'sop8', pins: spiFlashPins },
  'en25q128': { name: 'EN25Q128', desc: '128 Mbit SPI BIOS flash (SOIC-8)', package: 'sop8', pins: spiFlashPins }
};