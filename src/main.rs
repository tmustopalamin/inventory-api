use actix_web::{get, web, App, HttpServer, Responder, Result};
use models::response_data::ResponseData;

pub mod models;

#[get("/api/health_check")]
async fn hello() -> Result<impl Responder>  {
    Ok(web::Json(ResponseData {message: String::from("okay")}))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
    }).bind(("127.0.0.1", 3030))?
    .run()
    .await

}
