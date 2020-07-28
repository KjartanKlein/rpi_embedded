
use std::error::Error;
use std::thread;
use std::time::Duration;
use rpi_embedded::servo;



fn main() -> Result<(), Box<dyn Error>> {

    let mut pwm = servo::Servo::new(0);//new servo on PWM0 see pwm for more info on that
    let mut pwm1 = servo::Servo::new(1);//new servo on PWM1 see pwm for more info on that
    //servos should not be set to anything above 180 or below zero, some servos will allow this
    
    loop {
        pwm.write_pwm(0).expect("setting servo 1 failed"); //servo1 --> 0°
        pwm1.write_pwm(0).expect("setting servo 2 failed");//servo2 --> 0°
        thread::sleep(Duration::from_millis(1000));

        pwm1.write(90).expect("setting servo 2 failed");//servo1 --> 90°
        pwm.write_pwm(180).expect("setting servo 1 failed");//servo2 --> 180°
        thread::sleep(Duration::from_millis(1000));
    }
}
