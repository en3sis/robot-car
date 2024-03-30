// use std::error::Error;
use std::thread;
use std::time::Duration;

mod radxa_controller;
use radxa_controller::RadxaController;

use std::env;
use anyhow::{Error, Result};

fn joystick_to_pwm(x_axis: f32, y_axis: f32) -> (i32, i32) {
    let pwm_min = -4000;
    let pwm_max = 4000;
    let pwm_range = pwm_max - pwm_min;

    // Splitting the forward/backward movement and turning command handling
    let s = y_axis.max(-1.0).min(1.0);  // Ensure within range
    let t = x_axis.max(-1.0).min(1.0);  // Ensure within range

    let (mut pwm_l, mut pwm_r);

    if s.abs() > 0.0 {
        // Forward/backward movement with turning
        pwm_l = s + t * s.abs();  // Adjust left PWM based on turn magnitude and direction
        pwm_r = s - t * s.abs();  // Adjust right PWM similarly
    } else {
        // Pure turning - spin in place
        pwm_l = -t;
        pwm_r = t;
    }

    // Normalize PWM values to the expected range for each motor
    let normalize_pwm = |pwm: f32| -> i32 {
        (((pwm + 1.0) / 2.0 * pwm_range as f32) + pwm_min as f32) as i32
    };

    pwm_l = pwm_l.max(-1.0).min(1.0);
    pwm_r = pwm_r.max(-1.0).min(1.0);

    (normalize_pwm(pwm_l), normalize_pwm(pwm_r))
}

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
            println!("Joystic output: [{}, {}]", msg.axes[0],msg.axes[1]);
            let (pwm_left, pwm_right) = joystick_to_pwm(msg.axes[0], msg.axes[1]);
            radxa_pwm.set_vel(pwm_right as i16 , pwm_left as i16);
        },
    )?;
    println!("Subscribed, waiting for messages");
    rclrs::spin(node).map_err(|err| err.into())
}