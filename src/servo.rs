#![allow(dead_code)]  //removes some warnings for the user
//use std::Duration;
use crate::pwm::{Channel,Polarity,Pwm};
use std::time::Duration;

/// servos should have an internal minimum and maximum PWs check your data sheet and
/// adjust accordingly
pub struct Servo {
        pin: Pwm,
        /// this would be 0-deg
        pub min_us: u16,
        ///this would be MAX deg
        pub max_us: u16 ,
        ///on or off
        pub enable:bool ,
        pub period: u64,
        channel: u8,
}


impl Servo {
    /// Creates a new instance on a PWM channal, should be either 1 or 0 passed into it
    /// do note that it sets the defualt values for input for min and max
    pub fn new(n_channel: u8) -> Self{
        let mut n2_ch = Channel::Pwm0;
        match n_channel{
            0 => {},
            1 => {n2_ch = Channel::Pwm1},
            _ => panic!("ONLY CHANNEL 0 AND 1 ARE CURRENTLY SUPPORTED"),
        }
    let _pwm = Pwm::with_period(
            n2_ch,
            Duration::from_millis(20),
            Duration::from_micros(2500),
            Polarity::Normal,
            true,
        ).unwrap();
        Self{
            pin: _pwm,
            min_us: 500 ,
            max_us: 2500 ,
            enable: false ,
            period: 20,

            channel: n_channel,
        }

    }
    /// allows changing the minima and maxima
    pub fn set_min_max(&mut self,min:u16,max:u16){
        self.min_us = min;
        self.max_us = max;
    }
    /// allows changing the period, same as changing the period for PWM
    pub fn set_period(&mut self, perio:u64){
        self.period = perio;
        self.pin.set_period(Duration::from_millis(perio)).expect("failed in setting period");
    }
    /// Turns the PWM channel on
    pub fn enable(&mut self){
            self.enable = true;
            if !self.pin.is_enabled().expect("cant read enable status"){
                self.pin.enable().expect("cant enable");
            }
    }
    ///Turns the Pwm channel off
    pub fn disable(&mut self){
            self.enable = false;
            if self.pin.is_enabled().expect("cant read enable status"){
                self.pin.disable().expect("cant enable");
            }
    }
    ///USE WITH CAUTION some servos allow spinning if a suitable singal is sent.
    pub fn motor_mode(&mut self){
        self.write_pwm(2750).expect("cant write pwm in motor mode");
    }
    /// takes value as deg and then simply sends a singal to go there, makes sure that it worked
    pub fn write(&mut self, value:u8) -> Result<u64,u64>{
        let n_value = ((self.min_us as f64 + (value as f64)*((self.max_us - self.min_us)as f64/180.0)).floor()) as u64;
        let ok:bool;
        match self.pin.set_pulse_width(Duration::from_micros(n_value)) {
            Ok(_) => ok = true,
            Err(_) => ok = false,
        }
        if ok{
            Ok(n_value)
        }else{
            Err(n_value)
        }
    }
    // same as write but asks for pulse width instead of deg, bypasses any restriction
    pub fn write_pwm(&mut self, value:u64) -> Result<u64,u64>{
        let ok:bool;
        match self.pin.set_pulse_width(Duration::from_micros(value)) {
            Ok(_) => ok = true,
            Err(_) => ok = false,
        }
        if ok{
            Ok(value)
        }else{
            Err(value)
        }

    }
    /// boolean to see if the servo is active
    pub fn is_enabled(&mut self)->bool{
            self.pin.is_enabled().expect("cant read pwm")
    }
    /// gets the minimum pulse width
    pub fn get_min(&mut self) -> u16{
        self.min_us
    }
    /// gets the maximum pulse width
    pub fn get_max(&mut self) -> u16{
        self.max_us
    }
    /// gets the current period from the pwm and double checks if it is the one that is stored
    /// good to use to check if some other code is messing with the servo
    pub fn get_period(&mut self) -> Result<u64,u64>{
        let t : u64 = self.pin.period().expect("can read period").as_millis() as u64;
        if t == self.period {
            Ok(t)
        }
        else {Err(t)}
    }
    pub fn get_channel(&mut self)->u8{
        self.channel
    }
}
