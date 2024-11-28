use actix_web::{delete, get, post, put, web, HttpResponse, Result};

use diesel::prelude::*;

use crate::{models::{item::{Item, ItemDto}, response_data::ResponseDataSuccess}, utils::{db::DbPool, errors::MyError}};

#[get("/api/items")]
async fn get_all_item_route(db_pool: web::Data<DbPool>) -> Result<HttpResponse, MyError>  {

    use crate::services::item_service::ItemService;
    let list_items = ItemService::get_all_items(db_pool).await?;

    let res = ResponseDataSuccess::<Vec<Item>> { 
        message: "".to_string(), 
        data: Some(list_items)
    };
    Ok(HttpResponse::Ok().json(res))
}

#[get("/api/items/{id}")]
async fn get_item_route(db_pool: web::Data<DbPool>, path: web::Path<i32>) -> Result<HttpResponse, MyError>  {
    let id_item = path.into_inner();

    use crate::services::item_service::ItemService;
    let item = ItemService::get_item(db_pool, id_item).await?;
    
    let res = ResponseDataSuccess::<Item> { 
        message: "".to_string(), 
        data: Some(item)
    };
    Ok(HttpResponse::Ok().json(res))
}

#[post("/api/items")]
async fn insert_item_route(db_pool: web::Data<DbPool>, body_data: web::Json<ItemDto>) -> Result<HttpResponse, MyError>  {
    let item_dto = body_data.0;

    use crate::services::item_service::ItemService;
    let _ = ItemService::insert_item(db_pool, item_dto).await?;

    return Ok(HttpResponse::Ok().json(ResponseDataSuccess::<usize> {
        message: "insert success".to_string(),
        data: None
    }));
}

#[put("/api/items/{id}")]
async fn update_item_route(db_pool: web::Data<DbPool>, path: web::Path<i32>, body_data: web::Json<ItemDto>) -> Result<HttpResponse, MyError>  {
    let id_item = path.into_inner();
    let item_dto = body_data.0;

    use crate::services::item_service::ItemService;
    let _ = ItemService::update_item(db_pool, id_item, item_dto).await?;

    return Ok(HttpResponse::Ok().json(ResponseDataSuccess::<usize> {
        message: "update success".to_string(),
        data: None //0 / 1 adalah number affected
    }));
}

#[delete("/api/items/{id}")]
async fn delete_item_route(db_pool: web::Data<DbPool>, path: web::Path<i32>) -> Result<HttpResponse, MyError>  {
    let id_item = path.into_inner();
    
    use crate::schema::items::dsl::*;

    let results_async = web::block(move ||  {
        let mut conn = db_pool.get().expect("error getting db connection");

        diesel::delete(items.filter(id.eq(id_item))).execute(&mut conn)
    }).await;

    let _results = match results_async {
        Ok(Ok(res)) => res,
        Ok(Err(err)) => {
            match err {
                diesel::result::Error::NotFound => {
                    let notfound_res = ResponseDataSuccess::<String> { 
                        message: "tidak ada data di database".to_string(), 
                        data: None
                    };
                    return Ok(HttpResponse::Ok().json(notfound_res))
                },
                _ => {
                    return Err(MyError::InternalError);
                }
            }
        },
        _ => {
            return Err(MyError::InternalError);
        }
    };

    return Ok(HttpResponse::Ok().json(ResponseDataSuccess::<usize> {
        message: "delete success".to_string(),
        data: None
    }));
}