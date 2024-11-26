use actix_web::{delete, get, post, put, web, HttpResponse, Responder, Result};

use diesel::prelude::*;

use crate::{models::{item::{Item, ItemDto}, response_data::{ResponseDataError, ResponseDataSuccess}}, schema::items, utils::{db::DbPool, errors::MyError}};

#[get("/api/items")]
async fn get_items(db_pool: web::Data<DbPool>) -> Result<HttpResponse, MyError>  {
    let mut conn = db_pool.get().expect("Couldn't get DB connection from pool");

    use crate::schema::items::dsl::*;
    let results = match items
        .select(Item::as_select())
        .load(&mut conn) {
            Ok(data) => data,
            Err(err) => {
                match err {
                    diesel::result::Error::NotFound => {
                        let notfound_res = ResponseDataSuccess::<String> { 
                            message: "tidak ada data di database".to_string(), 
                            data: None
                        };
                        return Ok(HttpResponse::Ok().json(notfound_res))
                    }
                    _ => {
                        return Err(MyError::InternalError );
                    }
                }
            },
        };

    let res = ResponseDataSuccess::<Vec<Item>> { 
        message: "get list items berhasil".to_string(), 
        data: Some(results)
    };
    Ok(HttpResponse::Ok().json(res))
}

#[get("/api/items/{id}")]
async fn get_item(db_pool: web::Data<DbPool>, path: web::Path<i32>) -> Result<HttpResponse, MyError>  {
    let mut conn = db_pool.get().expect("Couldn't get DB connection from pool");

    let id_item = path.into_inner();

    use crate::schema::items::dsl::*;

    let results = match items
        .filter(id.eq(id_item)) 
        .select(Item::as_select()) 
        .first(&mut conn) {
            Ok(data) => data,
            Err(err) => {
                match err {
                    diesel::result::Error::NotFound => {
                        let notfound_res = ResponseDataSuccess::<String> { 
                            message: "tidak ada data di database".to_string(), 
                            data: None
                        };
                        return Ok(HttpResponse::Ok().json(notfound_res))
                    }
                    _ => {
                        return Err(MyError::InternalError );
                    }
                }
            },
        };

    ResponseDataSuccess::<String> { 
        message: "tidak ada data di database".to_string(), 
        data: None
    };
    Ok(HttpResponse::Ok().json(results))
}

#[post("/api/items")]
async fn insert_item(db_pool: web::Data<DbPool>, body_data: web::Json<ItemDto>) -> Result<HttpResponse, MyError>  {
    let mut conn: r2d2::PooledConnection<diesel::r2d2::ConnectionManager<SqliteConnection>> = db_pool.get().expect("Couldn't get DB connection from pool");

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
    
    let _res =  match diesel::insert_into(items::table)
        .values(&new_item)
        .execute(&mut conn) {
            Ok(data) => data,
            Err(err) => {
                match err {
                    _ => {
                        return Err(MyError::InternalError );
                    }
                }
            },
        };

    return Ok(HttpResponse::Ok().json(ResponseDataSuccess::<usize> {
        message: "insert success".to_string(),
        data: None
    }));
}

#[put("/api/items/{id}")]
async fn update_item(db_pool: web::Data<DbPool>, path: web::Path<i32>, body_data: web::Json<ItemDto>) -> Result<HttpResponse, MyError>  {
    let mut conn: r2d2::PooledConnection<diesel::r2d2::ConnectionManager<SqliteConnection>> = db_pool.get().expect("Couldn't get DB connection from pool");

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

    let _ = match diesel::update(items.find(id_item))
        .set(name.eq(updated_name))
        .execute(&mut conn) {
            Ok(data) => data,
            Err(err) => {
                match err {
                    _ => {
                        return Err(MyError::InternalError);
                    }
                }
            },
        };

    return Ok(HttpResponse::Ok().json(ResponseDataSuccess::<usize> {
        message: "update success".to_string(),
        data: None
    }));
}

#[delete("/api/items/{id}")]
async fn delete_item(db_pool: web::Data<DbPool>, path: web::Path<i32>) -> Result<HttpResponse, MyError>  {
    let mut conn: r2d2::PooledConnection<diesel::r2d2::ConnectionManager<SqliteConnection>> = db_pool.get().expect("Couldn't get DB connection from pool");

    use crate::schema::items::dsl::*;
    
    let id_item = path.into_inner();

    let _res = match diesel::delete(items.filter(id.eq(id_item)))
        .execute(&mut conn) {
            Ok(data) => data,
            Err(err) => {
                match err {
                    _ => {
                        return Err(MyError::InternalError );
                    }
                }
            }
        };

    return Ok(HttpResponse::Ok().json(ResponseDataSuccess::<usize> {
        message: "delete success".to_string(),
        data: None
    }));
}