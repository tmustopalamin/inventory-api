use actix_web::{get, web, Responder, Result};

use crate::{establish_connection, models::item::Item};
use diesel::prelude::*;

#[get("/api/items")]
async fn get_items() -> Result<impl Responder>  {
    
    use crate::schema::items::dsl::*;

    let connection = &mut establish_connection();
    let results = items
        .select(Item::as_select())
        .load(connection)
        .expect("Error loading posts");


    Ok(web::Json(results))
}