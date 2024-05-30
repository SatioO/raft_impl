use crate::log_array::{Index, LogArray};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Peer(pub usize);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Term(pub usize);

#[derive(Debug)]
pub(crate) enum State {
    Leader,
    Follower,
    Candidate,
}

#[derive(Debug)]
pub(crate) struct RaftState<Command> {
    pub current_term: Term,
    pub log: LogArray<Command>,
    // Who was voted for in the most recent term
    pub voted_for: Option<Index>,

    // Index of highest log entry known to be committed
    pub commit_index: Index,

    // Index of highest log entry applied to state machine
    pub last_applied: Index,

    // Candidate, follower, or leader
    pub state: State,
}

impl<Command> RaftState<Command> {
    pub fn create() -> Self {
        RaftState {
            current_term: Term(0),
            voted_for: None,
            log: LogArray::create(),
            commit_index: 0,
            last_applied: 0,
            state: State::Follower,
        }
    }
}
