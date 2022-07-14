use crate::config::ORIGIN;
use axum::http::header::*;
use axum::http::Method;
use tower_http::cors::CorsLayer;

pub fn handle() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(ORIGIN.parse::<HeaderValue>().unwrap())
        .allow_methods([Method::DELETE, Method::GET, Method::OPTIONS, Method::POST])
        .allow_headers([
            ACCEPT,
            AUTHORIZATION,
            CONTENT_LENGTH,
            CONTENT_TYPE,
            CONTENT_DISPOSITION,
        ])
}
