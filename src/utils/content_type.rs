use serde::Serialize;
use sqlx::{
    encode::IsNull,
    error::BoxDynError,
    postgres::{PgArgumentBuffer, PgTypeInfo, PgValueRef},
    Decode, Encode, Postgres, Type,
};
use std::fmt;

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum ContentType {
    Image(String),
    Video(String),
    Audio(String),
    Text(String),
    File(String),
}

impl fmt::Display for ContentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ContentType::Audio(s)
                | ContentType::File(s)
                | ContentType::Image(s)
                | ContentType::Video(s)
                | ContentType::Text(s) => s,
            }
        )
    }
}

impl From<String> for ContentType {
    fn from(t: String) -> Self {
        match t.as_str() {
            "image/jpeg" | "image/png" | "image/gif" | "image/webp" => Self::Image(t),
            "video/mp4" | "video/quicktime" | "video/webm" => Self::Video(t),
            "audio/mpeg" => Self::Audio(t),
            _ if t.starts_with("text/") => Self::Text(t),
            _ => Self::File(t),
        }
    }
}

impl ContentType {
    pub fn is_allowed_size(&self, size: usize) -> bool {
        let limited = match self {
            Self::Text(_) if size > 4_000_000 => 4_000_000, // 4MB
            Self::Audio(_) if size > 8_000_000 => 8_000_000, // 8MB
            Self::Image(_) if size > 16_000_000 => 16_000_000, // 16MB
            Self::Video(_) if size > 32_000_000 => 32_000_000, // 32MB
            _ if size > 100_000_000 => 100_000_000,         // 100MB
            _ => 0,
        };

        limited == 0
    }
}

impl Type<Postgres> for ContentType {
    fn type_info() -> PgTypeInfo {
        String::type_info()
    }

    fn compatible(ty: &PgTypeInfo) -> bool {
        String::compatible(ty)
    }
}

impl Encode<'_, Postgres> for ContentType {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> IsNull {
        Encode::<Postgres>::encode(self.to_string(), buf)
    }
}

impl<'r> Decode<'r, Postgres> for ContentType {
    fn decode(value: PgValueRef<'r>) -> Result<Self, BoxDynError> {
        let s: String = Decode::<Postgres>::decode(value)?;
        Ok(s.into())
    }
}
