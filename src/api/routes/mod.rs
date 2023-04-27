use axum::{http::StatusCode, Json};
use serde::Serialize;


#[derive(Serialize)]
pub struct Return {
    message: String
}

pub async fn root() -> (StatusCode, Json<Return>) {
    (StatusCode::OK, Json(Return{
        message: "Hewo world! uwu <3".into()
    }))
}