#bin/bash

# This is a script to cross-copile the project and deploy it to the target device, useful for Raspberry Pi2
source app.config
# Build the project
cargo build --target armv7-unknown-linux-musleabihf
# Copy the binary to the target device
scp target/armv7-unknown-linux-musleabihf/debug/robot-car $USER_NAME@$HOST:/home/en3sis
ssh $USER_NAME@$HOST sh -c "./robot-car"
