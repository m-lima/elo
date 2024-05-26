#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WithId<T> {
    pub id: Option<u32>,
    #[serde(flatten)]
    pub payload: T,
}

pub type Request = WithId<request::Message>;
pub type Response = WithId<response::Message>;

pub mod request {
    use super::WithId;

    pub enum Message {
        User(WithId<()>),
    }
}

pub mod response {
    use super::WithId;

    pub enum Message {
        Push(WithId<Push>),
        Response(WithId<Response>),
        Error(WithId<Error>),
    }

    pub enum Push {
        UserCreated(types::User),
    }

    pub enum Response {
        User(types::User),
    }

    pub struct Error {
        pub kind: u8,
        pub message: String,
    }

    pub enum Kind {
        BadRequest,
    }
}
