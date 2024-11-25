use actix_web::{get, post, web, HttpResponse, Responder, Result};

use diesel::prelude::*;

use crate::{models::{item::{Item, ItemDto}, response_data::{ResponseDataError, ResponseDataSuccess}}, schema::items, utils::db::establish_connection};

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

#[get("/api/items/{id}")]
async fn get_item(path: web::Path<i32>) -> Result<impl Responder>  {
    let id_item = path.into_inner();

    use crate::schema::items::dsl::*;

    let connection = &mut establish_connection();
    let results = items
        .filter(id.eq(id_item))
        .select(Item::as_select())
        .first(connection)
        .expect("Error loading posts");


    Ok(web::Json(results))
}

#[post("/api/items")]
async fn insert_item(body_data: web::Json<ItemDto>) -> Result<impl Responder>  {

    let name = body_data.name.clone().unwrap_or("".to_string());
    if name.is_empty() {
        let err = ResponseDataError {
            code: "empty_field".to_string(),
            message: "kolom nama perlu di isi".to_string()
        };
        return Ok(HttpResponse::BadRequest().json(err));
    }

    let new_item = ItemDto {
        name: body_data.name.clone()
    };

    let connection = &mut establish_connection();
    diesel::insert_into(items::table)
        .values(&new_item)
        .execute(connection)
        .expect("Error saving new post");

    return Ok(HttpResponse::Ok().json(ResponseDataSuccess::<usize> {
        message: "insert success".to_string(),
        data: None
    }));
}