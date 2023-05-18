use actix_web::error::BlockingError;
use actix_web::http::StatusCode;
use scraper::error::SelectorErrorKind;
use std::fmt;
use std::num::ParseIntError;

#[derive(Clone, Debug)]
pub enum AppError {
    InternalServerError(String),
    CrosswordNotFound(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::InternalServerError(message) => {
                write!(f, "Something went wrong: {}", message)
            }
            AppError::CrosswordNotFound(id) => {
                write!(f, "Could not find crossword for id: {}", id)
            }
        }
    }
}

pub fn to_status_code(error: AppError) -> StatusCode {
    match error {
        AppError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        AppError::CrosswordNotFound(_) => StatusCode::NOT_FOUND,
    }
}

impl From<BlockingError> for AppError {
    fn from(error: BlockingError) -> Self {
        AppError::InternalServerError(error.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(error: serde_json::Error) -> Self {
        AppError::InternalServerError(error.to_string())
    }
}

impl From<r2d2::Error> for AppError {
    fn from(error: r2d2::Error) -> Self {
        AppError::InternalServerError(error.to_string())
    }
}

impl From<reqwest::Error> for AppError {
    fn from(error: reqwest::Error) -> Self {
        AppError::InternalServerError(error.to_string())
    }
}

impl From<SelectorErrorKind<'_>> for AppError {
    fn from(error: SelectorErrorKind) -> Self {
        AppError::InternalServerError(format!("Invalid selector: {}", error.to_string()))
    }
}

impl From<String> for AppError {
    fn from(error: String) -> Self {
        AppError::InternalServerError(error)
    }
}

impl From<ParseIntError> for AppError {
    fn from(error: ParseIntError) -> Self {
        AppError::InternalServerError(error.to_string())
    }
}
