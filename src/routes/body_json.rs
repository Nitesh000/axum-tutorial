use axum::Json;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MirrofJson {
    message: String,
}

#[derive(Serialize)]
pub struct JsonResponse {
    message: String,
    message_from_server: String,
}

pub async fn body_json(Json(body): Json<MirrofJson>) -> Json<JsonResponse> {
    Json(JsonResponse {
        message: body.message,
        message_from_server: "Hello from axum server".to_owned()
    })
}

/*
 if we want to send the same body which came from the client.
 
pub async fn body_json(Json(body): Json<MirrofJson>) -> Json<MirrofJson> {
    Json(body)
}
*/
