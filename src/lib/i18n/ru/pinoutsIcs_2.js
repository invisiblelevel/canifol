import { pinoutsIcs } from './pinoutsIcs.js';

export const pinoutsIcs2 = {
  // --- Интерфейсные и специализированные микросхемы ---
  'max485': {
    name: 'MAX485',
    desc: 'Интерфейс RS-485 / RS-422 (DIP-8)',
    package: 'dip8',
    pins: {
      '1': { name: 'RO', desc: 'Приемник выход (Receiver Output)', side: 'left', num: 1 },
      '2': { name: '~RE', desc: 'Разрешение приема (активный низкий)', side: 'left', num: 2 },
      '3': { name: 'DE', desc: 'Разрешение передачи (активный высокий)', side: 'left', num: 3 },
      '4': { name: 'DI', desc: 'Передатчик вход (Driver Input)', side: 'left', num: 4 },
      '5': { name: 'GND', desc: 'Земля', side: 'right', num: 5 },
      '6': { name: 'A', desc: 'Линия связи A (неинвертированная)', side: 'right', num: 6 },
      '7': { name: 'B', desc: 'Линия связи B (инвертированная)', side: 'right', num: 7 },
      '8': { name: 'VCC', desc: 'Питание (+5V)', side: 'right', num: 8 }
    }
  },
  'mcp2515': {
    name: 'MCP2515',
    desc: 'Контроллер CAN-шины с SPI (SOIC-18 / DIP-18)',
    package: 'dip18',
    pins: {
      '1': { name: 'TXCAN', desc: 'Выход передатчика CAN', side: 'left', num: 1 },
      '2': { name: 'RXCAN', desc: 'Вход приемника CAN', side: 'left', num: 2 },
      '3': { name: 'CLKOUT', desc: 'Выход тактового генератора', side: 'left', num: 3 },
      '4': { name: '~TX0RTS', desc: 'Запрос передачи буфера 0', side: 'left', num: 4 },
      '5': { name: '~TX1RTS', desc: 'Запрос передачи буфера 1', side: 'left', num: 5 },
      '6': { name: '~TX2RTS', desc: 'Запрос передачи буфера 2', side: 'left', num: 6 },
      '7': { name: 'OSC2', desc: 'Кварц / Внешний тактовый', side: 'left', num: 7 },
      '8': { name: 'OSC1', desc: 'Кварц', side: 'left', num: 8 },
      '9': { name: 'VSS', desc: 'Земля', side: 'left', num: 9 },
      '10': { name: '~RX1BF', desc: 'Предупреждение буфера 1', side: 'right', num: 10 },
      '11': { name: '~RX0BF', desc: 'Предупреждение буфера 0', side: 'right', num: 11 },
      '12': { name: '~INT', desc: 'Выход прерывания', side: 'right', num: 12 },
      '13': { name: 'SCK', desc: 'SPI Тактирование', side: 'right', num: 13 },
      '14': { name: 'SI', desc: 'SPI Данные In', side: 'right', num: 14 },
      '15': { name: 'SO', desc: 'SPI Данные Out', side: 'right', num: 15 },
      '16': { name: '~CS', desc: 'SPI Выбор чипа', side: 'right', num: 16 },
      '17': { name: '~RESET', desc: 'Сброс', side: 'right', num: 17 },
      '18': { name: 'VDD', desc: 'Питание (2.7V - 5.5V)', side: 'right', num: 18 }
    }
  },
  'mcp23017': {
    name: 'MCP23017',
    desc: '16-битный расширитель портов I2C (DIP-28)',
    package: 'dip28',
    pins: {
      '1': { name: 'GPB0', desc: 'Порт B0', side: 'left', num: 1 },
      '2': { name: 'GPB1', desc: 'Порт B1', side: 'left', num: 2 },
      '3': { name: 'GPB2', desc: 'Порт B2', side: 'left', num: 3 },
      '4': { name: 'GPB3', desc: 'Порт B3', side: 'left', num: 4 },
      '5': { name: 'GPB4', desc: 'Порт B4', side: 'left', num: 5 },
      '6': { name: 'GPB5', desc: 'Порт B5', side: 'left', num: 6 },
      '7': { name: 'GPB6', desc: 'Порт B6', side: 'left', num: 7 },
      '8': { name: 'GPB7', desc: 'Порт B7', side: 'left', num: 8 },
      '9': { name: 'VDD', desc: 'Питание', side: 'left', num: 9 },
      '10': { name: 'VSS', desc: 'Земля', side: 'left', num: 10 },
      '11': { name: 'NC', desc: 'Не подключен', side: 'left', num: 11 },
      '12': { name: 'SCL', desc: 'I2C Тактирование', side: 'left', num: 12 },
      '13': { name: 'SDA', desc: 'I2C Данные', side: 'left', num: 13 },
      '14': { name: 'NC', desc: 'Не подключен', side: 'left', num: 14 },
      '15': { name: 'A0', desc: 'Адресный вход 0', side: 'right', num: 15 },
      '16': { name: 'A1', desc: 'Адресный вход 1', side: 'right', num: 16 },
      '17': { name: 'A2', desc: 'Адресный вход 2', side: 'right', num: 17 },
      '18': { name: '~RESET', desc: 'Сброс', side: 'right', num: 18 },
      '19': { name: 'INTA', desc: 'Прерывание порт A', side: 'right', num: 19 },
      '20': { name: 'INTB', desc: 'Прерывание порт B', side: 'right', num: 20 },
      '21': { name: 'GPA0', desc: 'Порт A0', side: 'right', num: 21 },
      '22': { name: 'GPA1', desc: 'Порт A1', side: 'right', num: 22 },
      '23': { name: 'GPA2', desc: 'Порт A2', side: 'right', num: 23 },
      '24': { name: 'GPA3', desc: 'Порт A3', side: 'right', num: 24 },
      '25': { name: 'GPA4', desc: 'Порт A4', side: 'right', num: 25 },
      '26': { name: 'GPA5', desc: 'Порт A5', side: 'right', num: 26 },
      '27': { name: 'GPA6', desc: 'Порт A6', side: 'right', num: 27 },
      '28': { name: 'GPA7', desc: 'Порт A7', side: 'right', num: 28 }
    }
  },
  'pcf8574': {
    name: 'PCF8574',
    desc: '8-битный расширитель портов I2C (DIP-16)',
    package: 'dip16',
    pins: {
      '1': { name: 'A0', desc: 'Адрес 0', side: 'left', num: 1 },
      '2': { name: 'A1', desc: 'Адрес 1', side: 'left', num: 2 },
      '3': { name: 'A2', desc: 'Адрес 2', side: 'left', num: 3 },
      '4': { name: 'P0', desc: 'Линия ввода-вывода 0', side: 'left', num: 4 },
      '5': { name: 'P1', desc: 'Линия ввода-вывода 1', side: 'left', num: 5 },
      '6': { name: 'P2', desc: 'Линия ввода-вывода 2', side: 'left', num: 6 },
      '7': { name: 'P3', desc: 'Линия ввода-вывода 3', side: 'left', num: 7 },
      '8': { name: 'VSS', desc: 'Земля', side: 'left', num: 8 },
      '9': { name: 'P4', desc: 'Линия ввода-вывода 4', side: 'right', num: 9 },
      '10': { name: 'P5', desc: 'Линия ввода-вывода 5', side: 'right', num: 10 },
      '11': { name: 'P6', desc: 'Линия ввода-вывода 6', side: 'right', num: 11 },
      '12': { name: 'P7', desc: 'Линия ввода-вывода 7', side: 'right', num: 12 },
      '13': { name: '~INT', desc: 'Выход прерывания', side: 'right', num: 13 },
      '14': { name: 'SCL', desc: 'I2C Тактирование', side: 'right', num: 14 },
      '15': { name: 'SDA', desc: 'I2C Данные', side: 'right', num: 15 },
      '16': { name: 'VDD', desc: 'Питание', side: 'right', num: 16 }
    }
  },

  // --- Модули связи (Wi-Fi / Bluetooth / RF) ---
  'esp32-cam': {
    name: 'ESP32-CAM',
    desc: 'Модуль ESP32 с камерой OV2640',
    package: 'module',
    pins: {
      '1': { name: '5V', desc: 'Питание 5В', side: 'left', num: 1 },
      '2': { name: 'GND', desc: 'Земля', side: 'left', num: 2 },
      '3': { name: 'U0R', desc: 'UART0 RX', side: 'left', num: 3 },
      '4': { name: 'U0T', desc: 'UART0 TX', side: 'left', num: 4 },
      '5': { name: 'GPIO0', desc: 'Прошивка (замкнуть на GND)', side: 'left', num: 5 },
      '6': { name: 'GND', desc: 'Земля', side: 'right', num: 6 },
      '7': { name: 'IO12', desc: 'GPIO 12', side: 'right', num: 7 },
      '8': { name: 'IO13', desc: 'GPIO 13', side: 'right', num: 8 },
      '9': { name: 'VCC 3V3', desc: 'Питание 3.3В', side: 'right', num: 9 },
      '10': { name: 'GND', desc: 'Земля', side: 'right', num: 10 }
    }
  },
  'nrf24l01': {
    name: 'NRF24L01+',
    desc: 'Радиомодуль 2.4 ГГц (SPI)',
    package: 'module2x4',
    pins: {
      '1': { name: 'GND', desc: 'Земля', side: 'left', num: 1 },
      '2': { name: 'VCC', desc: 'Питание 3.3V', side: 'left', num: 2 },
      '3': { name: 'CE', desc: 'Chip Enable', side: 'left', num: 3 },
      '4': { name: 'CSN', desc: 'SPI Chip Select', side: 'left', num: 4 },
      '5': { name: 'SCK', desc: 'SPI Тактирование', side: 'right', num: 5 },
      '6': { name: 'MOSI', desc: 'SPI Данные In', side: 'right', num: 6 },
      '7': { name: 'MISO', desc: 'SPI Данные Out', side: 'right', num: 7 },
      '8': { name: 'IRQ', desc: 'Прерывание', side: 'right', num: 8 }
    }
  },
  'hm10': {
    name: 'HM-10 (BLE 4.0)',
    desc: 'Bluetooth 4.0 Low Energy модуль',
    package: 'module',
    pins: {
      '1': { name: 'VCC', desc: 'Питание 3.3V', side: 'left', num: 1 },
      '2': { name: 'GND', desc: 'Земля', side: 'left', num: 2 },
      '3': { name: 'TXD', desc: 'UART TX', side: 'right', num: 3 },
      '4': { name: 'RXD', desc: 'UART RX', side: 'right', num: 4 }
    }
  },
  'rc522': {
    name: 'RFID-RC522',
    desc: 'Считыватель RFID карт 13.56 МГц',
    package: 'module',
    pins: {
      '1': { name: 'VCC', desc: 'Питание 3.3V', side: 'left', num: 1 },
      '2': { name: 'RST', desc: 'Сброс', side: 'left', num: 2 },
      '3': { name: 'GND', desc: 'Земля', side: 'left', num: 3 },
      '4': { name: 'IRQ', desc: 'Прерывание', side: 'left', num: 4 },
      '5': { name: 'MISO', desc: 'SPI MISO', side: 'right', num: 5 },
      '6': { name: 'MOSI', desc: 'SPI MOSI', side: 'right', num: 6 },
      '7': { name: 'SCK', desc: 'SPI SCK', side: 'right', num: 7 },
      '8': { name: 'SS/SDA', desc: 'SPI Chip Select', side: 'right', num: 8 }
    }
  },

  // --- Отладочные платы ---
  'arduino-mega': {
    name: 'Arduino Mega 2560',
    desc: 'Мощная плата на ATmega2560',
    package: 'board',
    pins: {
      '1': { name: 'GND', desc: 'Земля', side: 'left', num: 1 },
      '2': { name: '5V', desc: 'Питание +5V', side: 'left', num: 2 },
      '3': { name: '3V3', desc: 'Питание +3.3V', side: 'left', num: 3 },
      '4': { name: 'RESET', desc: 'Сброс', side: 'left', num: 4 },
      '5': { name: 'TX1 / D18', desc: 'UART 1 TX', side: 'left', num: 5 },
      '6': { name: 'RX1 / D19', desc: 'UART 1 RX', side: 'left', num: 6 },
      '7': { name: 'TX2 / D16', desc: 'UART 2 TX', side: 'left', num: 7 },
      '8': { name: 'RX2 / D17', desc: 'UART 2 RX', side: 'left', num: 8 },
      '9': { name: 'TX3 / D14', desc: 'UART 3 TX', side: 'right', num: 9 },
      '10': { name: 'RX3 / D15', desc: 'UART 3 RX', side: 'right', num: 10 },
      '11': { name: 'SDA / D20', desc: 'I2C Данные', side: 'right', num: 11 },
      '12': { name: 'SCL / D21', desc: 'I2C Тактирование', side: 'right', num: 12 },
      '13': { name: 'SDA1', desc: 'Второй I2C SDA', side: 'right', num: 13 },
      '14': { name: 'SCL1', desc: 'Второй I2C SCL', side: 'right', num: 14 }
    }
  },
  'raspberry-pi-pico': {
    name: 'Raspberry Pi Pico',
    desc: 'Микроконтроллер RP2040 (40 пинов)',
    package: 'board40',
    pins: {
      '1': { name: 'GP0', desc: 'GPIO 0', side: 'left', num: 1 },
      '2': { name: 'GP1', desc: 'GPIO 1', side: 'left', num: 2 },
      '3': { name: 'GND', desc: 'Земля', side: 'left', num: 3 },
      '4': { name: 'GP2', desc: 'GPIO 2', side: 'left', num: 4 },
      '5': { name: 'GP3', desc: 'GPIO 3', side: 'left', num: 5 },
      '6': { name: 'GP4', desc: 'GPIO 4', side: 'left', num: 6 },
      '7': { name: 'GP5', desc: 'GPIO 5', side: 'left', num: 7 },
      '8': { name: 'GND', desc: 'Земля', side: 'left', num: 8 },
      '9': { name: 'GP6', desc: 'GPIO 6', side: 'left', num: 9 },
      '10': { name: 'GP7', desc: 'GPIO 7', side: 'left', num: 10 },
      '11': { name: 'GP8', desc: 'GPIO 8', side: 'left', num: 11 },
      '12': { name: 'GP9', desc: 'GPIO 9', side: 'left', num: 12 },
      '13': { name: 'GND', desc: 'Земля', side: 'left', num: 13 },
      '14': { name: 'GP10', desc: 'GPIO 10', side: 'left', num: 14 },
      '15': { name: 'GP11', desc: 'GPIO 11', side: 'left', num: 15 },
      '16': { name: 'GP12', desc: 'GPIO 12', side: 'left', num: 16 },
      '17': { name: 'GP13', desc: 'GPIO 13', side: 'left', num: 17 },
      '18': { name: 'GND', desc: 'Земля', side: 'left', num: 18 },
      '19': { name: 'GP14', desc: 'GPIO 14', side: 'left', num: 19 },
      '20': { name: 'GP15', desc: 'GPIO 15', side: 'left', num: 20 },
      '21': { name: 'GP16', desc: 'GPIO 16', side: 'right', num: 21 },
      '22': { name: 'GP17', desc: 'GPIO 17', side: 'right', num: 22 },
      '23': { name: 'GND', desc: 'Земля', side: 'right', num: 23 },
      '24': { name: 'GP18', desc: 'GPIO 18', side: 'right', num: 24 },
      '25': { name: 'GP19', desc: 'GPIO 19', side: 'right', num: 25 },
      '26': { name: 'GP20', desc: 'GPIO 20', side: 'right', num: 26 },
      '27': { name: 'GP21', desc: 'GPIO 21', side: 'right', num: 27 },
      '28': { name: 'GND', desc: 'Земля', side: 'right', num: 28 },
      '29': { name: 'GP22', desc: 'GPIO 22', side: 'right', num: 29 },
      '30': { name: 'RUN', desc: 'Сброс', side: 'right', num: 30 },
      '31': { name: 'GP26', desc: 'GPIO 26 / ADC0', side: 'right', num: 31 },
      '32': { name: 'GP27', desc: 'GPIO 27 / ADC1', side: 'right', num: 32 },
      '33': { name: 'AGND', desc: 'Аналоговая земля', side: 'right', num: 33 },
      '34': { name: 'GP28', desc: 'GPIO 28 / ADC2', side: 'right', num: 34 },
      '35': { name: 'VREF', desc: 'Опорное АЦП', side: 'right', num: 35 },
      '36': { name: '3V3', desc: 'Выход 3.3В', side: 'right', num: 36 },
      '37': { name: '3V3_EN', desc: 'Включение 3.3В', side: 'right', num: 37 },
      '38': { name: 'GND', desc: 'Земля', side: 'right', num: 38 },
      '39': { name: 'VSYS', desc: 'Вход питания', side: 'right', num: 39 },
      '40': { name: 'VBUS', desc: 'Питание USB 5В', side: 'right', num: 40 }
    }
  },

  // --- Советские микросхемы ---
  'k176la7': {
    name: 'К176ЛА7',
    desc: '4 элемента 2И-НЕ (КМОП, DIP-14)',
    package: 'dip14',
    pins: {
      '1': { name: '1A', desc: 'Вход 1', side: 'left', num: 1 },
      '2': { name: '1B', desc: 'Вход 2', side: 'left', num: 2 },
      '3': { name: '1Y', desc: 'Выход 1', side: 'left', num: 3 },
      '4': { name: '2A', desc: 'Вход 3', side: 'left', num: 4 },
      '5': { name: '2B', desc: 'Вход 4', side: 'left', num: 5 },
      '6': { name: '2Y', desc: 'Выход 2', side: 'left', num: 6 },
      '7': { name: 'VSS', desc: 'Земля', side: 'left', num: 7 },
      '8': { name: '3Y', desc: 'Выход 3', side: 'right', num: 8 },
      '9': { name: '3A', desc: 'Вход 5', side: 'right', num: 9 },
      '10': { name: '3B', desc: 'Вход 6', side: 'right', num: 10 },
      '11': { name: '4Y', desc: 'Выход 4', side: 'right', num: 11 },
      '12': { name: '4A', desc: 'Вход 7', side: 'right', num: 12 },
      '13': { name: '4B', desc: 'Вход 8', side: 'right', num: 13 },
      '14': { name: 'VDD', desc: 'Питание (+9В)', side: 'right', num: 14 }
    }
  },
  'k134id1': {
    name: 'К134ИД1',
    desc: 'Дешифратор для газоразрядных индикаторов (ИН-14)',
    package: 'dip16',
    pins: {
      '1': { name: 'D0', desc: 'Вход кода 1', side: 'left', num: 1 },
      '2': { name: 'D1', desc: 'Вход кода 2', side: 'left', num: 2 },
      '3': { name: 'D2', desc: 'Вход кода 4', side: 'left', num: 3 },
      '4': { name: 'D3', desc: 'Вход кода 8', side: 'left', num: 4 },
      '5': { name: 'OUT 0', desc: 'Катод 0', side: 'left', num: 5 },
      '6': { name: 'OUT 1', desc: 'Катод 1', side: 'left', num: 6 },
      '7': { name: 'OUT 2', desc: 'Катод 2', side: 'left', num: 7 },
      '8': { name: 'GND', desc: 'Земля', side: 'left', num: 8 },
      '9': { name: 'OUT 3', desc: 'Катод 3', side: 'right', num: 9 },
      '10': { name: 'OUT 4', desc: 'Катод 4', side: 'right', num: 10 },
      '11': { name: 'OUT 5', desc: 'Катод 5', side: 'right', num: 11 },
      '12': { name: 'OUT 6', desc: 'Катод 6', side: 'right', num: 12 },
      '13': { name: 'OUT 7', desc: 'Катод 7', side: 'right', num: 13 },
      '14': { name: 'OUT 8', desc: 'Катод 8', side: 'right', num: 14 },
      '15': { name: 'OUT 9', desc: 'Катод 9', side: 'right', num: 15 },
      '16': { name: 'VCC', desc: 'Питание (+5В)', side: 'right', num: 16 }
    }
  }
};