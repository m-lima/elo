use super::model;

#[derive(Debug, Clone)]
pub struct Broadcaster {
    sender: tokio::sync::broadcast::Sender<model::Push>,
}

impl Broadcaster {
    pub fn new() -> Self {
        let (sender, _) = tokio::sync::broadcast::channel(16);
        Self { sender }
    }

    pub fn subscribe(&self) -> tokio::sync::broadcast::Receiver<model::Push> {
        self.sender.subscribe()
    }

    pub fn send(&self, payload: model::Push) {
        let message = payload.to_string();
        if let Ok(count) = self.sender.send(payload) {
            tracing::info!(listeners = %count, "Push {message}");
        }
    }
}
