# Rust & OSOYOO PCA9685 

<!--toc:start-->
- [Rust & OSOYOO PCA9685](#rust-osoyoo-pca9685)
    - [Building](#building)
    - [Resources](#resources)
    - [Notes:](#notes)
<!--toc:end-->

### Building
https://osoyoo.com/2022/07/21/osoyoo-raspberry-pi-car-v2-1-lesson-1-basic-install-and-coding-gpio-pca9685-python/

### Resources
GPI: https://github.com/golemparts/rppal?tab=readme-ov-file#gpio    
PWM: https://docs.rs/pwm-pca9685/latest/pwm_pca9685/       
Linux Embedded: https://docs.rs/linux-embedded-hal/latest/linux_embedded_hal/   

### Notes:
hal = hardware abstraction layer   
I2C = Inter-Integrated Circuit    
cross = https://github.com/cross-rs/cross

### Xbox controller

https://index.ros.org/p/teleop_twist_joy/github-ros2-teleop_twist_joy/

Install with:

```
sudo apt install ros-humble-teleop-twist-joy 
```

Run with (open new terminal):

```
source /opt/ros/humble/setup.bash
ros2 launch teleop_twist_joy teleop-launch.py joy_config:='xbox'
```

If you want to see if topics are being published:

```
ros2 topic list
```

To see topic contents:

```
ros2 topic echo <topic-name>
```

