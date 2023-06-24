use axum::{TypedHeader, headers::UserAgent, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct UserResponse {
    user_agent: String,
}

pub async fn user_agent(TypedHeader(user_agent): TypedHeader<UserAgent>) -> Json<UserResponse> {
    Json(UserResponse { user_agent: user_agent.to_string()}) 
}
