use std::thread;
use std::time::Duration;

use rpi_embedded::adxl::Adxl;
fn main() {

    let mut accel = Adxl::new();
    accel.start();
    accel.get_offsets();
    accel.get_power_status();
    println!("I2C ID: {} \t Power Status: {} \t XYZ Offsets ({}, {}, {})",accel.id, accel.power_status, accel.offsets[0],accel.offsets[1],accel.offsets[2]);
    while accel.get_power_status() != 8 {
        println!("Power status is not in the desired mode writing 8 (0x08)");
        accel.set_power_status(8);
        thread::sleep(Duration::from_millis(10)); //Give it some time to react before trying agian
    }
    println!("Starting mesurements");

    loop{
        accel.get_data_raw();
        println!("GOT raw [ {:?} ]",accel.raw_data);
        accel.get_data();
        println!("GOT clean [ {:?} ]",accel.data);
        println!("Got rotations[ {} , {} ]",accel.pitch , accel.roll);
        thread::sleep(Duration::from_millis(200));
    }
}
