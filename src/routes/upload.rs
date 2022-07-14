use crate::{
    structures::*,
    utils::{s3, snowflake, ContentType, Error, Result, Tag},
};
use axum::extract::*;
use futures::StreamExt;
use ormlite::{model::*, PgPool};

pub async fn upload(
    Path(tag): Path<String>,
    ContentLengthLimit(mut multipart): ContentLengthLimit<
        Multipart,
        {
            100 * 1024 * 1024 // 100MB
        },
    >,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Attachment>> {
    let tag = Tag::try_from(tag)?;
    let limit = tag.limit();

    let mut buffer: Vec<u8> = Vec::new();
    let mut size: usize = 0;
    let mut height: Option<i32> = None;
    let mut width: Option<i32> = None;

    let filename = if let Ok(Some(mut field)) = multipart.next_field().await {
        let name = field.file_name().ok_or(Error::CannotProcess)?.to_string();

        while let Some(Ok(chunk)) = field.next().await {
            size += chunk.len();

            if size > limit {
                Err(Error::TooLarge)?;
            }

            buffer.append(&mut chunk.to_vec());
        }

        name
    } else {
        return Err(Error::Unknown);
    };

    let content_type = tree_magic::from_u8(&buffer).into();

    if let ContentType::Image(_) = content_type {
        let size = imagesize::blob_size(&buffer).map_err(|_| Error::CannotProcess)?;
        height = Some(size.height.try_into().map_err(|_| Error::CannotProcess)?);
        width = Some(size.width.try_into().map_err(|_| Error::CannotProcess)?);
    }

    if let ContentType::Video(_) = content_type {
        // TODO: Extract height and width
    }

    if !content_type.is_allowed_size(size) {
        return Err(Error::TooLarge);
    }

    if !tag.accept(&content_type) {
        return Err(Error::TypeNotAllowed);
    }

    let mut tx = pool.begin().await?;

    let file = Attachment {
        id: snowflake::generate(),
        filename,
        content_type,
        height,
        width,
        size: size as i32,
        deleted: false,
    }
    .insert(&mut tx)
    .await?;

    s3::save(&tag, file.id, &buffer).await?;

    tx.commit().await?;

    Ok(file.into())
}
