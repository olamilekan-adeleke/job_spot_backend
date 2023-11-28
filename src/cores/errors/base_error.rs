use actix_web::{error, http::StatusCode, HttpResponse};
use serde::Serialize;
use sqlx::error::Error as SQLxError;
use std::fmt::Display;

#[derive(Debug, Serialize)]
pub enum BaseError {
    DBError(String),
    ActixServerError(String),
    NotFound(String),
    NahMeFuckUp(String),
    InvalidInput(String),
    InvalidBody(String),
    InvalidAccessToken(String),
    InternalServerError(String),
}

#[derive(Debug, Serialize)]
pub struct BaseErrorResponse {
    message: String,
    status_code: u16,
    state: String,
}

impl From<&BaseError> for BaseErrorResponse {
    fn from(error: &BaseError) -> Self {
        BaseErrorResponse {
            message: error.error_response(),
            status_code: error.status_code().as_u16(),
            state: "error".to_string(),
        }
    }
}

impl BaseError {
    fn error_response(&self) -> String {
        match self {
            BaseError::DBError(msg) => {
                tracing::error!("Database error occurred: {:?}", msg);
                msg.into()
            }
            BaseError::ActixServerError(msg) => {
                tracing::error!("Server error occurred: {:?}", msg);
                msg.into()
            }
            BaseError::NotFound(msg) => {
                tracing::error!("Not found error occurred: {:?}", msg);
                msg.into()
            }
            BaseError::NahMeFuckUp(msg) => {
                tracing::error!("Sorry, nah me code nonsense: {:?}", msg);
                msg.into()
            }
            BaseError::InvalidInput(msg) => {
                tracing::error!("Invalid parameters received: {:?}", msg);
                msg.into()
            }
            BaseError::InvalidBody(msg) => {
                tracing::error!("Invalid Request Body: {:?}", msg);
                msg.into()
            }
            BaseError::InvalidAccessToken(msg) => {
                tracing::error!("Invalid Access Token: {:?}", msg);
                msg.into()
            }
            BaseError::InternalServerError(msg) => {
                tracing::error!("Internal server error: {:?}", msg);
                msg.into()
            }
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            BaseError::DBError(_) | BaseError::ActixServerError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            BaseError::NahMeFuckUp(_) => StatusCode::INTERNAL_SERVER_ERROR,
            BaseError::NotFound(_) => StatusCode::NOT_FOUND,
            BaseError::InvalidInput(_) => StatusCode::BAD_REQUEST,
            BaseError::InvalidBody(_) => StatusCode::BAD_REQUEST,
            BaseError::InvalidAccessToken(_) => StatusCode::UNAUTHORIZED,
            BaseError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl error::ResponseError for BaseError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(BaseErrorResponse::from(self))
    }
}

impl Display for BaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error_response())
    }
}

impl From<actix_web::error::Error> for BaseError {
    fn from(error: actix_web::error::Error) -> Self {
        BaseError::ActixServerError(error.to_string())
    }
}

impl From<SQLxError> for BaseError {
    fn from(error: SQLxError) -> Self {
        BaseError::DBError(error.to_string())
    }
}

impl From<jwt_simple::Error> for BaseError {
    fn from(error: jwt_simple::Error) -> Self {
        BaseError::NahMeFuckUp(error.to_string())
    }
}
