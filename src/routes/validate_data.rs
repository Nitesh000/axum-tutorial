use axum::Json;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RequestUser {
    username: Option<String>,
    password: String,
}

pub async fn validate_data(Json(user_data): Json<RequestUser>)  {
   dbg!(user_data); 
}
