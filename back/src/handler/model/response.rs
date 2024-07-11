use crate::types;

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Response {
    User {
        id: types::Id,
        #[serde(skip_serializing_if = "Option::is_none")]
        pending: Option<bool>,
    },
    Players(Vec<types::PlayerTuple>),
    Games(Vec<types::GameTuple>),
    Invites(Vec<types::InviteTuple>),
    Done,
}
