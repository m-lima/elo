mod message;
mod mode;

pub use mode::Mode;

enum FlowControl<T> {
    Break,
    Continue,
    Pass(T),
}

#[allow(clippy::struct_field_names)]
pub struct Socket<M>
where
    M: Mode,
{
    id: String,
    socket: axum::extract::ws::WebSocket,
    _mode: std::marker::PhantomData<M>,
}

impl<M> Socket<M>
where
    M: Mode,
{
    pub fn new(socket: axum::extract::ws::WebSocket) -> Self {
        let id = format!("{id:04x}", id = rand::random::<u16>());
        tracing::debug!(ws = %id, mode = %M::mode(), "Opening websocket");

        Self {
            id,
            socket,
            _mode: std::marker::PhantomData,
        }
    }

    #[tracing::instrument(skip_all, fields(ws = %self.id, mode = %M::mode()))]
    pub async fn serve<S, I, O, P>(
        mut self,
        mut service: S,
        mut broadcast: tokio::sync::broadcast::Receiver<P>,
    ) where
        S: tower_service::Service<I, Response = O>,
        S::Error: Into<message::Error>,
        I: serde::de::DeserializeOwned,
        O: serde::Serialize,
        P: Clone + serde::Serialize,
    {
        macro_rules! flow {
            ($flow_control: expr) => {
                match $flow_control {
                    FlowControl::Pass(value) => value,
                    FlowControl::Break => break,
                    FlowControl::Continue => continue,
                }
            };
        }

        loop {
            tokio::select! {
                () = tokio::time::sleep(std::time::Duration::from_secs(30)) => self.heartbeat().await,
                message = broadcast.recv() => {
                    let push = match message {
                        Ok(push) => push,
                        Err(error) => {
                            tracing::warn!(ws = %self.id, mode = %M::mode(), %error, "Failed to read from broadcaster");
                            continue;
                        }
                    };
                    tracing::debug!(ws = %self.id, mode = %M::mode(), "Pushing message");
                    flow!(self.send(None, push).await);
                }
                request = self.recv() => {
                    let (id, payload) = flow!(request);
                    match service.call(payload).await {
                        Ok(response) => flow!(self.send(id, response).await),
                        Err(err) => {
                            let payload = message::ErrorPayload { error: err.into() };
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

    async fn recv<T>(&mut self) -> FlowControl<(Option<message::Id>, T)>
    where
        T: serde::de::DeserializeOwned,
    {
        // Closed socket
        let Some(message) = self.socket.recv().await else {
            tracing::debug!(ws = %self.id, mode = %M::mode(), "Closing websocket");
            return FlowControl::Break;
        };

        // Broken socket
        let message = match message {
            Ok(message) => message,
            Err(error) => {
                tracing::warn!(ws = %self.id, mode = %M::mode(), %error, "Broken websocket");
                return FlowControl::Break;
            }
        };

        let bytes = match message {
            // Control messages
            axum::extract::ws::Message::Ping(_) | axum::extract::ws::Message::Pong(_) => {
                tracing::debug!(ws = %self.id, mode = %M::mode(), "Received ping");
                return FlowControl::Continue;
            }
            axum::extract::ws::Message::Close(_) => {
                tracing::debug!(ws = %self.id, mode = %M::mode(), "Received close request");
                return FlowControl::Continue;
            }

            // Payload messages
            axum::extract::ws::Message::Text(text) => text.into_bytes(),
            axum::extract::ws::Message::Binary(binary) => binary,
        };

        match M::deserialize(&bytes) {
            Ok(message::WithId { id, payload }) => FlowControl::Pass((id, payload)),
            Err(error) => {
                tracing::warn!(ws = %self.id, mode = %M::mode(), %error, "Failed to deserialize request");
                let error = message::Error::new(hyper::StatusCode::BAD_REQUEST, error.to_string());
                self.send(try_extract_id::<M>(&bytes), message::ErrorPayload { error })
                    .await
            }
        }
    }

    async fn send<T, R>(&mut self, id: Option<message::Id>, payload: T) -> FlowControl<R>
    where
        T: serde::Serialize,
    {
        match M::serialize(message::WithId { id, payload }) {
            Ok(message) => {
                if let Err(error) = self.socket.send(message).await {
                    tracing::error!(ws = %self.id, mode = %M::mode(), %error, "Failed to send message");
                    FlowControl::Break
                } else {
                    FlowControl::Continue
                }
            }
            Err(error) => {
                tracing::error!(ws = %self.id, mode = %M::mode(), %error, "Failed to serialize message");
                FlowControl::Break
            }
        }
    }
}

fn try_extract_id<M>(bytes: &[u8]) -> Option<message::Id>
where
    M: Mode,
{
    M::deserialize::<message::WithId<()>>(bytes).map_or(None, |r| r.id)
}

#[cfg(test)]
mod tests {
    use super::{message::WithId, Mode};

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
