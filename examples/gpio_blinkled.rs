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

// gpio_blinkled.rs - Blinks an LED connected to a GPIO pin in a loop.
//
// Remember to add a resistor of an appropriate value in series, to prevent
// exceeding the maximum current rating of the GPIO pin and the LED.
//
// Interrupting the process by pressing Ctrl-C causes the application to exit
// immediately without resetting the pin's state, so the LED might stay lit.
// Check out the gpio_blinkled_signals.rs example to learn how to properly
// handle incoming signals to prevent an abnormal termination.

use std::error::Error;
use std::thread;
use std::time::Duration;

use rpi_embedded::gpio::{Mode, Gpio};

// Gpio uses BCM pin numbering. BCM GPIO 23 is tied to physical pin 16.
const GPIO_LED_0: u8 = 16;
const GPIO_LED_1: u8 = 20;
const GPIO_LED_2: u8 = 21;


fn main() -> Result<(), Box<dyn Error>> {
    // Retrieve the GPIO pin and configure it as an output.
    // This is the longest method to get stuff done but it is used in other moethods
    let mut pin_0 = Gpio::new()?.get(GPIO_LED_0)?.into_output();
    let mut pin_1 = Gpio::output(GPIO_LED_1)?;
    let mut pin_2 = Gpio::io(GPIO_LED_2, Mode::Output)?; //This one can switch between input and output easily, use with caution
    loop {
        //set the LEDS one by one to high
        pin_0.set_high();
        thread::sleep(Duration::from_millis(500));
        pin_1.set_high();
        thread::sleep(Duration::from_millis(500));
        pin_2.set_high();
        thread::sleep(Duration::from_millis(500));
        //set the LEDS one by one to low
        pin_0.set_low();
        thread::sleep(Duration::from_millis(500));
        pin_1.set_low();
        thread::sleep(Duration::from_millis(500));
        pin_2.set_low();
        thread::sleep(Duration::from_millis(500));
        //toggle all pins
        pin_1.toggle();
        pin_0.toggle();
        pin_2.toggle();
        thread::sleep(Duration::from_millis(500));
        //toggle all pins
        pin_1.toggle();
        pin_0.toggle();
        pin_2.toggle();
        thread::sleep(Duration::from_millis(500));
    }
}
