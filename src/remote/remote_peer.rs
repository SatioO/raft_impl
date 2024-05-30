pub(crate) struct RemotePeer<UniqueID> {
    pub unique_id: UniqueID,
}

impl<UniqueID> RemotePeer<UniqueID> {
    pub fn create(unique_id: UniqueID) -> Self {
        RemotePeer { unique_id }
    }
}
