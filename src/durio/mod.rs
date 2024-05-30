use std::net::SocketAddr;

use tokio::sync::OnceCell;

use crate::remote::remote_raft::RemoteRaft;

pub(crate) trait RaftService {}

pub struct LazyRaftServiceClient {
    pub socket_addr: SocketAddr,
}

impl RemoteRaft<String> for LazyRaftServiceClient {
    async fn request_vote(&self) -> std::io::Result<()> {
        todo!()
    }
}
