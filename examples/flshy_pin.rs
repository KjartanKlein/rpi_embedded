use rpi_embedded::gpio::{Gpio};
// this example is a demo that shows how much of the processor is taken if code is malused
// took about 96% of my processor when running it,

fn main(){
    let mut pin = Gpio::output(21).unwrap();
    loop{
        pin.toggle();
    }
}
