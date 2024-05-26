pub trait Mode: sealed::Sealed + Send + Sync + 'static {
    type SerializeError: std::fmt::Display + Send;
    type DeserializeError: std::fmt::Display + Send;

    fn mode() -> &'static str;
    fn serialize<T>(payload: T) -> Result<axum::extract::ws::Message, Self::SerializeError>
    where
        T: serde::Serialize;
    fn deserialize<'b, 'de, T>(bytes: &'b [u8]) -> Result<T, Self::DeserializeError>
    where
        'b: 'de,
        T: serde::Deserialize<'de>;
}

impl Mode for String {
    type SerializeError = serde_json::Error;
    type DeserializeError = serde_json::Error;

    fn mode() -> &'static str {
        "text"
    }

    fn serialize<T>(payload: T) -> Result<axum::extract::ws::Message, Self::SerializeError>
    where
        T: serde::Serialize,
    {
        serde_json::to_string(&payload).map(axum::extract::ws::Message::Text)
    }

    fn deserialize<'b, 'de, T>(bytes: &'b [u8]) -> Result<T, Self::DeserializeError>
    where
        'b: 'de,
        T: serde::Deserialize<'de>,
    {
        serde_json::from_slice(bytes)
    }
}

impl Mode for Vec<u8> {
    type SerializeError = rmp_serde::encode::Error;
    type DeserializeError = rmp_serde::decode::Error;

    fn mode() -> &'static str {
        "binary"
    }

    fn serialize<T>(payload: T) -> Result<axum::extract::ws::Message, Self::SerializeError>
    where
        T: serde::Serialize,
    {
        rmp_serde::to_vec(&payload).map(axum::extract::ws::Message::Binary)
    }

    fn deserialize<'b, 'de, T>(bytes: &'b [u8]) -> Result<T, Self::DeserializeError>
    where
        'b: 'de,
        T: serde::Deserialize<'de>,
    {
        rmp_serde::from_slice(bytes)
    }
}

mod sealed {
    pub trait Sealed {}

    impl Sealed for String {}
    impl Sealed for Vec<u8> {}
}
