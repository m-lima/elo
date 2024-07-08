pub trait Service {
    type Request: serde::de::DeserializeOwned;
    type Response: serde::Serialize;
    type Error: Into<Error>;
    type Push: Clone + serde::Serialize;

    fn subscribe(&self) -> tokio::sync::broadcast::Receiver<Self::Push>;
    fn call(
        &mut self,
        request: Self::Request,
    ) -> impl std::future::Future<Output = Result<Self::Response, Self::Error>>;
}

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct Error {
    #[serde(serialize_with = "to_u16")]
    pub code: hyper::StatusCode,
    pub message: String,
}

impl Error {
    pub fn new<M: ToString + ?Sized>(code: hyper::StatusCode, message: &M) -> Self {
        Self {
            code,
            message: message.to_string(),
        }
    }
}

// allow(clippy::trivially_copy_pass_by_ref): To match serde's signature
#[allow(clippy::trivially_copy_pass_by_ref)]
fn to_u16<S>(code: &hyper::StatusCode, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_u16(code.as_u16())
}
