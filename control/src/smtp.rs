pub enum Payload {
    Invite(String),
    _Challenge(types::Id),
    _Match(types::Id),
}

impl Payload {
    pub async fn send(self) {
        let connection = tokio::net::TcpStream::connect("smtp-relay.gmail.com:587")
            .await
            .unwrap();
        todo!("{connection:?}");
    }
}
