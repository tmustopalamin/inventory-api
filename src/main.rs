use actix_web::{error, get, http::{header::ContentType, StatusCode}, web, App, HttpResponse, HttpServer, Responder, Result};
use derive_more::derive::{Display, Error};

use models::response_data::{ResponseDataError, ResponseDataSuccess};

pub mod schema;
pub mod models;
pub mod routes;
pub mod utils;

#[derive(Debug, Display, Error)]
enum MyError {
    InternalError,    
    Timeout,

    #[display("not found")]
    NotFound { field: String, value: String },

    #[display("bad client data")]
    BadClientData { field: String, value: String },
}

impl error::ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        let err_response = match self {
            MyError::InternalError => ResponseDataError {
                code: "500".to_string(),
                message: "terjadi kesalahan internal".to_string(),
            },
            MyError::BadClientData { field, value } => ResponseDataError {
                code: "400".to_string(),
                message: format!("field={} dan value={}", field, value),
            },
            MyError::Timeout => ResponseDataError {
                code: "504".to_string(),
                message: "waktu habis".to_string(),
            },
            MyError::NotFound { field, value } => ResponseDataError {
                code: "404".to_string(),
                message: format!("field={} dan value={} tidak ditemukan", field, value),
            },
        };

        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(err_response)
    }

    fn status_code(&self) -> StatusCode {
        match self {
            MyError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::BadClientData{ .. } => StatusCode::BAD_REQUEST,
            MyError::Timeout => StatusCode::GATEWAY_TIMEOUT,
            MyError::NotFound{ .. } => StatusCode::NOT_FOUND,
        }
    }
}

#[get("/api/health_check")]
async fn hello() -> Result<impl Responder>  {
    Ok(web::Json(ResponseDataSuccess::<String> {
        message: String::from("okay"),
        data: None
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(routes::item_route::get_items)
            .service(routes::item_route::get_item)
            .service(routes::item_route::insert_item)
            .service(routes::item_route::update_item)
    }).bind(("127.0.0.1", 3030))?
    .run()
    .await

}
