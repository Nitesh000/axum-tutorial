mod always_error;
mod body_json;
mod body_string;
mod custom_headers;
mod get_json;
mod hello_world;
mod middleware_message;
mod path_variables;
mod query_params;
mod read_middleware_custom_header;
mod return_201;
mod set_middleare_custom_header;
mod user_agent;
mod validate_data;

use axum::{
    http::Method,
    middleware,
    routing::{get, post},
    Extension, Router,
};
use body_json::body_json;
use body_string::body_strng;
use custom_headers::custom_header;
use hello_world::hello_world;
use middleware_message::middleware_message;
use path_variables::path_variables;
use query_params::query_params;
use tower_http::cors::{Any, CorsLayer};
use user_agent::user_agent;

use self::{
    always_error::always_error, get_json::get_json, path_variables::hard_coded_path,
    read_middleware_custom_header::read_middleware_custom_header, return_201::returns_201,
    set_middleare_custom_header::set_middleware_custom_header, validate_data::validate_data,
};

#[derive(Clone)]
pub struct SharedData {
    pub message: String,
}

pub fn create_routes() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let shared_data = SharedData {
        message: "Hello wrold from shared data".to_owned(),
    };

    Router::new()
        .route(
            "/read_middleware_custom_header",
            get(read_middleware_custom_header),
        )
        .route_layer(middleware::from_fn(set_middleware_custom_header))
        .route("/", get(hello_world))
        .route("/body_string", post(body_strng))
        .route("/body_json", post(body_json))
        .route("/path_variables/15", get(hard_coded_path))
        .route("/path_variables/:id", get(path_variables))
        .route("/query_params", get(query_params))
        .route("/user_agent", get(user_agent))
        .route("/custom_header", get(custom_header))
        .route("/middleware_message", get(middleware_message))
        .layer(cors)
        .layer(Extension(shared_data))
        .route("/always_error", get(always_error))
        .route("/returns_201", post(returns_201))
        .route("/get_json", get(get_json))
        .route("/validate_data", post(validate_data))
}
