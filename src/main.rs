#[macro_use]
extern crate lazy_static;
extern crate s3;

use axum::{middleware, routing::*, Extension, Router, Server};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;

mod config;
mod middlewares;
mod routes;
mod structures;
mod utils;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&*config::DATABASE_URI)
        .await
        .expect("Couldn't connect to database");

    let app = Router::new()
        .route("/:tag", post(routes::upload::upload))
        .route("/:tag/:id/:filename", get(routes::download::download))
        .layer(middleware::from_fn(middlewares::auth::handle))
        .layer(Extension(pool))
        .layer(middlewares::cors::handle());

    let addr: SocketAddr = format!("0.0.0.0:{}", *config::PORT).parse().unwrap();

    println!("Listening on: {}", addr.port());

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
