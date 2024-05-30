mod config;
mod durio;
mod election;
mod heartbeat;
mod kv;
mod log_array;
mod raft;
mod raft_state;
mod remote;
mod state_machine;
mod storage;

use crate::{raft::Raft, state_machine::StateMachine};
use durio::LazyRaftServiceClient;
use kv::storage::KVStorage;
use std::{collections::HashMap, net::SocketAddr};

const IP: [u8; 4] = [127, 0, 0, 1];

fn main() {
    let raft_addr: Vec<SocketAddr> = vec![(IP, 9001).into(), (IP, 9002).into(), (IP, 9003).into()];
    let servers = raft_addr
        .into_iter()
        .map(|addr| LazyRaftServiceClient { socket_addr: addr })
        .collect();

    // let config = Config::new();

    // let state_machine = StateMachine {
    //     db: HashMap::new(),
    //     server: config.index,
    // };

    let storage = KVStorage::default();

    let raft: Raft<String> = Raft::new(servers, 0, storage);

    loop {}
    // println!("final state: {:?}", server);
}
