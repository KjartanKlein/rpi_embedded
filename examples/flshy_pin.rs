use rpi_embedded::gpio::{Gpio};


fn main(){
    let mut pin = Gpio::output(21).unwrap();
    loop{
        pin.toggle();
    }
}
