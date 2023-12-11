use std::{
    fmt::Display,
    time::{Duration, Instant},
};

#[derive(Debug)]
pub(super) struct Timer {
    pub elapsed: Duration,
}

impl Default for Timer {
    fn default() -> Self {
        Timer::new()
    }
}

impl Display for Timer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.elapsed)
    }
}

impl Timer {
    pub(super) fn new() -> Timer {
        Self {
            elapsed: Duration::ZERO,
        }
    }

    pub(super) fn span(&mut self) -> TimerGuard<'_> {
        TimerGuard {
            timer: self,
            start: Instant::now(),
        }
    }

    pub(super) fn time<T, F: FnOnce() -> T>(&mut self, f: F) -> T {
        let now = Instant::now();
        let v = f();
        self.elapsed += now.elapsed();
        v
    }
}

#[derive(Debug)]
pub(super) struct TimerGuard<'a> {
    timer: &'a mut Timer,
    start: Instant,
}

impl Drop for TimerGuard<'_> {
    fn drop(&mut self) {
        self.timer.elapsed += self.start.elapsed();
    }
}

#[derive(Debug, Default)]
/// To collect the time spent in various stages inside the debugger.
pub(super) struct ProcessTimer {
    pub global: Timer,
    pub init: Timer,
    pub set_breakpoint: Timer,
    pub debugger_launch: Timer,
    pub debugger_run: Timer,
    pub debugger_cleanup: Timer,
    pub handle_breakpoint: Timer,
    pub process_resume: Timer,
}
