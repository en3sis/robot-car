use linux_embedded_hal::sysfs_gpio::{Direction, Pin};
// use std::error::Error;

use pwm_pca9685::{Address, Channel, Pca9685};
use linux_embedded_hal::I2cdev;

pub struct RadxaController {
    pin1: Pin,
    pin2: Pin,
    pin3: Pin,
    pin4: Pin,
    pwm: Pca9685<I2cdev>,
}

impl RadxaController {
    pub fn new() -> RadxaController {

        let dev = I2cdev::new("/dev/i2c-0").unwrap();
        let address = Address::default(); 

        let mut pwm = Pca9685::new(dev, address).unwrap();
        pwm.set_prescale(100).unwrap();
        pwm.enable().unwrap();

        let pin1 = Pin::new(154);
        match pin1.export() {
            Ok(()) => println!("Gpio {} exported!", pin1.get_pin()),
            Err(err) => println!("Gpio {} could not be exported: {}", pin1.get_pin(), err),
        }
        pin1.set_direction(Direction::Out).unwrap();

        let pin2 = Pin::new(156);
        match pin2.export() {
            Ok(()) => println!("Gpio {} exported!", pin2.get_pin()),
            Err(err) => println!("Gpio {} could not be exported: {}", pin2.get_pin(), err),
        }
        pin2.set_direction(Direction::Out).unwrap();
        let pin3 = Pin::new(150);
        match pin3.export() {
            Ok(()) => println!("Gpio {} exported!", pin3.get_pin()),
            Err(err) => println!("Gpio {} could not be exported: {}", pin3.get_pin(), err),
        }
        pin3.set_direction(Direction::Out).unwrap();
        let pin4 = Pin::new(149);
        match pin4.export() {
            Ok(()) => println!("Gpio {} exported!", pin4.get_pin()),
            Err(err) => println!("Gpio {} could not be exported: {}", pin4.get_pin(), err),
        }
        pin4.set_direction(Direction::Out).unwrap();

        RadxaController { pin1, pin2, pin3, pin4, pwm}
    }
    pub fn set_left_dir_forward(&mut self){
        self.pin1.set_value(0).unwrap();
        self.pin2.set_value(1).unwrap();
    }
    pub fn set_left_dir_barckward(&mut self){
        self.pin1.set_value(1).unwrap();
        self.pin2.set_value(0).unwrap();
    }
    pub fn set_right_dir_forward(&mut self){
        self.pin3.set_value(0).unwrap();
        self.pin4.set_value(1).unwrap();
    }
    pub fn set_right_dir_barckward(&mut self){
        self.pin3.set_value(1).unwrap();
        self.pin4.set_value(0).unwrap();
    }

    pub fn set_vel(&mut self, vel_right : i16 , vel_left: i16){
        if vel_right > 0 {
            self.set_right_dir_forward();
        } else {
            self.set_right_dir_barckward()
        }
        if vel_left > 0 {
            self.set_left_dir_forward();
        } else {
            self.set_left_dir_barckward()
        }

        self.pwm.set_channel_on_off(Channel::C0, 0, vel_right.abs().try_into().unwrap()).unwrap();
        self.pwm.set_channel_on_off(Channel::C1, 0, vel_left.abs().try_into().unwrap()).unwrap();
    }
}
