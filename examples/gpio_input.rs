use std::error::Error;
use rpi_embedded::gpio::Gpio;

const GPIO_BUTT: u8 = 21; //pin where the button is, button might bounce if done wrong
const GPIO_OUT: u8 = 17; // Just a LED to test

fn main() -> Result<(), Box<dyn Error>> {
    let pin_butt = Gpio::input(GPIO_BUTT)?;
    let mut pin_out = Gpio::output(GPIO_OUT)?;

    pin_out.set_low();//start low, not neccersery but just nice to have

    loop {
        if pin_butt.is_high() {
            pin_out.set_high();
        }else{
            pin_out.set_low();
        }
    }
}
