use crate::utils::ContentType;
use ormlite::model::*;
use serde::{Serialize, Serializer};

#[derive(Model, FromRow, Serialize, Debug)]
#[ormlite(table = "attachments")]
pub struct Attachment {
    #[serde(serialize_with = "bigint_to_string")]
    pub id: i64,
    pub filename: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    pub content_type: ContentType,
    pub size: i32,
    #[serde(skip)]
    pub deleted: bool,
}

pub fn bigint_to_string<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: std::fmt::Display,
    S: Serializer,
{
    serializer.collect_str(value)
}
