use axum::{http::{Request, StatusCode}, middleware::Next, response::Response};

use crate::routes::read_middleware_custom_header::HeaderMessage;

pub async fn set_middleware_custom_header<B>(mut req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    println!("--> MIDDLEWARE Called");
    let headers = req.headers();
    let message = headers.get("message").ok_or_else(|| StatusCode::BAD_REQUEST)?;
    let message = message.to_str().map_err(|_error| {
        StatusCode::BAD_REQUEST
    })?.to_owned();
let extensions = req.extensions_mut();
    extensions.insert(HeaderMessage(message));
   Ok(next.run(req).await) 
}

#[allow(unused)]
pub async fn show_authentication<B>(req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    let headers = req.headers();
    let auth_token = headers.get("Authorization");
    match auth_token {
        Some(_val) => {
            println!("--> MIDDLEWARE Authorization");
            return Ok(next.run(req).await);
        },
            None => {
            println!("--> MIDDLEWARE Non Authorization Person");
            return Err(StatusCode::NON_AUTHORITATIVE_INFORMATION)},
    }
}
