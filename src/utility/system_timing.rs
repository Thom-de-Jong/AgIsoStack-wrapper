//================================================================================================
/// @file system_timing.rs
///
/// @brief Utility class for getting system time and handling u32 time rollover
/// @author Thom de Jong
///
/// @copyright 2023 Thom de Jong
//================================================================================================

use std::time::{Instant, Duration};

lazy_static::lazy_static! {
	static ref STARTUP_TIME: Instant = Instant::now();
}

pub fn get_timestamp() -> Instant {
	STARTUP_TIME.clone()
}

pub fn get_time_elapsed() -> Duration {
	STARTUP_TIME.elapsed()
}

pub fn time_expired(time_stamp: Instant, timeout: Duration) -> bool {
	time_stamp.elapsed() >= timeout
}