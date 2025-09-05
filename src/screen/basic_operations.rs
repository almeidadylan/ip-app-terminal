use std::{thread::sleep, time::Duration};

pub fn clear_screen () {
    clearscreen::clear().expect("Failed to Clear Screen");
}

fn _wait(time: u64) {
    sleep(Duration::from_secs(time));
}