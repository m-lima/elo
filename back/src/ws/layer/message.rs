pub type Id = u32;

pub trait Message: serde::Serialize {}
impl<T> Message for Response<T> where T: serde::Serialize {}
impl Message for Error {}
impl<T> Message for Push<T> where T: serde::Serialize {}

#[derive(Debug, serde::Serialize)]
pub struct Response<T> {
    pub id: Id,
    pub ok: T,
}

#[derive(Debug, serde::Serialize)]
pub struct Error {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id>,
    pub error: super::service::Error,
}

#[derive(Debug, serde::Serialize)]
pub struct Push<T> {
    pub push: T,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[cfg_attr(test, derive(PartialEq, serde::Serialize))]
pub struct Request<T> {
    pub id: Id,
    #[serde(rename = "do")]
    pub payload: T,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[cfg_attr(test, derive(PartialEq, serde::Serialize))]
pub struct OnlyId {
    pub id: Id,
}
