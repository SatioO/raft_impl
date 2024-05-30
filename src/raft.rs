use crate::{
    election::ElectionState,
    heartbeat::{HeartbeatsDaemon, HEARTBEAT_INTERVAL},
    log_array::{Index, LogEntry},
    raft_state::{Peer, RaftState},
    remote::{remote_peer::RemotePeer, remote_raft::RemoteRaft},
    state_machine::StateMachine,
    storage::{RaftStoragePersisterTrait, RaftStorageTrait},
};
use std::{
    sync::{atomic::AtomicBool, Arc, Mutex},
    time::Duration,
};

#[derive(Debug, Default, Clone)]
pub struct ClusterMember {
    pub id: u64,
    pub address: String,

    // Highest log entry known to be replicated
    pub match_index: Index,

    // Index of the next log entry to send
    pub next_index: Index,
}

#[allow(dead_code)]
pub struct Raft<Command> {
    pub(crate) inner_state: Arc<Mutex<RaftState<Command>>>,
    // ----------- PERSISTENT STATE -----------
    // the current term
    // current_term: u64,
    // log: Vec<Entry>,
    // votedFor is stored in `cluster []ClusterMember` below,
    // mapped by `clusterIndex` below

    // ----------- READONLY STATE -----------
    // Unique identifier for this Server
    // id: u64,

    // // The TCP address for RPC
    // address: String,

    // When to start elections after no append entry messages
    // election_timeout: Option<time::Instant>,

    // // How often to send empty messages
    // heartbeat_ms: u64,

    // // When to send next empty message
    // heartbeat_timeout: Option<time::Instant>,

    // User-provided state machine
    // state_machine: StateMachine,

    // Metadata directory
    // metadata_dir: String,

    // // Metadata store
    // fd: Option<File>,
    // ----------- VOLATILE STATE -----------
    // // Index of highest log entry known to be committed
    // commit_index: u64,

    // // Index of highest log entry applied to state machine
    // last_applied: u64,

    // Candidate, follower, or leader
    // pub state: Mutex<ServerState>,

    // Servers in the cluster, including this one
    // The index of current server
    pub(crate) election: Arc<ElectionState>,
    pub(crate) persister: Arc<dyn RaftStoragePersisterTrait<LogEntry<Command>>>,
    pub(crate) peers: Vec<Peer>,
    pub(crate) peer: Peer,
    pub(crate) heartbeats_daemon: HeartbeatsDaemon,
    pub(crate) thread_pool: tokio::runtime::Handle,
    pub(crate) keep_running: Arc<AtomicBool>,
    // join_handle: Arc<Mutex<Option<RaftJoinHandle>>>,
}

impl<Command> Raft<Command> {
    pub fn new(
        peers: Vec<impl RemoteRaft<Command>>,
        peer_index: usize,
        storage: impl RaftStorageTrait,
    ) -> Self {
        let peer_size = peers.len();
        assert!(
            peer_size > peer_index,
            "Peer Index should be smaller than number of peers"
        );

        let mut raft_state = RaftState::create();
        // if let Ok(stored_state) = storage.read_state() {
        //     // TODO: Yet to be developed
        // }

        let inner_state = Arc::new(Mutex::new(raft_state));
        let election = Arc::new(ElectionState::create());
        election.reset_election_timer();

        let persister = storage.persister();

        // let remote_peers = peers
        //     .into_iter()
        //     .enumerate()
        //     .map(|(index, remote_raft)| RemotePeer::create(Peer(index)))
        //     .collect();

        let peers = (0..peer_size)
            .filter(|p| *p != peer_index)
            .map(Peer)
            .collect();

        println!("peers: {:?}", peers);

        let thread_pool = tokio::runtime::Builder::new_multi_thread()
            .enable_io()
            .enable_time()
            .thread_name(format!("raft-{}", peer_index))
            .worker_threads(peer_size)
            .build()
            .expect("Creating thread pool should not fail");

        let this = Raft {
            peers,
            peer: Peer(peer_index),
            inner_state,
            election,
            persister,
            heartbeats_daemon: HeartbeatsDaemon::create(),
            thread_pool: thread_pool.handle().clone(),
            keep_running: Arc::new(AtomicBool::new(true)),
            // join_handle: Arc::new(Mutex::new(None)),
        };

        this.schedule_heartbeats(HEARTBEAT_INTERVAL);
        // this.join_handle
        //     .lock()
        //     .unwrap()
        //     .replace(RaftJoinHandle { thread_pool });

        // this.thread_pool.block_on(async {});

        this
    }

    // pub fn start(&mut self) {}

    // fn timeout(&mut self) {
    //     // if self.election_timeout.is_none() {
    //     //     return;
    //     // }

    //     // let has_timed_out = Instant::now() > self.election_timeout.unwrap();

    //     // if has_timed_out {
    //     //     println!("Timed out.. starting new election");
    //     //     // self.state = State::Candidate.into();
    //     //     // self.current_term += 1;
    //     // }
    // }

    // pub fn ensure_log(&mut self) {
    //     // if self.log.len() == 0 {
    //     //     self.log.push(Entry::default());
    //     // }
    // }

    // fn u64_from_le_bytes(bytes: &[u8]) -> u64 {
    //     u64::from_le_bytes([
    //         bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
    //     ])
    // }

    // // fn set_voted_for(&mut self, id: u64) {
    // // if let Some(node) = self.cluster.get_mut(self.cluster_index) {
    // //     node.voted_for = Some(id);
    // // } else {
    // //     eprintln!("Invalid cluster");
    // // }
    // // }

    // fn reset_election_timeout(&mut self) {
    //     // let mut rng = rand::thread_rng();

    //     // let interval_ms = rng.gen_range(0..self.heartbeat_ms * 2) + self.heartbeat_ms * 2;
    //     // let interval = Duration::from_millis(interval_ms);
    //     // println!("{}", format!("New interval: {:?}.", interval));
    //     // self.election_timeout = Some(Instant::now() + interval);
    // }

    // pub fn restore(&mut self) {
    //     // if self.fd.is_none() {
    //     //     let path = PathBuf::from(format!("{}/md_{}.dat", self.metadata_dir, self.id));

    //     //     let file = OpenOptions::new()
    //     //         .read(true)
    //     //         .write(true)
    //     //         .create(true)
    //     //         .open(path)
    //     //         .unwrap();
    //     //     self.fd = Some(file);
    //     // }

    //     // // Bytes 0  - 8:   Current term
    //     // // Bytes 8  - 16:  Voted for
    //     // // Bytes 16 - 24:  Log length
    //     // // Bytes 4096 - N: Log
    //     // let fd = self.fd.as_mut().unwrap();
    //     // fd.seek(SeekFrom::Start(0)).unwrap();

    //     // let mut page = [0u8; PAGE_SIZE];
    //     // let n = fd.read(&mut page).unwrap();

    //     // if n == 0 {
    //     //     self.ensure_log();
    //     //     return;
    //     // } else if n != PAGE_SIZE {
    //     //     panic!("Read full page failed");
    //     // }

    //     // self.current_term = Self::u64_from_le_bytes(&page[0..8]);
    //     // self.set_voted_for(Self::u64_from_le_bytes(&page[8..16]));
    //     // let len_log = Self::u64_from_le_bytes(&page[16..24]);
    //     // self.log.clear();

    //     // TODO: revisit this logic as persist logic is not yet clear
    //     // if len_log > 0 {}

    //     // self.ensure_log();
    // }
}

// #[must_use]
// #[derive(Debug)]
// pub struct RaftJoinHandle {
//     thread_pool: tokio::runtime::Runtime,
// }

// impl RaftJoinHandle {
//     const SHUTDOWN_TIMEOUT: std::time::Duration =
//         Duration::from_millis(HEARTBEAT_INTERVAL.as_millis() as u64 * 2);

//     pub fn join(self) {
//         self.thread_pool.shutdown_timeout(Self::SHUTDOWN_TIMEOUT);
//     }
// }
