pub enum Metadata {
    Image {},
    Video {},
    Text {},
    Audio {},
    File {},
}

pub struct Attachment {
    pub name: String,
    pub id: u64,
    pub uploader: u64,
    pub meta: Metadata,
    pub size: u32,
    pub tag: String,
}
