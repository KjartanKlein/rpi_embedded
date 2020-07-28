#![allow(dead_code)]  //removes some warnings for the user
//use std::Duration;
use crate::pwm::{Channel,Polarity,Pwm};
use std::time::Duration;


pub struct Servo {
        pin: Pwm,
        pub min_us: u16,
        pub max_us: u16 ,
        pub enable:bool ,
        pub period: u64,
        channel: u8,
}


impl Servo {
    pub fn new(n_channel: u8) -> Self{
        let mut n2_ch = Channel::Pwm0;
        if n_channel == 0 {}
        if n_channel == 1 {n2_ch = Channel::Pwm1}
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

    pub fn set_min_max(&mut self,min:u16,max:u16){
        self.min_us = min;
        self.max_us = max;
    }
    pub fn set_period(&mut self, perio:u64){
        self.period = perio;
        self.pin.set_period(Duration::from_millis(perio)).expect("failed in setting period");
    }

    pub fn enable(&mut self){
            self.enable = true;
            if !self.pin.is_enabled().expect("cant read enable status"){
                self.pin.enable().expect("cant enable");
            }
    }
    pub fn disable(&mut self){
            self.enable = false;
            if self.pin.is_enabled().expect("cant read enable status"){
                self.pin.disable().expect("cant enable");
            }
    }
    pub fn motor_mode(&mut self){
        self.write_pwm(2750).expect("cant write pwm in motor mode");
    }

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
    pub fn is_enabled(&mut self)->bool{
            self.pin.is_enabled().expect("cant read pwm")
    }
    pub fn get_min(&mut self) -> u16{
        self.min_us
    }
    pub fn get_max(&mut self) -> u16{
        self.max_us
    }
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
