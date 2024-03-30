use std::error::Error;
use std::thread;
use std::time::Duration;

mod radxa_controller;
use radxa_controller::RadxaController;
// use rppal::gpio::Gpio;
// use rppal::system::DeviceInfo;

// Gpio uses BCM pin numbering. BCM GPIO 23 is tied to physical pin 16.
// const GPIO_LED: u8 = 23;
// const IN1: u8 = 23;
// const IN2: u8 = 24;
// const IN3: u8 = 27;
// const IN4: u8 = 22;

fn main() -> Result<(), Box<dyn Error>> {
    // println!("Hello an LED on a {}.", DeviceInfo::new()?.model());
    let mut radxa_pwm = RadxaController::new();
    println!("Turning off the LED");
    radxa_pwm.set_vel(100,100);
    thread::sleep(Duration::from_secs(1));

    radxa_pwm.set_vel(100,-100);
    thread::sleep(Duration::from_secs(1));

    radxa_pwm.set_vel(-100,100);
    thread::sleep(Duration::from_secs(1));

    radxa_pwm.set_vel(-100,-100);
    thread::sleep(Duration::from_secs(1));

    radxa_pwm.set_vel(0,0);
    thread::sleep(Duration::from_secs(1));
    // get the I2C device back
    // pwm.set_channel_on_off(Channel::C0, 0, 0).unwrap();
    // pwm.set_channel_on_off(Channel::C1, 0, 0).unwrap();

    // let dev = pwm.destroy();

    Ok(())
}
