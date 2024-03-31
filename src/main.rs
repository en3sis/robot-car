mod joystick;
mod radxa_controller;

use anyhow::{Error, Result};
use joystick::joystick_to_pwm;
use radxa_controller::RadxaController;
use std::env;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Error> {
    println!("Init controller");
    let mut radxa_pwm = RadxaController::new();

    println!("Initializing ros...");
    let context = rclrs::Context::new(env::args())?;
    let node = rclrs::create_node(&context, "minimal_subscriber")?;

    println!("Subscribing to joystick topic");
    let _subscription = node.create_subscription::<sensor_msgs::msg::Joy, _>(
        "joy",
        rclrs::QOS_PROFILE_SENSOR_DATA,
        move |msg: sensor_msgs::msg::Joy| {
            println!("Joystic output: [{}, {}]", msg.axes[0], msg.axes[1]);
            let (pwm_left, pwm_right) = joystick_to_pwm(msg.axes[0], msg.axes[1]);
            radxa_pwm.set_vel(pwm_right as i16, pwm_left as i16);
        },
    )?;

    println!("Subscribed, waiting for messages");
    rclrs::spin(node).map_err(|err| err.into())
}
