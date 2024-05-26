mod mode;
mod model;

enum FlowControl<T> {
    Break,
    Continue,
    Pass(T),
}

pub struct Socket<M: mode::Mode> {
    id: String,
    socket: axum::extract::ws::WebSocket,
    store: store::Store,
    _mode: std::marker::PhantomData<M>,
}

impl<M: mode::Mode> Socket<M> {
    pub fn new(socket: axum::extract::ws::WebSocket, store: store::Store) -> Self {
        let id = format!("{id:04x}", id = rand::random::<u16>());
        tracing::debug!(ws = %id, mode = %M::mode(), "Opening websocket");

        Self {
            id,
            socket,
            store,
            _mode: std::marker::PhantomData,
        }
    }

    // #[tracing::instrument(target = "ws", skip_all, fields(ws = %self.id, mode = %M::mode()))]
    // pub async fn serve(mut self) {
    //     macro_rules! flow {
    //         ($flow_control: expr) => {
    //             match $flow_control {
    //                 FlowControl::Pass(value) => value,
    //                 FlowControl::Break => break,
    //                 FlowControl::Continue => continue,
    //             }
    //         };
    //     }
    //
    //     // let mut broadcast = self.service.subscribe();
    //
    //     loop {
    //         tokio::select! {
    //             () = tokio::time::sleep(std::time::Duration::from_secs(30)) => self.heartbeat().await,
    //             // message = broadcast.recv() => {
    //             //     let push = match message {
    //             //         Ok(push) => model::response::Message::Push(push),
    //             //         Err(error) => {
    //             //             tracing::warn!(ws = %self.id, mode = %M::mode(), %error, "Failed to read from broadcaster");
    //             //             continue;
    //             //         }
    //             //     };
    //             //     tracing::debug!(ws = %self.id, mode = %M::mode(), "Pushing message");
    //             //     flow!(self.send(push).await);
    //             // }
    //             // request = self.recv() => {
    //             //     let start = std::time::Instant::now();
    //             //     let request = flow!(request);
    //             //     let model::Request { id, payload } = request;
    //             //     // let (resource, action) = flow::incoming(&payload);
    //             //     let response = self.service.handle(payload).await;
    //             //     // let outgoing = flow::outgoing(&response);
    //             //
    //             //     let message = match response {
    //             //         payload @ types::Response::Payload(_) => {
    //             //             tracing::info!(ws = %self.id, mode = %M::mode(), %resource, %action, latency = ?start.elapsed(), "{outgoing}");
    //             //             types::Message::Response(types::ws::Response { id, payload })
    //             //         }
    //             //         types::Response::Error(error) => {
    //             //             if error.kind == types::Kind::InternalError {
    //             //                 tracing::error!(ws = %self.id, mode = %M::mode(), %resource, %action, latency = ?start.elapsed(), "{outgoing}");
    //             //             } else {
    //             //                 tracing::warn!(ws = %self.id, mode = %M::mode(), %resource, %action, latency = ?start.elapsed(), "{outgoing}");
    //             //             }
    //             //             types::Message::Response(types::ws::Response { id, payload: types::Response::Error(error) })
    //             //         }
    //             //     };
    //             //
    //             //     flow!(self.send(message).await);
    //             // }
    //         }
    //     }
    // }
    //
    // async fn heartbeat(&mut self) {
    //     tracing::debug!("Sending heartbeat");
    //     if let Err(error) = self
    //         .socket
    //         .send(axum::extract::ws::Message::Ping(Vec::new()))
    //         .await
    //     {
    //         tracing::warn!(%error, "Failed to send heartbeat");
    //     }
    // }
    //
    // async fn recv(&mut self) -> FlowControl<model::Request> {
    //     // Closed socket
    //     let Some(message) = self.socket.recv().await else {
    //         tracing::debug!(ws = %self.id, mode = %M::mode(), "Closing websocket");
    //         return FlowControl::Break;
    //     };
    //
    //     // Broken socket
    //     let message = match message {
    //         Ok(message) => message,
    //         Err(error) => {
    //             tracing::warn!(ws = %self.id, mode = %M::mode(), %error, "Broken websocket");
    //             return FlowControl::Break;
    //         }
    //     };
    //
    //     let bytes = match message {
    //         // Control messages
    //         axum::extract::ws::Message::Ping(_) | axum::extract::ws::Message::Pong(_) => {
    //             tracing::debug!(ws = %self.id, mode = %M::mode(), "Received ping");
    //             return FlowControl::Continue;
    //         }
    //         axum::extract::ws::Message::Close(_) => {
    //             tracing::debug!(ws = %self.id, mode = %M::mode(), "Received close request");
    //             return FlowControl::Continue;
    //         }
    //
    //         // Payload messages
    //         axum::extract::ws::Message::Text(text) => text.into_bytes(),
    //         axum::extract::ws::Message::Binary(binary) => binary,
    //     };
    //
    //     match M::deserialize(&bytes) {
    //         Ok(request) => FlowControl::Pass(request),
    //         Err(error) => {
    //             tracing::warn!(ws = %self.id, mode = %M::mode(), %error, "Failed to deserialize request");
    //             let error = model::response::Error {
    //                 kind: model::response::Kind::BadRequest,
    //                 message: Some(error.to_string()),
    //             };
    //             self.send(model::Response {
    //                 id: M::try_extract_id(&bytes),
    //                 payload: model::response::Message::Error(error),
    //             })
    //             .await
    //         }
    //     }
    // }
    //
    // async fn send<R>(&mut self, message: model::response::Message) -> FlowControl<R> {
    //     match M::serialize(message) {
    //         Ok(message) => {
    //             if let Err(error) = self.socket.send(message).await {
    //                 tracing::error!(ws = %self.id, mode = %M::mode(), %error, "Failed to send message");
    //                 FlowControl::Break
    //             } else {
    //                 FlowControl::Continue
    //             }
    //         }
    //         Err(error) => {
    //             tracing::error!(ws = %self.id, mode = %M::mode(), %error, "Failed to serialize message");
    //             FlowControl::Break
    //         }
    //     }
    // }
}
