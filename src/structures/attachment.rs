use crate::utils::tag::Tag;
use ormlite::{model::*, types::Json};
use serde::{Deserialize, Serialize, Serializer};
use std::fmt::Display;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Metadata {
    Image {},
    Video {},
    Text {},
    Audio {},
    File {},
}

impl Metadata {
    pub fn accept(&self, size: usize) -> bool {
        let limited = match self {
            Metadata::Text { .. } if size > 4_000_000 => 4_000_000,
            Metadata::Audio { .. } if size > 8_000_000 => 8_000_000,
            Metadata::Image { .. } if size > 16_000_000 => 16_000_000,
            Metadata::Video { .. } if size > 32_000_000 => 32_000_000,
            _ if size > 100_000_000 => 100_000_000,
            _ => 0,
        };
        limited == 0
    }
}

#[derive(Model, FromRow, Serialize, Debug)]
#[ormlite(table = "attachments")]
pub struct Attachment {
    #[serde(serialize_with = "bigint_to_string")]
    pub id: i64,
    pub name: String,
    pub meta: Json<Metadata>,
    pub content_type: String,
    pub size: i32,
    pub tag: Tag,
}

pub fn bigint_to_string<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: Display,
    S: Serializer,
{
    serializer.collect_str(value)
}
