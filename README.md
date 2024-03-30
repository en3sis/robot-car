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

### Prerequisites
Follow install steps on:

https://github.com/ros2-rust/ros2_rust

### Local compilation

You need to source the ros2_rust environment every time you work on a new terminal:

```bash
source ~/workspace/install/setup.bash
```
After sourcing once, you can do cargo build

### Xbox controller

https://index.ros.org/p/teleop_twist_joy/github-ros2-teleop_twist_joy/

Install with:

```bash
sudo apt install ros-humble-teleop-twist-joy 
```

Run with (open new terminal):

```bash
sudo su
source /opt/ros/humble/setup.bash
ros2 launch teleop_twist_joy teleop-launch.py joy_config:='xbox'
```
### Troubleshouting with ros2 tools



Remember that every time you need to run ros commands, you need to source the setup.bash file. You need to do that in every new terminal you open.
```bash
source /opt/ros/humble/setup.bash
```

If you want to see if topics are being published:

```bash
ros2 topic list
```

To see topic contents:

```bash
ros2 topic echo <topic-name>
```

