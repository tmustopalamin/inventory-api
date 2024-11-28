use actix_web::{get, web, App, HttpServer, Responder, Result};

use models::response_data::ResponseDataSuccess;
use utils::db::establish_connection_pool;

pub mod schema;
pub mod models;
pub mod routes;
pub mod repositories;
pub mod services;
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
    let db_pool = establish_connection_pool();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .service(hello)
            .service(routes::item_route::get_all_item_route)
            .service(routes::item_route::get_item_route)
            .service(routes::item_route::insert_item_route)
            .service(routes::item_route::update_item_route)
            .service(routes::item_route::delete_item_route)
    }).bind(("127.0.0.1", 3030))?
    .run()
    .await

}
