extern crate sysfs_gpio;

use std::thread::sleep;
use std::time::Duration;

use env_logger::Env;
use log::info;
use sysfs_gpio::{Direction, Pin};

fn main() {
    env_logger::from_env(Env::default().default_filter_or("info")).init();
    info!(target: "rust-pi-blink", "Starting service");

    let my_led = Pin::new(5); // number depends on chip, etc.
    let sleep_time = Duration::from_secs(1);
    my_led
        .with_exported(|| {
            my_led.set_direction(Direction::Out).unwrap();
            loop {
                my_led.set_value(0).unwrap();
                sleep(sleep_time);
                my_led.set_value(1).unwrap();
                sleep(sleep_time);
            }
        })
        .unwrap();
}
