# `max7219`

> A platform agnostic driver to interface with the MAX7219 (LED display driver)

## What works

- Powering on/off the MAX chip
- Basic commands for setting LEDs on/off.

## TODO

- [ ] Using hardware SPI
- [ ] Implementing chaining support

## Example

Here is a simple example for using the MAX7219 on a stm32f103xx device with stm32f103xx_hal:
```rust
#![deny(unsafe_code)]
#![no_std]

extern crate cortex_m;
extern crate stm32f103xx_hal as hal;
extern crate max7219;

use hal::stm32f103xx;
use hal::prelude::*;

use max7219::{MAX7219, Command};


fn main() {
    let dp = stm32f103xx::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);
    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);

    let _clocks = rcc.cfgr
        .sysclk(64.mhz())
        .pclk1(32.mhz())
        .freeze(&mut flash.acr);

    let max_data = gpioa.pa7.into_push_pull_output(&mut gpioa.crl);
    let max_clk = gpioa.pa5.into_push_pull_output(&mut gpioa.crl);
    let max_cs = gpioa.pa4.into_push_pull_output(&mut gpioa.crl);

    let mut max7219 = MAX7219::new(max_data, max_cs, max_clk);
    max7219.power_on();
    max7219.set_intensity(8);

    max7219.write_raw(0x01, 0x01);
    max7219.write_raw(0x02, 0x02);
    max7219.write_raw(0x03, 0x03);
    max7219.write_raw(0x04, 0x04);
    max7219.write_raw(0x05, 0x05);
    max7219.write_raw(0x06, 0x06);
    max7219.write_raw(0x07, 0x07);
    max7219.write_raw(0x08, 0x08);
}
```

## License

Licensed under MIT license ([LICENSE](LICENSE) or http://opensource.org/licenses/MIT)

