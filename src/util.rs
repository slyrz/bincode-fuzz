use std::io::{self, Write};
use std::time::Duration;

#[inline]
pub fn heartbeat() {
    let mut stdout = io::stdout();
    stdout.write(b".").unwrap();
    stdout.flush().unwrap();
}

#[inline]
pub fn discrete(duration: Duration) -> u64 {
    duration.as_secs() * 1_000_000_000 + (duration.subsec_nanos() as u64)
}
