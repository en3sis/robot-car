#!/bin/bash

# Check if the script is run as root (sudo)
if [ "$EUID" -eq 0 ]; then
    echo "The script is running with root privileges."
else
    echo "Please run the script with sudo."
    exit 1
fi

source /opt/ros/humble/setup.bash
source /home/pirobot/workspace/install/setup.bash

target/debug/robot-car