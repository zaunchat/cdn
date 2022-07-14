use crate::{
    structures::*,
    utils::{result::*, s3, Tag},
};
use axum::{
    extract::{Extension, Path},
    http::header::*,
    response::IntoResponse,
};
use ormlite::{model::*, PgPool};

pub async fn download(
    Path((tag, id, filename)): Path<(String, i64, String)>,
    Extension(pool): Extension<PgPool>,
) -> Result<impl IntoResponse> {
    let tag = Tag::try_from(tag)?;

    let info = Attachment::select()
        .filter("id = $1 AND filename = $2 AND deleted = FALSE")
        .bind(id)
        .bind(filename)
        .fetch_one(&pool)
        .await?;

    let buffer = s3::get(&tag, id).await?;

    Ok((
        [
            (
                CONTENT_DISPOSITION,
                format!("attachment; filename=\"{}\"", info.filename),
            ),
            (CONTENT_TYPE, info.content_type.to_string()),
        ],
        buffer,
    ))
}
