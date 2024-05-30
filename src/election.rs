use std::{
    sync::{Condvar, Mutex},
    time::{Duration, Instant},
};

use rand::{thread_rng, Rng};

#[derive(Debug)]
struct VersionedDeadline {
    version: usize,
    deadline: Option<Instant>,
}

#[derive(Debug)]
pub(crate) struct ElectionState {
    // Timer will be removed upon shutdown or elected
    timer: Mutex<VersionedDeadline>,

    // Wake up the timer thread when the timer is reset or cancelled
    signal: Condvar,
}

const ELECTION_TIMEOUT_BASE_MILLIS: u64 = 200;
const ELECTION_TIMEOUT_VAR_MILLIS: u64 = 200;

impl ElectionState {
    pub(crate) fn create() -> Self {
        Self {
            timer: Mutex::new(VersionedDeadline {
                version: 0,
                deadline: None,
            }),
            signal: Condvar::new(),
        }
    }

    pub(crate) fn reset_election_timer(&self) {
        let mut guard = self.timer.lock().unwrap();
        guard.version += 1;
        guard.deadline.replace(Self::election_timeout());
        self.signal.notify_one();
    }

    fn election_timeout() -> Instant {
        Instant::now()
            + Duration::from_millis(
                ELECTION_TIMEOUT_BASE_MILLIS
                    + thread_rng().gen_range(0..ELECTION_TIMEOUT_VAR_MILLIS),
            )
    }
}
