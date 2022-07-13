use crate::{
    structures::*,
    utils::{s3, snowflake, Error, Result, Tag},
};
use axum::extract::{ContentLengthLimit, Extension, Json as JsonRes, Multipart, Path};
use content_inspector::inspect;
use futures::StreamExt;
use ormlite::{model::*, types::Json, PgPool};

pub async fn upload(
    Path(tag): Path<String>,
    ContentLengthLimit(mut multipart): ContentLengthLimit<
        Multipart,
        {
            100 * 1024 * 1024 // 100MB
        },
    >,
    Extension(pool): Extension<PgPool>,
) -> Result<JsonRes<Attachment>> {
    let tag = Tag::try_from(tag)?;
    let limit = tag.limit();
    let mut size: usize = 0;
    let mut buffer: Vec<u8> = Vec::new();

    let name = if let Ok(Some(mut field)) = multipart.next_field().await {
        let filename = field.file_name().ok_or(Error::Unknown)?.to_string();

        while let Some(Ok(chunk)) = field.next().await {
            size += chunk.len();

            if size > limit {
                Err(Error::TooLarge)?;
            }

            buffer.append(&mut chunk.to_vec());
        }

        filename
    } else {
        return Err(Error::Unknown);
    };

    let content_type = tree_magic::from_u8(&buffer);

    // TODO:
    let metadata = match content_type.as_str() {
        "image/jpeg" | "image/png" | "image/gif" | "image/webp" => Metadata::Image {},
        "video/mp4" | "video/webm" => Metadata::Video {},
        "audio/mpeg" => Metadata::Audio {},
        _ => {
            if inspect(&buffer).is_text() {
                Metadata::Text {}
            } else {
                Metadata::File {}
            }
        }
    };

    if !metadata.accept(size) {
        return Err(Error::TooLarge);
    }

    if !tag.accept(&metadata) {
        return Err(Error::TypeNotAllowed);
    }

    let mut tx = pool.begin().await?;

    let file = Attachment {
        id: snowflake::generate(),
        name,
        content_type,
        meta: Json(metadata),
        size: size as i32,
        tag,
    }
    .insert(&mut tx)
    .await?;

    s3::save(&tag, file.id, &buffer).await?;

    tx.commit().await?;

    Ok(file.into())
}
