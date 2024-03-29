use embedded_hal::digital::OutputPin;
use linux_embedded_hal::I2cdev;
use pwm_pca9685::{Address, Channel, Pca9685};
use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;

// Gpio uses BCM pin numbering. BCM GPIO 23 is tied to physical pin 16.
const GPIO_LED: u8 = 23;
const IN1: u8 = 23;
const IN2: u8 = 24;
const IN3: u8 = 27;
const IN4: u8 = 22;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello an LED on a {}.", DeviceInfo::new()?.model());

    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let address = Address::default();
    let mut pwm = Pca9685::new(dev, address).unwrap();
    // do something...
    pwm.set_prescale(100).unwrap();
    pwm.enable().unwrap();

    let mut pin1 = Gpio::new()?.get(IN1)?.into_output();
    let mut pin2 = Gpio::new()?.get(IN2)?.into_output();
    let mut pin3 = Gpio::new()?.get(IN3)?.into_output();
    let mut pin4 = Gpio::new()?.get(IN4)?.into_output();

    // backward
    pin1.set_high();
    pin2.set_low();
    pin3.set_high();
    pin4.set_low();

    // forward
    pin1.set_low();
    pin2.set_high();
    pin3.set_low();
    pin4.set_high();

    pwm.set_channel_on_off(Channel::C0, 0, 2000).unwrap();
    pwm.set_channel_on_off(Channel::C1, 0, 2000).unwrap();

    // sleep for 1 seconds
    thread::sleep(Duration::from_secs(1));
    println!("Turning off the LED");
    // get the I2C device back
    pwm.set_channel_on_off(Channel::C0, 0, 0).unwrap();
    pwm.set_channel_on_off(Channel::C1, 0, 0).unwrap();

    let dev = pwm.destroy();

    Ok(())
}
