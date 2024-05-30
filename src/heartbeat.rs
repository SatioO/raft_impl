use crate::raft::Raft;
use std::{
    pin::pin,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};

pub(crate) const HEARTBEAT_INTERVAL: Duration = Duration::from_millis(1000);

#[derive(Debug)]
pub(crate) struct HeartbeatsDaemon {
    start: Instant,
    last_trigger: Arc<AtomicU64>,
    sender: tokio::sync::broadcast::Sender<()>,
}

impl HeartbeatsDaemon {
    const HEARTBEAT_MAX_DELAY_MILLIS: u64 = 30;

    pub fn create() -> Self {
        let (sender, _) = tokio::sync::broadcast::channel(1);
        Self {
            start: Instant::now(),
            last_trigger: Arc::new(AtomicU64::new(0)),
            sender,
        }
    }
}

impl<Command> Raft<Command> {
    /// Schedules tasks that send heartbeats to peers.
    ///
    /// One task is scheduled for each peer. The task sleeps for a duration
    /// specified by `interval`, wakes up, builds the request message to send
    /// and delegates the actual RPC-sending to another task before going back
    /// to sleep.
    ///
    /// The sleeping task does nothing if we are not the leader.
    ///
    /// The request message is a stripped down version of `AppendEntries`. The
    /// response from the peer is ignored.
    pub(crate) fn schedule_heartbeats(&self, interval: Duration) {
        let rf = self.inner_state.clone();
        let mut trigger = self.heartbeats_daemon.sender.subscribe();
        let peers = self.peers.clone();
        let keep_running = self.keep_running.clone();

        self.thread_pool.spawn(async move {
            let mut interval = tokio::time::interval(interval);
            while keep_running.load(Ordering::Relaxed) {
                let tick = pin!(interval.tick());
                let trigger = pin!(trigger.recv());

                let _ = futures_util::future::select(tick, trigger).await;
                println!("send heartbeat");
            }
        });
    }
}
