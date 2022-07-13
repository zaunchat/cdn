use ormlite::model::*;
use ormlite::types::Json;
use serde::{Deserialize, Serialize};

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
    pub id: i64,
    pub name: String,
    pub meta: Json<Metadata>,
    pub content_type: String,
    pub size: i32,
    pub tag: String,
}
