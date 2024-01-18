use std::time::{SystemTime, UNIX_EPOCH};

const SECONDS_IN_MINUTE: u64 = 60;
const SECONDS_IN_HOUR: u64 = 60 * SECONDS_IN_MINUTE;

/// Converts seconds to date components
///
/// # Returns
///
/// Returns a tuple of the current hour, minute and second
#[inline]
pub(crate) fn utc_now() -> (u8, u8, u8) {
    let duration_since_epoch = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards!");
    let seconds = duration_since_epoch.as_secs();

    let hour = seconds / SECONDS_IN_HOUR % 24;
    let minute = seconds / SECONDS_IN_MINUTE % 60;
    let second = seconds % 60;

    (hour as u8, minute as u8, second as u8)
}
