mod message;

pub use message::Error;

pub trait Service {
    type Request: Request;
    type Response: serde::Serialize;
    type Error: IntoError;
    type Push: Clone + serde::Serialize;

    fn subscribe(&self) -> tokio::sync::broadcast::Receiver<Self::Push>;
    fn call(
        &mut self,
        request: Self::Request,
    ) -> impl std::future::Future<Output = Result<Self::Response, Self::Error>>;
}

pub trait Request: serde::de::DeserializeOwned {
    fn action(&self) -> &'static str;
}

pub trait IntoError: Into<message::Error> + std::fmt::Display {
    fn is_warn(&self) -> bool;
}
