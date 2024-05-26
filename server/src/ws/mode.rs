pub trait Mode {
    fn mode() -> &'static str;
    fn serialize<T, R>(payload: T) -> Result<R, ()>;
    fn deserialize<T, R>(payload: T) -> Result<R, ()>;
}

impl Mode for String {
    fn mode() -> &'static str {
        "text"
    }

    fn serialize<T, R>(payload: T) -> Result<R, ()> {
        todo!()
    }

    fn deserialize<T, R>(payload: T) -> Result<R, ()> {
        todo!()
    }
}

impl Mode for Vec<u8> {
    fn mode() -> &'static str {
        "binary"
    }

    fn serialize<T, R>(payload: T) -> Result<R, ()> {
        todo!()
    }

    fn deserialize<T, R>(payload: T) -> Result<R, ()> {
        todo!()
    }
}
