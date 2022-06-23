#[macro_use]
extern crate lazy_static;
extern crate s3;

use axum::{routing::post, Router, Server};
use std::net::SocketAddr;

mod config;
mod routes;
mod structures;
mod utils;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let app = Router::new().route("/:tag", post(routes::upload::upload));

    let addr: SocketAddr = format!("0.0.0.0:{}", *config::PORT).parse().unwrap();
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
