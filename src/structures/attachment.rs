use ormlite::{model::*, types::Json};
use serde::{Deserialize, Serialize, Serializer};
use std::fmt::Display;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Metadata {
    Image {},
    Video {},
    Text {},
    Audio {},
    File {},
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
    pub tag: String,
}

pub fn bigint_to_string<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: Display,
    S: Serializer,
{
    serializer.collect_str(value)
}
