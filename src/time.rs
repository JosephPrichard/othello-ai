/*
 * Copyright (c) Joseph Prichard 2022.
 */

use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn current_time() -> Duration {
    SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards???")
}

pub fn current_time_millis() -> u128 {
    current_time().as_millis()
}

pub fn current_time_secs() -> u64 {
    current_time().as_secs()
}

