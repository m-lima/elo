pub fn build() -> axum::Router {
    axum::Router::new()
        .route("/text", axum::routing::get(upgrade::<String>))
        .route("/binary", axum::routing::get(upgrade::<Vec<u8>>))
}

async fn upgrade<M>(upgrade: axum::extract::WebSocketUpgrade)
where
    M: Mode,
{
}

trait Mode {}

impl Mode for String {}
impl Mode for Vec<u8> {}
