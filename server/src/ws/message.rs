pub type Id = u32;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WithId<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id>,
    #[serde(flatten)]
    pub payload: T,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct ErrorPayload {
    pub error: Error,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct Error {
    #[serde(serialize_with = "to_u16")]
    pub code: hyper::StatusCode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl Error {
    pub fn new<M: Into<String>>(code: hyper::StatusCode, message: M) -> Self {
        Self {
            code,
            message: Some(message.into()),
        }
    }
}

impl From<hyper::StatusCode> for Error {
    fn from(code: hyper::StatusCode) -> Self {
        Self {
            code,
            message: None,
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
