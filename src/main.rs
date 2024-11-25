use actix_web::{get, web, App, HttpServer, Responder, Result};
use models::response_data::ResponseDataSuccess;


pub mod schema;
pub mod models;
pub mod routes;
pub mod utils;

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
    }).bind(("127.0.0.1", 3030))?
    .run()
    .await

}
