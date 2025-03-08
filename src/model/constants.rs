use std::time::Duration;

pub const PLAYER_COUNT: usize = 2;

pub const TICKS_PER_SECOND: usize = 60;
pub const TICK_INTERVAL: Duration = Duration::from_millis(1000 / TICKS_PER_SECOND as u64);
