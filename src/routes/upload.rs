use crate::{
    structures::*,
    utils::{result::*, s3, snowflake},
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
    let limit: usize = match tag.as_str() {
        "avatars" => 8_000_000,       // 8MB
        "icons" => 8_000_000,         // 8MB
        "backgrounds" => 16_000_000,  // 16MB
        "attachments" => 100_000_000, // 100MB
        _ => return Err(Error::UnknownTag),
    };

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

    // TODO:
    let metadata = match tree_magic::from_u8(&buffer).as_str() {
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

    let limited = match &metadata {
        Metadata::Text { .. } if size > 4_000_000 => 4_000_000,
        Metadata::Audio { .. } if size > 8_000_000 => 8_000_000,
        Metadata::Image { .. } if size > 16_000_000 => 16_000_000,
        Metadata::Video { .. } if size > 32_000_000 => 32_000_000,
        _ if size > 100_000_000 => 100_000_000,
        _ => 0,
    };

    if limited > 0 {
        return Err(Error::TooLarge);
    }

    let content_type = tree_magic::from_u8(&buffer);

    let file = Attachment {
        name,
        id: snowflake::generate(),
        tag: tag.clone(),
        content_type,
        meta: Json(metadata),
        size: size as i32,
    }
    .insert(&pool)
    .await
    .map_err(|_| Error::Database)?;

    s3::save(&tag, file.id, &buffer)
        .await
        .map_err(|_| Error::S3Unavailable)?;

    Ok(JsonRes(file))
}
