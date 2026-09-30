use std::time::{Duration, Instant};
use vexide::prelude::*;

pub struct Ticker {
    period: Duration,
    next: Instant,
}

impl Ticker {
    /// Creates a default ticker
    pub fn new(period: Duration) -> Self {
        Self {
            period,
            next: Instant::now() + period,
        }
    }

    /// Makes sure the robot's main loop is always run in the time decided by the ticker.
    /// This should always be the last thing called in the main loop
    pub async fn tick(&mut self) -> Instant {
        let deadline = self.next;

        sleep_until(deadline).await;

        let now = Instant::now();

        // If the work of the main loop puts us behind schedule,
        // decrease the amount of time in between the next tick.
        self.next = if deadline + self.period > now {
            deadline + self.period
        } else {
            now + self.period
        };

        deadline
    }
}
