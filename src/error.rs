use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;
pub type Result<T> = std::result::Result<T, Error>;
#[derive(Error, Debug)]
pub enum Error {
    #[error("Sqlite error: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
    #[error("Json parsing error: {0}")]
    JsonParsing(#[from] serde_json::Error),
     #[error("chatgpt api client error: {0}")]
    ChatGptAPI(#[from] openai_chat::error::Errpr),
    // #[error("Zip parsing error: {0}")]
    // ZipParsing(#[from] zip::result::ZipError),
    #[error("Actix web error: {0}")]
    Actix(#[from] actix_web::Error),
    // #[cfg(feature = "tls")]
    // #[error("Rustls error: {0}")]
    // Rustls(#[from] rustls::Error),
        #[error("Value error: {0}")]
    ValueNotFound(String),
    // #[error("ParseConfig error: {0}")]
    // ParseConfig(String),
    // this will happen if the cliient has already been authenticated yet the server create
    // an equal username and ?
    #[error("ParseConfig error: {0}")]
    InvalidHostKey(String),
    #[error(transparent)]
    // UserError(#[from] crate::user::UserError),
    // #[error("Error while serializing data: {0}")]
    SerdeTomlSerializingError(#[from] toml::ser::Error),
    #[error("Error while deserializing data: {0}")]
    SerdeTomlDeserializingError(#[from] toml::de::Error),
    // #[error("Error while paring multipart stream: {0}")]
    // Multipart(#[from] actix_multipart::MultipartError),
    /// 500
    #[error("InternalServerError {0}")]
    InternalServerError(String),
    }

/// Actix Web uses `ResponseError` for conversion of errors to a response
impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match self {
            // Error::UserError(e) => {
            //     // found in anki/rslib/src/error/network.rs
            //     log::error!("{}", e.to_string());
            //     HttpResponse::Forbidden().finish()
            // }
            Error::InvalidHostKey(e) => {
                // found in anki/rslib/src/error/network.rs
                log::error!("{}", e.to_string());
                HttpResponse::Forbidden().finish()
            }
            e => {
                log::error!("{}", e.to_string());
                HttpResponse::InternalServerError().finish()
            }
        }
    }
}
