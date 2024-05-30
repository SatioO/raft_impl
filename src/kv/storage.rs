use std::sync::Arc;

use crate::storage::{
    RaftLogEntryRef, RaftStoragePersisterTrait, RaftStorageTrait, RaftStoredState,
};

#[derive(Default)]
pub struct KVStorage;

impl RaftStorageTrait for KVStorage {
    type RaftStoragePersister<LogEntry: RaftLogEntryRef> = KVPersister;

    fn persister<LogEntry: RaftLogEntryRef>(
        self,
    ) -> std::sync::Arc<Self::RaftStoragePersister<LogEntry>> {
        Arc::new(KVPersister)
    }

    fn read_state(&self) -> std::io::Result<RaftStoredState> {
        todo!()
    }
}

#[derive(Debug)]
pub struct KVPersister;

impl<LogEntry: RaftLogEntryRef> RaftStoragePersisterTrait<LogEntry> for KVPersister {
    fn save_term_vote(&self, term: crate::raft_state::Term, voted_for: String) {
        todo!()
    }

    fn append_one_entry(&self, entry: &LogEntry) {
        todo!()
    }

    // fn append_entries<'a, LogEntryList>(&self, entries: LogEntryList)
    // where
    //     LogEntry: 'a,
    //     LogEntryList: IntoIterator<Item = &'a LogEntry>,
    // {
    //     todo!()
    // }
}
