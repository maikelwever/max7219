//! A platform agnostic driver to interface with the MAX7219 (LED matrix display driver)
//!
//! This driver was built using [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://docs.rs/embedded-hal/~0.1

#![deny(unsafe_code)]
//#![deny(warnings)]
#![no_std]

extern crate embedded_hal;

use embedded_hal::digital::OutputPin;

pub enum Command {
    Noop = 0x00,
    Digit0 = 0x01,
    Digit1 = 0x02,
    Digit2 = 0x03,
    Digit3 = 0x04,
    Digit4 = 0x05,
    Digit5 = 0x06,
    Digit6 = 0x07,
    Digit7 = 0x08,
    DecodeMode = 0x09,
    Intensity = 0x0A,
    ScanLimit = 0x0B,
    Power = 0x0C,
    DisplayTest = 0x0F,
}

pub enum DecodeMode {
    NoDecode = 0x00,
    CodeBDigit0 = 0x01,
    CodeBDigits3_0 = 0x0F,
    CodeBDigits7_0 = 0xFF,
}

pub struct MAX7219<DATA, CS, CLK> {
    data: DATA,
    cs: CS,
    clk: CLK,
    devices: u8,
    buffer: [u8; 8],
}

impl<DATA, CS, CLK> MAX7219<DATA, CS, CLK>
where
    DATA: OutputPin,
    CS: OutputPin,
    CLK: OutputPin,
{
    pub fn new(devices: u8, data: DATA, cs: CS, clk: CLK) -> Self {
        let mut num_devices = devices;
        if num_devices > 8 {
            num_devices = 8;
        }

        let mut max7219 = MAX7219 {
            data,
            cs,
            clk,
            devices: num_devices,
            buffer: [0; 8],
        };

        max7219.init();
        max7219
    }

    pub fn init(&mut self) {
        for i in 0..self.devices {
            self.write_command(i, Command::DisplayTest);
            self.write_data(i, Command::ScanLimit, 0x07);
            self.set_decode_mode(i, DecodeMode::NoDecode);
            self.clear_display(i);
        }
        self.power_off();
    }

    pub fn set_decode_mode(&mut self, addr: u8, mode: DecodeMode) {
        self.write_data(addr, Command::DecodeMode, mode as u8);
    }

    pub fn power_on(&mut self) {
        for i in 0..self.devices {
            self.write_data(i, Command::Power, 0x01);
        }
    }

    pub fn power_off(&mut self) {
        for i in 0..self.devices {
            self.write_data(i, Command::Power, 0x00);
        }
    }

    pub fn write_command(&mut self, addr: u8, command: Command) {
        self.write_data(addr, command, 0x00);
    }

    pub fn write_data(&mut self, addr: u8, command: Command, data: u8) {
        self.write_raw(addr, command as u8, data);
    }

    fn empty_buffer(&mut self) {
        self.buffer = [0; 8];
    }

    pub fn write_raw(&mut self, addr: u8, header: u8, data: u8) {
        let offset = addr * 2;
        let max_bytes = self.devices * 2;
        self.empty_buffer();

        self.buffer[offset as usize] = header;
        self.buffer[offset as usize + 1] = data;

        self.cs.set_low();
        for i in 0..max_bytes {
            let buffer_data = self.buffer[i as usize];
            self.shift_out(buffer_data);
        }
        self.cs.set_high();
    }

    pub fn set_intensity(&mut self, addr: u8, intensity: u8) {
        self.write_data(addr, Command::Intensity, intensity);
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

    pub fn clear_display(&mut self, addr: u8) {
        for i in 1..9 {
            self.write_raw(addr, i, 0x00);
        }
    }

    pub fn test(&mut self, addr: u8, is_on: bool) {
        if is_on {
            self.write_raw(addr, 0x01, 0x01);
        } else {
            self.write_raw(addr, 0x01, 0x00);
        }
    }
}
