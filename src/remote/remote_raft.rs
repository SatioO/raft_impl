pub trait RemoteRaft<Command> {
    async fn request_vote(&self) -> std::io::Result<()>;
}
