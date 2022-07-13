use axum::{
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};
use quick_error::quick_error;
use serde::Serialize;

quick_error! {
    #[derive(Debug, Serialize)]
    pub enum Error {
         TooLarge { display("This file is too lar") }
         TypeNotAllowed
         UnknownTag
         NotFound
         MissingHeader
         InvalidToken
         S3Unavailable
         Database
         Unknown
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let status = match self {
            Error::NotFound => StatusCode::NOT_FOUND,
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
