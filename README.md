# `max7219`

> A platform agnostic driver to interface with the MAX7219 (LED display driver)

[![Build Status](https://travis-ci.org/maikelwever/max7219.svg?branch=master)](https://travis-ci.org/maikelwever/max7219)

## What works

- Powering on/off the MAX chip
- Basic commands for setting LEDs on/off.
- Chaining support (max 8 devices)

## TODO

- [ ] Using hardware SPI

## Example

Here is a simple example for using the MAX7219 on a stm32f103xx device with stm32f103xx_hal:
```rust
#![deny(unsafe_code)]
#![no_std]

extern crate stm32f103xx_hal as hal;
extern crate max7219;

use hal::stm32f103xx;
use hal::prelude::*;

use max7219::{MAX7219, Command, DecodeMode};


fn main() {
    let dp = stm32f103xx::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);

    let _clocks = rcc.cfgr
        .sysclk(64.mhz())
        .pclk1(32.mhz())
        .freeze(&mut flash.acr);

    let max_data = gpioa.pa7.into_push_pull_output(&mut gpioa.crl);
    let max_clk = gpioa.pa5.into_push_pull_output(&mut gpioa.crl);
    let max_cs = gpioa.pa4.into_push_pull_output(&mut gpioa.crl);

    let number_of_devices: u8 = 1;
    let mut max7219 = MAX7219::new(number_of_devices, max_data, max_cs, max_clk);
    max7219.power_on();
    max7219.set_intensity(0, 8);

    // For a 7-segment display, optionally set a decode mode:
    // max7219.set_decode_mode(DecodeMode::CodeBDigits7_0);
    // You can add a dot to any number with an OR operation:
    // max7219.write_raw(0x01, 0x01 | 0x80);
    // Numbers 0-9 are written using their raw values. 0x0F is empty

    max7219.write_raw(0, 0x01, 1);
    max7219.write_raw(0, 0x02, 0x02);
    max7219.write_raw(0, 0x03, 3);
    max7219.write_data(0, Command::Digit4, 0x04);
    max7219.write_raw(0, 0x05, 0x05);
    max7219.write_raw(0, 0x06, 0x06 | 0x80);
    max7219.write_raw(0, 0x07, 0x05);
    max7219.write_raw(0, 0x08, 0x8F);
}
```

## License

Licensed under MIT license ([LICENSE](LICENSE) or http://opensource.org/licenses/MIT)

