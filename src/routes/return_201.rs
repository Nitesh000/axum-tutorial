use axum::{http::StatusCode, response::{Response, IntoResponse}};

pub async fn returns_201() -> Response {
    (
        StatusCode::CREATED,
        "This is a 201, which shows it is created".to_owned()
    ).into_response() 
}
