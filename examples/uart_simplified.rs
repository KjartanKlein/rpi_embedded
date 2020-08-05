
use rpi_embedded::uart::Uart;

use rpi_embedded::uart::Parity;

use std::time::Duration;
use std::thread;
fn main(){

//let mut uart = Uart::set().unwrap();
let mut uart = Uart::new(115_200, Parity::None, 8, 1).unwrap();
//THis is the same as calling
//   Uart::new(115_200, Parity::None, 8, 1). As this is used as a defualt mode

// there are a couple of ways to read values, and other stuff however translating from String
//is the only hard thing left to do
uart.set_read_mode(1, Duration::default()).unwrap();

uart.write(String::from("Hello there \n")).unwrap();
let res_1 = uart.read_until('\n').unwrap(); //reads uart in blocking until the character happens or the buffer of size 255 was emptied
println!("Result 1| {}",res_1);
thread::sleep(Duration::from_millis(500));

uart.write(String::from("good day \n")).unwrap();
let res_2 = uart.read_line().unwrap(); //does the same as the line above, simply just a simplifacation
println!("Result 2| {}",res_2);
thread::sleep(Duration::from_millis(500));
let mut buffer = [0u32;24];

uart.write(String::from("52,53,54\n")).unwrap();
let k = uart.read_csv(&mut buffer).unwrap();
print!("Result 3| {}",buffer[0]);
for j in 1..k {
    print!(",{}",buffer[(j) as usize])
}
println!("");
}
