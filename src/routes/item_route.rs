use actix_web::{get, post, put, web, HttpResponse, Responder, Result};

use diesel::prelude::*;

use crate::{models::{item::{Item, ItemDto}, response_data::{ResponseDataError, ResponseDataSuccess}}, schema::items, utils::db::establish_connection, MyError};

#[get("/api/items")]
async fn get_items() -> Result<HttpResponse>  {
    
    use crate::schema::items::dsl::*;

    let connection = &mut establish_connection();
    let results = items
        .select(Item::as_select())
        .load(connection)
        .expect("Error loading posts");


    Ok(HttpResponse::Ok().json(results))
}

#[get("/api/items/{id}")]
async fn get_item(path: web::Path<i32>) -> Result<HttpResponse, MyError>  {
    let id_item = path.into_inner();

    use crate::schema::items::dsl::*;

    let connection = &mut establish_connection();
    let results = match items
        .filter(id.eq(id_item)) 
        .select(Item::as_select()) 
        .first(connection) {
            Ok(data) => data,
            Err(err) => {
                match err {
                    diesel::result::Error::NotFound => {
                        return Err(MyError::NotFound { field: "id".to_string(), value: id_item.to_string() } );
                    }
                    _ => {
                        return Err(MyError::InternalError );
                    }
                }
            },
        };
        
    Ok(HttpResponse::Ok().json(results))
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

#[put("/api/items/{id}")]
async fn update_item(path: web::Path<i32>, body_data: web::Json<ItemDto>) -> Result<impl Responder>  {
    use crate::schema::items::dsl::*;
    
    let id_item = path.into_inner();

    let updated_name = body_data.name.clone().unwrap_or("".to_string());
    if updated_name.is_empty() {
        let err = ResponseDataError {
            code: "empty_field".to_string(),
            message: "kolom nama perlu di isi".to_string()
        };
        return Ok(HttpResponse::BadRequest().json(err));
    }

    let connection = &mut establish_connection();
    diesel::update(items.find(id_item))
        .set(name.eq(updated_name))
        .execute(connection)
        .expect("Error saving new post");

    return Ok(HttpResponse::Ok().json(ResponseDataSuccess::<usize> {
        message: "update success".to_string(),
        data: None
    }));
}