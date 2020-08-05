// Copyright (c) 2017-2019 Rene van der Meer
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

//! rpi_embedded is a fork of the RPPAL library. This fork is made to increase the usability
//! of the RPPAL library. Spesificaly making it more user friendly and beginer friendly
//! rpi_embedded provides access to the Raspberry Pi's GPIO, I2C, PWM, SPI, UART and Bluetooth
//! peripherals. There is also a ADXL345 and pwm servo library included for ease of use.
//! RPPAL also offers support for USB to serial adapters. The library
//! can be used in conjunction with a variety of platform-agnostic drivers
//! through its `embedded-hal` trait implementations by enabling the optional
//! `hal` feature. However the new functions included in rpi_embedded might fail.
//!
//! rpi_embedded requires Raspbian or any similar, recent, Linux distribution.
//! rpie_embedded has only been tested on Rpi Zero W but RPPAL is compatible with
//! the Raspberry Pi A, A+, B, B+, 2B, 3A+, 3B, 3B+, 4B, CM, CM 3, CM 3+, Zero and
//! Zero W. In theory it should all work except for bluetooth maybe.
//!

// Used by rustdoc to link other crates to rppal's docs
#![doc(html_root_url = "https://docs.rs/rpi_embedded")]

#[macro_use]
mod macros;

pub mod gpio;
#[cfg(feature = "hal")]
pub mod hal;
pub mod i2c;
pub mod pwm;
pub mod spi;
pub mod system;
pub mod uart;
pub mod servo;
pub mod adxl;
