pub type Id = u32;

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Message<Response, Push> {
    Ok(Id, Response),
    Error(
        #[serde(skip_serializing_if = "Option::is_none")] Option<Id>,
        super::service::Error,
    ),
    Push(Push),
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
