use axum::http::HeaderMap;

pub async fn custom_header(headers: HeaderMap) -> String {
    let message = headers.get("x-message").unwrap(); 
    message.to_str().unwrap().to_string()
}
