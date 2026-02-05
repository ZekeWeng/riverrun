//! Clock port for time-related operations.

use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Unix timestamp in milliseconds.
pub type Timestamp = u64;

/// Port for time operations.
///
/// This trait abstracts time to allow:
/// - Dependency injection of different time sources
/// - Deterministic testing with fixed/controlled time
/// - Mocking time progression in simulations
pub trait Clock: Send + Sync {
    /// Get the current timestamp in milliseconds since Unix epoch.
    fn now(&self) -> Timestamp;

    /// Get the current time as a SystemTime.
    fn system_time(&self) -> SystemTime {
        UNIX_EPOCH + Duration::from_millis(self.now())
    }
}

/// A clock that uses the system time.
#[derive(Debug, Clone, Copy, Default)]
pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> Timestamp {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before Unix epoch")
            .as_millis() as u64
    }
}

/// A fixed clock that always returns the same timestamp.
///
/// Useful for testing scenarios where deterministic time is needed.
#[derive(Debug, Clone, Copy)]
pub struct FixedClock {
    timestamp: Timestamp,
}

impl FixedClock {
    /// Create a new fixed clock with the given timestamp.
    pub fn new(timestamp: Timestamp) -> Self {
        FixedClock { timestamp }
    }

    /// Create a fixed clock set to Unix epoch (timestamp 0).
    pub fn epoch() -> Self {
        FixedClock { timestamp: 0 }
    }
}

impl Clock for FixedClock {
    fn now(&self) -> Timestamp {
        self.timestamp
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_clock_returns_reasonable_timestamp() {
        let clock = SystemClock;
        let now = clock.now();
        // Should be after 2020-01-01 (1577836800000 ms)
        assert!(now > 1_577_836_800_000);
    }

    #[test]
    fn test_fixed_clock_returns_set_timestamp() {
        let clock = FixedClock::new(1_000_000);
        assert_eq!(clock.now(), 1_000_000);
        // Multiple calls return same value
        assert_eq!(clock.now(), 1_000_000);
    }

    #[test]
    fn test_fixed_clock_epoch() {
        let clock = FixedClock::epoch();
        assert_eq!(clock.now(), 0);
    }

    #[test]
    fn test_system_time_conversion() {
        let clock = FixedClock::new(1_577_836_800_000); // 2020-01-01 00:00:00 UTC
        let system_time = clock.system_time();
        let duration = system_time.duration_since(UNIX_EPOCH).unwrap();
        assert_eq!(duration.as_millis(), 1_577_836_800_000);
    }
}
