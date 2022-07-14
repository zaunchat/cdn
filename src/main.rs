#[macro_use]
extern crate lazy_static;
extern crate s3;

use axum::{routing::*, Extension, Router, Server};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::{task, time};

mod config;
mod middlewares;
mod routes;
mod structures;
mod utils;

static FIVE_HOURS_IN_SECONDS: u64 = 18000;

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
        .layer(Extension(pool.clone()))
        .layer(middlewares::cors::handle());

    let deletion_task = task::spawn(async move {
        use structures::Attachment;
        use utils::{s3, Tag};
        println!("Started auto deletion task... ");

        let tags = [Tag::Attachments, Tag::Avatars, Tag::Icons, Tag::Backgrounds];

        loop {
            let attachments = sqlx::query_as::<_, Attachment>(
                "DELETE FROM attachments WHERE deleted = TRUE RETURNING *",
            )
            .fetch_all(&pool)
            .await
            .unwrap_or_else(|e| {
                println!("Couldn't fetch deleted attachments {}", e);
                vec![]
            });

            for attachment in attachments {
                'inner: for tag in tags {
                    if s3::delete(&tag, attachment.id).await.is_ok() {
                        break 'inner;
                    }
                }
            }

            time::sleep(Duration::from_secs(FIVE_HOURS_IN_SECONDS)).await;
        }
    });

    let addr: SocketAddr = format!("0.0.0.0:{}", *config::PORT).parse().unwrap();

    println!("Listening on: {}", addr.port());

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    if let Err(err) = deletion_task.await {
        panic!("Auto deletion task exited with error: {}", err);
    }
}
