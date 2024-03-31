# Rust & OSOYOO PCA9685

<!--toc:start-->
- [Rust & OSOYOO PCA9685](#rust-osoyoo-pca9685)
  - [Building](#building)
  - [Prerequisites](#prerequisites)
  - [Init repo](#init-repo)
  - [MacOS Cross-compilation](#macos-cross-compilation)
  - [Local compilation](#local-compilation)
  - [Xbox controller](#xbox-controller)
  - [Troubleshouting with ros2 tools](#troubleshouting-with-ros2-tools)
    - [Resources](#resources)
    - [Notes:](#notes)
<!--toc:end-->

## Building
https://osoyoo.com/2022/07/21/osoyoo-raspberry-pi-car-v2-1-lesson-1-basic-install-and-coding-gpio-pca9685-python/

## Prerequisites
If needed, install vcs tool:

```bash
pip3 install vcstool
```

## Init repo

```bash
git submodule update --init
vcs import src < src/ros2_rust/ros2_rust_humble.repos
```

## MacOS Cross-compilation
On MacOS, we can use docker to cross-compile the code. First, we need to build the docker image:
```bash
docker build -t ros2_rust .
docker run --rm --network=host -v $PWD:$PWD -w $PWD -it ros2_rust /bin/bash
```

## Local compilation
You need to source the ros2_rust environment every time you work on a new terminal:

```bash
source install/setup.bash
```
Then, we can build with `colcon build`:

```bash
colcon build
```

## Xbox controller

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
## Troubleshouting with ros2 tools

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
---

### Resources
- GPI: https://github.com/golemparts/rppal?tab=readme-ov-file#gpio   
- PWM: https://docs.rs/pwm-pca9685/latest/pwm_pca9685/   
- Linux Embedded: https://docs.rs/linux-embedded-hal/latest/linux_embedded_hal/   

### Notes:
- hal = hardware abstraction layer
- I2C = Inter-Integrated Circuit
- cross = https://github.com/cross-rs/cross
