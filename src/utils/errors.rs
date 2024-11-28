use actix_web::{error, http::{header::ContentType, StatusCode}, HttpResponse};
use derive_more::derive::{Display, Error};

use crate::models::response_data::ResponseDataError;

#[derive(Debug, Display, Error)]
pub enum MyError {
    InternalError,    
    Timeout,
    DbConnectionError,

    #[display("not found")]
    NotFound { msg: String },

    #[display("bad client data")]
    BadClientData { msg: String },
}

impl error::ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        let err_response = match self {
            MyError::InternalError => ResponseDataError {
                code: "500".to_string(),
                message: "terjadi kesalahan internal".to_string(),
            },
            MyError::BadClientData { msg } => ResponseDataError {
                code: "400".to_string(),
                message: format!("{}", msg),
            },
            MyError::Timeout => ResponseDataError {
                code: "504".to_string(),
                message: "waktu habis".to_string(),
            },
            MyError::NotFound { msg } => ResponseDataError {
                code: "404".to_string(),
                message: format!("{}", msg),
            },
            MyError::DbConnectionError => ResponseDataError {
                code: "500".to_string(),
                message: "terjadi kesalahan internal".to_string(),
            },
        };

        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(err_response)
    }

    fn status_code(&self) -> StatusCode {
        match self {
            MyError::DbConnectionError => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::BadClientData{ .. } => StatusCode::BAD_REQUEST,
            MyError::Timeout => StatusCode::GATEWAY_TIMEOUT,
            MyError::NotFound{ .. } => StatusCode::NOT_FOUND,
        }
    }
}