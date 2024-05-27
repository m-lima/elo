mod mode;
mod payload;

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
    user: String,
    service: S,
    socket: axum::extract::ws::WebSocket,
    _mode: std::marker::PhantomData<M>,
}

impl<M, S> Layer<M, S>
where
    M: Mode,
    S: service::Service,
{
    pub fn new(socket: axum::extract::ws::WebSocket, service: S, user: String) -> Self {
        let id = format!("{id:04x}", id = rand::random::<u16>());
        tracing::debug!(ws = %id, %user, mode = %M::mode(), "Opening websocket");

        Self {
            id,
            user,
            socket,
            service,
            _mode: std::marker::PhantomData,
        }
    }

    #[tracing::instrument(skip_all, fields(ws = %self.id, user = %self.user, mode = %M::mode()))]
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
                    tracing::debug!("Pushing message");
                    flow!(self.send(None, push).await);
                }
                request = self.recv() => {
                    use service::Request;
                    use service::IntoError;

                    let start = std::time::Instant::now();
                    let (id, payload) = flow!(request);
                    let action = payload.action();

                    match self.service.call(payload).await {
                        Ok(response) =>{
                            tracing::info!(latency = ?start.elapsed(), "{action}");
                            flow!(self.send(id, response).await);
                        }
                        Err(error) => {
                            if error.is_warn() {
                                tracing::warn!(%error, latency = ?start.elapsed(), "{action}");
                            } else {
                                tracing::error!(%error, latency = ?start.elapsed(), "{action}");
                            }

                            let payload = payload::Error::from(error);
                            flow!(self.send(id, payload).await);
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

    async fn recv(&mut self) -> FlowControl<(Option<payload::Id>, S::Request)> {
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
            Ok(payload::WithId { id, payload }) => FlowControl::Pass((id, payload)),
            Err(error) => {
                tracing::warn!(%error, "Failed to deserialize request");
                let error = service::Error::new(hyper::StatusCode::BAD_REQUEST, error.to_string());
                self.send(try_extract_id::<M>(&bytes), payload::Error::from(error))
                    .await
            }
        }
    }

    async fn send<T, R>(&mut self, id: Option<payload::Id>, payload: T) -> FlowControl<R>
    where
        T: serde::Serialize,
    {
        match M::serialize(payload::WithId { id, payload }) {
            Ok(message) => {
                if let Err(error) = self.socket.send(message).await {
                    tracing::error!(%error, "Failed to send message");
                    FlowControl::Break
                } else {
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

fn try_extract_id<M>(bytes: &[u8]) -> Option<payload::Id>
where
    M: Mode,
{
    M::deserialize::<payload::WithId<()>>(bytes).map_or(None, |r| r.id)
}

#[cfg(test)]
mod tests {
    use super::{mode::sealed::Mode, payload::WithId};

    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    struct Payload {
        name: String,
        count: u32,
    }

    #[test]
    fn try_extract_id_present() {
        let id =
            super::try_extract_id::<String>(br#"{"id": 27, "name": "name_value", "count": 8855}"#);

        assert_eq!(id, Some(27));
    }

    #[test]
    fn try_extract_id_missing() {
        let id = super::try_extract_id::<String>(br#"{"name": "name_value", "count": 8855}"#);

        assert_eq!(id, None);
    }

    #[test]
    fn serialize_with_id() {
        let payload = WithId {
            id: Some(27),
            payload: Payload {
                name: String::from("name_value"),
                count: 8855,
            },
        };

        let output = String::serialize(payload).unwrap();

        let expected = axum::extract::ws::Message::Text(String::from(
            r#"{"id":27,"name":"name_value","count":8855}"#,
        ));

        assert_eq!(output, expected);
    }

    #[test]
    fn serialize_without_id() {
        let payload = WithId {
            id: None,
            payload: Payload {
                name: String::from("name_value"),
                count: 8855,
            },
        };

        let output = String::serialize(payload).unwrap();

        let expected =
            axum::extract::ws::Message::Text(String::from(r#"{"name":"name_value","count":8855}"#));

        assert_eq!(output, expected);
    }

    #[test]
    fn deserialize_with_id() {
        let expected = WithId {
            id: Some(27),
            payload: Payload {
                name: String::from("name_value"),
                count: 8855,
            },
        };

        let output = String::deserialize::<WithId<Payload>>(
            br#"{"id":27,"name":"name_value","count":8855}"#,
        )
        .unwrap();

        assert_eq!(output, expected);
    }

    #[test]
    fn deserialize_without_id() {
        let expected = WithId {
            id: None,
            payload: Payload {
                name: String::from("name_value"),
                count: 8855,
            },
        };

        let output =
            String::deserialize::<WithId<Payload>>(br#"{"name":"name_value","count":8855}"#)
                .unwrap();

        assert_eq!(output, expected);
    }
}
