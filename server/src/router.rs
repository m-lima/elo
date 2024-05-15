pub fn build() -> axum::Router {
    axum::Router::new().route("/", axum::routing::get(auth))
    // .route("/text", axum::routing::get(upgrade::<String>))
    // .route("/binary", axum::routing::get(upgrade::<Vec<u8>>))
}

async fn auth(axum::Extension(user_id): axum::Extension<types::Id>) -> String {
    format!("You are logged in with id: {user_id}")
}
