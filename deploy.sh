# exit when any command fails
set -e

docker-compose up build

scp target/arm-unknown-linux-gnueabihf/release/rust-pi-blink pi@pi:/home/pi/

# -t -t kills the process on ssh down
ssh -t -t pi@pi /home/pi/rust-pi-blink
