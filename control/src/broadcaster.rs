#[derive(Debug)]
pub struct Broadcaster<T>
where
    T: Clone,
{
    sender: tokio::sync::broadcast::Sender<T>,
}

impl<T> Broadcaster<T>
where
    T: Clone,
{
    pub fn new() -> Self {
        let (sender, _) = tokio::sync::broadcast::channel(16);
        Self { sender }
    }

    pub fn subscribe(&self) -> tokio::sync::broadcast::Receiver<T> {
        self.sender.subscribe()
    }

    pub fn send(&self, payload: T) {
        if let Ok(count) = self.sender.send(payload) {
            if count == 1 {
                tracing::debug!(target: "elo::control", "Broadcasting to 1 listener");
            } else {
                tracing::debug!(target: "elo::control", "Broadcasting to {count} listeners");
            }
        }
    }
}
