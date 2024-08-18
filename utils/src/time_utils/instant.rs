//! This `Instant` measures time with high performance and high accuracy powered by TSC.
use super::cycles;
use std::time::Duration;

/// A measurement of a monotonically nondecreasing clock.
#[derive(Debug, Default, Clone, Copy)]
pub struct Instant(u64);

impl Instant {
    /// Returns an `Instant` corresponding to "now".
    #[inline]
    pub fn now() -> Self {
        Instant(cycles::rdtsc())
    }

    /// Returns the amount of CPU cycles from another `Instant` to this one,
    /// or `None` if that `Instant` is later than this one.
    pub fn checked_cycles_since(&self, earlier: Instant) -> Option<u64> {
        self.0.checked_sub(earlier.0)
    }

    /// Returns the amount of nanoseconds from another `Instant` to this one,
    /// or `None` if that `Instant` is later than this one.
    pub fn checked_nanos_since(&self, earlier: Instant) -> Option<f64> {
        self.checked_cycles_since(earlier)
            .map(cycles::convert_cycles_to_ns_f64)
    }

    /// Returns the amount of `Duration` from another `Instant` to this one,
    /// or `None` if that `Instant` is later than this one.
    pub fn checked_duration_since(&self, earlier: Instant) -> Option<Duration> {
        self.checked_cycles_since(earlier)
            .map(cycles::convert_cycles_to_duration)
    }
}
