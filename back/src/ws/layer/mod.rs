mod message;
mod mode;

pub use mode::Mode;

use super::service;

enum FlowControl<T> {
    Break,
    Continue,
    Pass(T),
}

pub struct Layer<M, S>
where
    M: Mode,
    S: service::Service,
{
    id: String,
    service: S,
    socket: axum::extract::ws::WebSocket,
    _mode: std::marker::PhantomData<M>,
}

impl<M, S> Layer<M, S>
where
    M: Mode,
    S: service::Service,
{
    pub fn new(socket: axum::extract::ws::WebSocket, service: S) -> Self {
        let id = format!("{id:04x}", id = rand::random::<u16>());
        tracing::debug!(ws = %id, mode = %M::mode(), "Opening websocket");

        Self {
            id,
            socket,
            service,
            _mode: std::marker::PhantomData,
        }
    }

    #[tracing::instrument(skip_all, fields(ws = %self.id, mode = %M::mode()))]
    pub async fn serve(mut self) {
        macro_rules! flow {
            ($flow_control: expr) => {
                match $flow_control {
                    FlowControl::Pass(value) => value,
                    FlowControl::Break => break,
                    FlowControl::Continue => continue,
                }
            };
        }

        let mut broadcast = self.service.subscribe();

        loop {
            tokio::select! {
                () = tokio::time::sleep(std::time::Duration::from_secs(30)) => self.heartbeat().await,
                message = broadcast.recv() => {
                    let push = match message {
                        Ok(push) => push,
                        Err(error) => {
                            tracing::warn!(%error, "Failed to read from broadcaster");
                            continue;
                        }
                    };

                    let message = message::Push { push };
                    tracing::debug!("Pushing message");
                    flow!(self.send(message).await);
                }
                request = self.recv() => {
                    let message::Request{ id, payload } = flow!(request);

                    match self.service.call(payload).await {
                        Ok(ok) =>{
                            let message = message::Response { id, ok };
                            tracing::debug!("Responding OK");
                            flow!(self.send(message).await);
                        }
                        Err(error) => {
                            let error = error.into();
                            let message = message::Error { id: Some(id), error };
                            tracing::debug!("Responding ERROR");
                            flow!(self.send(message).await);
                        },
                    }
                }
            }
        }
    }

    async fn heartbeat(&mut self) {
        tracing::debug!("Sending heartbeat");
        if let Err(error) = self
            .socket
            .send(axum::extract::ws::Message::Ping(Vec::new()))
            .await
        {
            tracing::warn!(%error, "Failed to send heartbeat");
        }
    }

    async fn recv(&mut self) -> FlowControl<message::Request<S::Request>> {
        // Closed socket
        let Some(message) = self.socket.recv().await else {
            tracing::debug!("Closing websocket");
            return FlowControl::Break;
        };

        // Broken socket
        let message = match message {
            Ok(message) => message,
            Err(error) => {
                tracing::debug!(%error, "Closing broken websocket");
                return FlowControl::Break;
            }
        };

        let bytes = match message {
            // Control messages
            axum::extract::ws::Message::Ping(_) | axum::extract::ws::Message::Pong(_) => {
                tracing::debug!("Received ping");
                return FlowControl::Continue;
            }
            axum::extract::ws::Message::Close(_) => {
                tracing::debug!("Received close request");
                return FlowControl::Continue;
            }

            // Payload messages
            axum::extract::ws::Message::Text(text) => text.into_bytes(),
            axum::extract::ws::Message::Binary(binary) => binary,
        };

        match M::deserialize(&bytes) {
            Ok(message) => FlowControl::Pass(message),
            Err(error) => {
                tracing::warn!(%error, "Failed to deserialize request");
                let message = message::Error {
                    id: try_extract_id::<M>(&bytes),
                    error: service::Error::new(hyper::StatusCode::BAD_REQUEST, &error),
                };
                self.send(message).await
            }
        }
    }

    async fn send<T, R>(&mut self, message: T) -> FlowControl<R>
    where
        T: message::Message,
    {
        match M::serialize(message) {
            Ok((message, len)) => {
                if let Err(error) = self.socket.send(message).await {
                    tracing::error!(%error, "Failed to send message");
                    FlowControl::Break
                } else {
                    tracing::debug!(bytes = %len, "Message sent");
                    FlowControl::Continue
                }
            }
            Err(error) => {
                tracing::error!(%error, "Failed to serialize message");
                FlowControl::Break
            }
        }
    }
}

fn try_extract_id<M>(bytes: &[u8]) -> Option<message::Id>
where
    M: Mode,
{
    M::deserialize::<message::OnlyId>(bytes).map(|r| r.id).ok()
}

#[cfg(test)]
mod tests {
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    struct Payload<'a> {
        name: &'a str,
        count: u32,
    }

    const OBJ: Payload<'_> = Payload {
        name: "name_value",
        count: 8855,
    };
    const STR: &str = r#"{"name":"name_value","count":8855}"#;

    mod request {
        use super::{
            super::{message::Request, try_extract_id},
            Payload, OBJ, STR,
        };

        #[test]
        fn try_extract_id_present() {
            let id = try_extract_id::<String>(format!(r#"{{"do":{STR},"id":27}}"#).as_bytes());

            assert_eq!(id, Some(27));
        }

        #[test]
        fn try_extract_id_missing() {
            let id = try_extract_id::<String>(format!(r#"{{"do":{STR}}}"#).as_bytes());

            assert_eq!(id, None);
        }

        #[test]
        fn try_extract_id_bad_request() {
            let id = try_extract_id::<String>(br#"{"id":27}"#);

            assert_eq!(id, Some(27));
        }

        #[test]
        fn try_extract_id_really_bad_request() {
            let id = try_extract_id::<String>(br#"{"id":27,"":{""}}"#);

            assert_eq!(id, None);
        }

        #[test]
        fn happy() {
            let payload = format!(r#"{{"do":{STR},"id":27}}"#);
            let message = <String as super::super::mode::sealed::Mode>::deserialize::<
                Request<Payload<'_>>,
            >(payload.as_bytes())
            .unwrap();

            let expected = Request {
                id: 27,
                payload: OBJ,
            };
            assert_eq!(message, expected);
        }
    }

    mod push {
        use super::{
            super::{message::Push, mode::sealed::Mode},
            OBJ, STR,
        };

        #[test]
        fn happy() {
            let payload = Push { push: OBJ };

            let output = String::serialize(payload).unwrap().0;

            let expected = axum::extract::ws::Message::Text(format!(r#"{{"push":{STR}}}"#));

            assert_eq!(output, expected);
        }
    }

    mod response {
        use super::{
            super::{message::Response, mode::sealed::Mode},
            OBJ, STR,
        };

        #[test]
        fn happy() {
            let payload = Response { id: 27, ok: OBJ };

            let output = String::serialize(payload).unwrap().0;

            let expected = axum::extract::ws::Message::Text(format!(r#"{{"id":27,"ok":{STR}}}"#,));

            assert_eq!(output, expected);
        }
    }

    mod error {
        use super::super::{message::Error, mode::sealed::Mode, service};

        const STR: &str = r#"{"code":400,"message":"Bad request"}"#;

        #[test]
        fn with_id() {
            let payload = Error {
                id: Some(27),
                error: service::Error::new(hyper::StatusCode::BAD_REQUEST, "Bad request"),
            };

            let output = String::serialize(payload).unwrap().0;

            let expected =
                axum::extract::ws::Message::Text(format!(r#"{{"id":27,"error":{STR}}}"#,));

            assert_eq!(output, expected);
        }

        #[test]
        fn without_id() {
            let payload = Error {
                id: None,
                error: service::Error::new(hyper::StatusCode::BAD_REQUEST, "Bad request"),
            };

            let output = String::serialize(payload).unwrap().0;

            let expected = axum::extract::ws::Message::Text(format!(r#"{{"error":{STR}}}"#,));

            assert_eq!(output, expected);
        }
    }
}
