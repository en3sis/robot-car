#bin/bash
source app.config

echo $USER_NAME

cargo build --target armv7-unknown-linux-musleabihf
scp target/armv7-unknown-linux-musleabihf/debug/robot-car $USER_NAME@$HOST:/home/en3sis
ssh $USER_NAME@$HOST sh -c "./robot-car"
