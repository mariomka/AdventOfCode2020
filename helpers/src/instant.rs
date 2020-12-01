use std::time::{Duration, Instant};

pub trait BasicInstant {
    fn now() -> Self;

    fn elapsed(&self) -> Duration;
}

impl BasicInstant for Instant {
    fn now() -> Self {
        Instant::now()
    }

    fn elapsed(&self) -> Duration {
        self.elapsed()
    }
}

#[cfg(test)]
pub mod fake {
    use std::cell::Cell;
    use std::time::Duration;

    thread_local! {
        static FAKE_ELAPSED: Cell<Duration> = Cell::new(Duration::default());
    }

    pub fn with_fake_elapsed(duration: Duration) {
        FAKE_ELAPSED.with(|cell| cell.set(duration));
    }

    #[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
    pub struct Instant;

    impl super::BasicInstant for Instant {
        fn now() -> Self {
            Instant
        }

        fn elapsed(&self) -> Duration {
            FAKE_ELAPSED.with(|cell| cell.get())
        }
    }
}
