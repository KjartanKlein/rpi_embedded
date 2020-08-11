use std::thread;
use std::time::Duration;

use rpi_embedded::i2c::I2c;


fn main(){
    //opens the I2c port
    let mut i2c = I2c::new().expect("i2c failed in initilazation");
    //sets address as 0x53
    i2c.set_slave_address(0x53).expect("slave adress failed");

    //simple rwrite and read, used if the device is not using internal registers like most sensors
    //for example between two processors like an Arduino and a Rpi
    // the buffer is neccecery for read
    i2c.write(&[0x01 as u8]).expect("write failed");

    let mut buffer = [0u8;1];
    i2c.read(&mut buffer).expect("read failed");
    println!("Basic read write gives -> {}",buffer[0]);

    thread::sleep(Duration::from_millis(5000));

    // writes one command and reads one command, nice to use when changing one register in a device
    // note that theese functions might change to have a diffrent input and output becuse writing
    // hex commands like this is painful
    i2c.cmd_write(0x2D as u8,0x08 as u8).expect("cmd write failed");
    let mut out = [0u8];
    i2c.cmd_read(0x2D as u8, &mut out).expect("cmd write failed");
    println!("Command write with command 0x0D -> {}",out[0]);
    thread::sleep(Duration::from_millis(5000));

    // block read and write, reads/writes as many bytes as the size of the input/output buffer

    let mut buffer_r = [0u8;3];
    let mut buffer_w = [10u8;3];

    i2c.block_write(0x1E as u8, &mut buffer_w).expect("block write failed");
    i2c.block_read(0x1E as u8,&mut buffer_r).expect("block read failed");

    println!("block read with length {} using command 0x1E -> {:?} ", buffer_r.len(), buffer_r);
}
