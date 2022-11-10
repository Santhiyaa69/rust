use actix_web::http::StatusCode;

use std::{error, fmt, result};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, Clone, )]
#[non_exhaustive]
pub enum ErrorKind {
    Internal,
    InvalidData,
    NotFound,
    BadInput(mongodb::bson::oid::Error),
    DatabaseError(mongodb::error::Error),
    DataParseError(mongodb::bson::ser::Error),
    DataIntergrityError(mongodb::bson::de::Error),
}
impl From<mongodb::bson::oid::Error> for ErrorKind {
    fn from(err: mongodb::bson::oid::Error) -> Self {
        ErrorKind::BadInput(err)
    }
}
impl From<mongodb::error::Error> for ErrorKind {
    fn from(err: mongodb::error::Error) -> Self {
        ErrorKind::DatabaseError(err)
    }
}
impl From<mongodb::bson::ser::Error> for ErrorKind {
    fn from(err: mongodb::bson::ser::Error) -> Self {
        ErrorKind::DataParseError(err)
    }
}
impl From<mongodb::bson::de::Error> for ErrorKind {
    fn from(err: mongodb::bson::de::Error) -> Self {
        ErrorKind::DataIntergrityError(err)
    }
}

#[derive(Debug, Clone)]
pub struct Error {
    msg: String,
    kind: ErrorKind,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ErrorKind::*;
        let error_str = match self.kind {
            Internal => "internal server error",
            InvalidData => "invalid data",
            NotFound => "not found",
            BadInput(_) => "bad input",
            DatabaseError(_) => "database error",
            DataParseError(_) => "data parse error",
            DataIntergrityError(_) => "data integrity error",
        };
        f.write_str(&error_str)
    }
}

impl Error {
    pub fn new(msg: impl Into<String>, kind: impl Into<ErrorKind>) -> Error {
        Error {
            msg: msg.into(),
            kind: kind.into(),
        }
    }
    pub fn msg(&self) -> String {
        self.msg.to_string()
    }
    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self.kind {
            ErrorKind::Internal => None,
            ErrorKind::InvalidData => None,
            ErrorKind::NotFound => None,
            ErrorKind::BadInput(err) => Some(err),
            ErrorKind::DatabaseError(err) => Some(err),
            ErrorKind::DataParseError(err) => Some(err),
            ErrorKind::DataIntergrityError(err) => Some(err),
        }
    }
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    status_code: u16,
    message: String,
    error: String
}
impl ErrorResponse {
    pub fn new(err: &Error, status_code: u16) -> Self {
        Self {
            status_code,
            message: err.msg.to_string(),
            error: err.to_string(),
        }
    }
}

impl actix_web::ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self.kind {
            ErrorKind::Internal => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorKind::InvalidData => StatusCode::BAD_REQUEST,
            ErrorKind::NotFound => StatusCode::NOT_FOUND,
            ErrorKind::BadInput(_) => StatusCode::BAD_REQUEST,
            ErrorKind::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorKind::DataParseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorKind::DataIntergrityError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        let status_code = self.status_code();
        let error_response = ErrorResponse::new(self,status_code.as_u16());
        actix_web::HttpResponse::build(status_code).json(error_response)
    }
}

pub trait Context<T> {
    fn context(self, msg: impl Into<String>) -> Result<T>;

    fn handover(self) -> Result<T>;

    fn with_context<F, S>(self, cb: F) -> Result<T>
        where
            F: Fn() -> S,
            S: Into<String>;
}

impl<T, E: Into<ErrorKind> + fmt::Display> Context<T> for result::Result<T, E> {
    fn context(self, msg: impl Into<String>) -> Result<T> {
        self.map_err(|err| Error::new(msg, err))
    }

    fn handover(self) -> Result<T> {
        self.map_err(|err| Error::new(err.to_string(), err))
    }

    fn with_context<F, S>(self, cb: F) -> Result<T>
        where
            F: Fn() -> S,
            S: Into<String>,
    {
        self.map_err(move |err| Error::new(cb(), err))
    }
}