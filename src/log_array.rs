use crate::raft_state::Term;

pub type Index = usize;

#[derive(Clone, Debug)]
pub(crate) struct LogEntry<Command> {
    pub index: Index,
    pub term: Term,
    command: Option<Command>,
}

#[derive(Clone, Debug)]
pub(crate) struct LogArray<C> {
    inner: Vec<LogEntry<C>>,
}

impl<C> LogArray<C> {
    /// Create the initial Raft log with no user-supplied commands.
    pub fn create() -> LogArray<C> {
        let ret = LogArray {
            inner: vec![Self::build_first_entry(0, Term(0))],
        };
        ret
    }
}

impl<C> LogArray<C> {
    fn build_first_entry(index: Index, term: Term) -> LogEntry<C> {
        LogEntry {
            index,
            term,
            command: None,
        }
    }
}
