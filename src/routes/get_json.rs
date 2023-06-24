use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Data {
    message: String,
    count: i32,
    username: String,
}

pub async fn get_json() -> Json<Data> {
    Json(Data { message: "I'm in data!".to_owned(), count: 234, username: "bazuka".to_owned()}) 
}
