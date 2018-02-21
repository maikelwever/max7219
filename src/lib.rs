//! A platform agnostic driver to interface with the MAX7219 (LED matrix display driver)
//!
//! This driver was built using [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://docs.rs/embedded-hal/~0.1


#![deny(unsafe_code)]
#![deny(warnings)]
#![no_std]

extern crate embedded_hal;

use embedded_hal::digital::OutputPin;


pub enum Command {
    Noop = 0x00,
    Digit0 = 0x01,
    DecodeMode = 0x09,
    Intensity = 0x0A,
    ScanLimit = 0x0B,
    Power = 0x0C,
    DisplayTest = 0x0F
}


pub struct MAX7219<DATA, CS, CLK> {
    data: DATA,
    cs: CS,
    clk: CLK
}

impl<DATA, CS, CLK> MAX7219<DATA, CS, CLK>
    where DATA: OutputPin, CS: OutputPin, CLK: OutputPin {
        pub fn new(data: DATA, cs: CS, clk: CLK) -> Self {
            let mut max7219 = MAX7219 {
                data, cs, clk
            };

            max7219.init();
            return max7219;
        }

        pub fn init(&mut self) {
            self.cs.set_high();
            self.write_command(Command::DisplayTest);
            self.write_data(Command::ScanLimit, 0x07);
            self.write_command(Command::DecodeMode);
            self.clear_display();
            self.power_off();
        }

        pub fn power_on(&mut self) {
            self.write_data(Command::Power, 0x01);
        }

        pub fn power_off(&mut self) {
            self.write_data(Command::Power, 0x00);
        }

        pub fn write_command(&mut self, command: Command) {
            self.write_data(command, 0x00);
        }

        pub fn write_data(&mut self, command: Command, data: u8) {
            self.write_raw(command as u8, data);
        }

        pub fn write_raw(&mut self, header: u8, data: u8) {
            self.shift_out(header);
            self.shift_out(data);
            self.latch();
        }

        pub fn latch(&mut self) {
            self.cs.set_high();
            self.cs.set_low();
        }

        pub fn set_intensity(&mut self, intensity: u8) {
            self.write_data(Command::Intensity, intensity);
        }

        fn shift_out(&mut self, value: u8) {
            for i in 0..8 {
                if value & (1 << (7 - i)) > 0 {
                    self.data.set_high();
                } else {
                    self.data.set_low();
                }

                self.clk.set_high();
                self.clk.set_low();
            }

        }

        pub fn clear_display(&mut self) {
            for i in 1..9 {
                self.write_raw(i, 0x00);
            }
        }

        pub fn test(&mut self, is_on: bool) {
            if is_on {
                self.write_raw(0x01, 0x01);
            } else {
                self.write_raw(0x01, 0x00);
            }
        }
    }
