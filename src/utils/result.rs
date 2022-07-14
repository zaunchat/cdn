use axum::{
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};
use quick_error::quick_error;
use serde::Serialize;

quick_error! {
    #[derive(Debug, Serialize)]
    pub enum Error {
         TooLarge { display("This file is too large") }
         TypeNotAllowed { display("This file format isn't allowed for this tag") }
         CannotProcess { display("Couldn't process this file") }
         UnknownTag
         NotFound
         MissingHeader
         InvalidToken { display("The token you provided is invalid") }
         S3Unavailable { display("S3 Storage Service is currently unavailable") }
         Database { display("Database cannot process this operation currently") }
         Unknown { display("Unknown error has occurred") }
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

impl From<s3::error::S3Error> for Error {
    fn from(err: s3::error::S3Error) -> Self {
        println!("S3 Error: {:?}", err);
        Self::S3Unavailable
    }
}

impl From<ormlite::Error> for Error {
    fn from(err: ormlite::Error) -> Self {
        println!("DB Error: {:?}", err);
        Self::Database
    }
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        println!("DB Error: {:?}", err);
        Self::Database
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let status = match self {
            Error::NotFound => StatusCode::NOT_FOUND,
            Error::InvalidToken => StatusCode::UNAUTHORIZED,
            Error::MissingHeader => StatusCode::UNAUTHORIZED,
            Error::S3Unavailable => StatusCode::SERVICE_UNAVAILABLE,
            Error::Database => StatusCode::SERVICE_UNAVAILABLE,
            Error::TooLarge => StatusCode::PAYLOAD_TOO_LARGE,
            Error::UnknownTag => StatusCode::FORBIDDEN,
            _ => StatusCode::BAD_REQUEST,
        };

        let mut body = serde_json::json!({ "type": self });
        let msg = self.to_string();

        if msg.contains(' ') {
            body["message"] = serde_json::json!(msg);
        }

        (status, Json(body)).into_response()
    }
}
