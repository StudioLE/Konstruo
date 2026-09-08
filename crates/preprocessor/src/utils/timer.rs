//! Measure elapsed time across pauses.

use std::fmt::Result as FmtResult;
use std::fmt::{Display, Formatter};
use std::time::{Duration, Instant};

/// Measure elapsed time across pauses.
///
/// - Starts running when created
/// - Accumulates only the spans between [`Timer::resume`] and [`Timer::stop`]
pub struct Timer {
    /// Instant the timer last started running.
    start: Instant,
    /// Time accumulated over the spans the timer has run for.
    elapsed: Duration,
    /// Is the timer stopped?
    stopped: bool,
}

impl Timer {
    /// Create a new running [`Timer`].
    #[must_use]
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
            elapsed: Duration::ZERO,
            stopped: false,
        }
    }

    /// Create a new stopped [`Timer`].
    ///
    /// - Accumulates nothing until [`Timer::resume`]
    #[must_use]
    pub fn new_stopped() -> Self {
        Self {
            start: Instant::now(),
            elapsed: Duration::ZERO,
            stopped: true,
        }
    }

    /// Stop the timer, adding the span since it started to the total.
    ///
    /// # Panics
    ///
    /// - Panics IF the timer is already stopped
    pub fn stop(&mut self) {
        assert!(!self.stopped, "Can't stop a stopped timer");
        self.add_span();
        self.stopped = true;
    }

    /// Start the timer running again, keeping the total so far.
    ///
    /// # Panics
    ///
    /// - Panics IF the timer is not stopped
    pub fn resume(&mut self) {
        assert!(self.stopped, "Can't resume a timer that isn't stopped");
        self.start = Instant::now();
        self.stopped = false;
    }

    /// Add the span since the timer started to the total.
    fn add_span(&mut self) {
        self.elapsed += self.start.elapsed();
    }
}

impl Display for Timer {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if self.stopped {
            write!(f, "{:.3}s", self.elapsed.as_secs_f64())
        } else {
            write!(f, "running")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;

    /// Time passing while stopped is left out of the total.
    #[test]
    fn timer_resume() {
        // Arrange
        let mut timer = Timer::new();
        sleep(Duration::from_millis(20));
        timer.stop();
        // Act
        sleep(Duration::from_millis(200));
        timer.resume();
        sleep(Duration::from_millis(20));
        timer.stop();
        // Assert
        assert!(timer.elapsed >= Duration::from_millis(40));
        assert!(timer.elapsed < Duration::from_millis(150));
    }

    #[test]
    #[should_panic(expected = "Can't stop a stopped timer")]
    fn timer_stop_stopped() {
        let mut timer = Timer::new();
        timer.stop();
        timer.stop();
    }

    #[test]
    #[should_panic(expected = "Can't resume a timer that isn't stopped")]
    fn timer_resume_running() {
        let mut timer = Timer::new();
        timer.resume();
    }
}
