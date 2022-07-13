use crate::structures::Metadata;
use crate::utils::Error;
use serde::Serialize;
use sqlx::{
    encode::IsNull,
    error::BoxDynError,
    postgres::{PgArgumentBuffer, PgTypeInfo, PgValueRef},
    Decode, Encode, Postgres, Type,
};
use std::fmt;

#[derive(Serialize, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Tag {
    Avatars,
    Backgrounds,
    Icons,
    Attachments,
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Attachments => "attachments",
                Self::Avatars => "avatars",
                Self::Backgrounds => "backgrounds",
                Self::Icons => "icons",
            }
        )
    }
}

impl Tag {
    pub fn limit(&self) -> usize {
        match self {
            Tag::Avatars => 8_000_000,       // 8MB
            Tag::Icons => 8_000_000,         // 8MB
            Tag::Backgrounds => 16_000_000,  // 16MB
            Tag::Attachments => 100_000_000, // 100MB
        }
    }

    pub fn accept(&self, meta: &Metadata) -> bool {
        if matches!(self, Tag::Avatars | Tag::Backgrounds | Tag::Icons) {
            return matches!(meta, &Metadata::Image { .. });
        }
        true
    }
}

impl Type<Postgres> for Tag {
    fn type_info() -> PgTypeInfo {
        String::type_info()
    }
}

impl Encode<'_, Postgres> for Tag {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> IsNull {
        Encode::<Postgres>::encode(self.to_string(), buf)
    }
}

impl<'r> Decode<'r, Postgres> for Tag {
    fn decode(value: PgValueRef<'r>) -> Result<Self, BoxDynError> {
        let s: String = Decode::<Postgres>::decode(value)?;
        Ok(Tag::try_from(s).unwrap())
    }
}

impl TryFrom<String> for Tag {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let tag = match value.as_str() {
            "avatars" => Self::Avatars,
            "backgrounds" => Self::Backgrounds,
            "icons" => Self::Icons,
            "attachments" => Self::Attachments,
            _ => return Err(Error::UnknownTag),
        };
        Ok(tag)
    }
}
