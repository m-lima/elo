pub type Id = u32;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WithId<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id>,
    #[serde(flatten)]
    pub payload: T,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct Error {
    pub error: super::service::Error,
}

impl<E> From<E> for Error
where
    E: Into<super::service::Error>,
{
    fn from(value: E) -> Self {
        Self {
            error: value.into(),
        }
    }
}
