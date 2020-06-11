extern crate sysfs_gpio;

use std::thread::sleep;
use std::time::Duration;

use sysfs_gpio::{Direction, Pin};

use log::{info, warn};

fn main() {
    info!(target: "rust-pi-blink", "Starting service");

    let my_led = Pin::new(5); // number depends on chip, etc.
    let sleep_time = 1;
    my_led.with_exported(|| {
        my_led.set_direction(Direction::Out).unwrap();
        loop {
            my_led.set_value(0).unwrap();
            sleep(Duration::from_secs(sleep_time));
            my_led.set_value(1).unwrap();
            sleep(Duration::from_secs(sleep_time));
        }
    }).unwrap();
}
