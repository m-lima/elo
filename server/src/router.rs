pub fn build() -> axum::Router {
    axum::Router::new()
        .route("/ws/text", axum::routing::get(upgrade::<String>))
        .route("/ws/binary", axum::routing::get(upgrade::<Vec<u8>>))
}

trait Mode {}
impl Mode for String {}
impl Mode for Vec<u8> {}

#[tracing::instrument]
async fn upgrade<M: Mode>(
    // upgrade: axum::extract::WebSocketUpgrade,
    axum::Extension(user_id): axum::Extension<types::Id>,
) -> String {
    format!("You are logged in with id: {user_id}")
}
