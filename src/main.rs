use std::env;

use actix_web::{get, web, App, HttpServer, Responder, Result};
use diesel::{Connection, SqliteConnection};
use dotenvy::dotenv;
use models::response_data::ResponseData;

pub mod schema;
pub mod models;
pub mod routes;

#[get("/api/health_check")]
async fn hello() -> Result<impl Responder>  {
    Ok(web::Json(ResponseData {message: String::from("okay")}))
}

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(routes::item_route::get_items)
    }).bind(("127.0.0.1", 3030))?
    .run()
    .await

}
