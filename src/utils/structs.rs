use actix_web::{HttpResponse, ResponseError};
use log::SetLoggerError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WesterError {
    #[error("I/O error: {0}")]
    Io(#[from] tokio::io::Error),
    #[error("Serde Json Error: {0}")]
    SerdeJson(#[from] serde_json::Error),
    #[error("Actix Web Error: {0}")]
    ActixUrl(#[from] actix_web::Error),
    #[error("Set Logger Error: {0}")]
    SetLoggerError(#[from] SetLoggerError),
    // #[error("Error: {0}")]
    // Other(String),
}

/// A `Result` alias where the `Err` case is `ClodociError`.
pub type Result<T> = std::result::Result<T, WesterError>;

// Implement the ResponseError trait for WesterError
impl ResponseError for WesterError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            WesterError::Io(ref err) => {
                HttpResponse::InternalServerError().body(format!("I/O error: {}", err))
            }
            WesterError::SerdeJson(ref err) => {
                HttpResponse::BadRequest().body(format!("Serde Json Error: {}", err))
            }
            WesterError::ActixUrl(ref err) => {
                HttpResponse::InternalServerError().body(format!("Actix Web Error: {}", err))
            }
            WesterError::SetLoggerError(ref err) => {
                HttpResponse::InternalServerError().body(format!("Set Logger Error: {}", err))
            } // WesterError::Other(ref err) => HttpResponse::InternalServerError().body(err.clone()),
        }
    }
}
