pub trait Mode: sealed::Mode + Send + Sync + 'static {}

impl Mode for String {}
impl Mode for Vec<u8> {}

pub(crate) mod sealed {
    pub trait Mode {
        type SerializeError: std::fmt::Display + std::fmt::Debug + Send;
        type DeserializeError: std::fmt::Display + std::fmt::Debug + Send;

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
            rmp_serde::to_vec_named(&payload).map(axum::extract::ws::Message::Binary)
        }

        fn deserialize<'b, 'de, T>(bytes: &'b [u8]) -> Result<T, Self::DeserializeError>
        where
            'b: 'de,
            T: serde::Deserialize<'de>,
        {
            rmp_serde::from_slice(bytes)
        }
    }
}
