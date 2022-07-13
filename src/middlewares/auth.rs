use crate::utils::result::Error;
use axum::{
    http::{header, Request},
    middleware::Next,
    response::Response,
};
use ormlite::PgPool;

pub async fn handle<B>(req: Request<B>, next: Next<B>) -> Result<Response, Error> {
    let token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    if token.is_none() {
        return Err(Error::MissingHeader);
    }

    let pool = req.extensions().get::<PgPool>().unwrap();

    let exists = sqlx::query("SELECT COUNT(*) FROM sessions WHERE token = $1")
        .bind(token.unwrap())
        .execute(pool)
        .await
        .map_err(|_| Error::Database)?;

    if exists.rows_affected() == 0 {
        return Err(Error::InvalidToken);
    }

    Ok(next.run(req).await)
}
