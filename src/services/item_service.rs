use actix_web::web;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};

use crate::{models::item::{Item, ItemDto}, repositories::item_repository::get_all, utils::{db::DbPool, errors::MyError}};

pub struct ItemService {}

impl ItemService {
    pub async fn get_all_items(db_pool: web::Data<DbPool>) -> Result<Vec<Item>, MyError> {
        let results_async = web::block(move ||  {
            let mut conn = db_pool.get().expect("error getting db connection");
    
            get_all(&mut conn)
        }).await;

        let results = match results_async {
            Ok(Ok(items)) => items,
            Ok(Err(err)) => {
                match err {
                    diesel::result::Error::NotFound => {
                        return Err(MyError::NotFound { msg: "data tidak ditemukan".to_string() });
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

        return Ok(results)
    }
    
    pub async fn get_item(db_pool: web::Data<DbPool>, id_item: i32) -> Result<Item, MyError> {
        let results_async = web::block(move ||  {
            let mut conn = db_pool.get().expect("error getting db connection");
    
            use crate::schema::items::dsl::*;
            items.filter(id.eq(id_item)).select(Item::as_select()).first(&mut conn)
        }).await;

        let results = match results_async {
            Ok(Ok(items)) => items,
            Ok(Err(err)) => {
                match err {
                    diesel::result::Error::NotFound => {
                        return Err(MyError::InternalError);
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

        return Ok(results);
    }

    pub async fn insert_item(db_pool: web::Data<DbPool>, item_dto: ItemDto) -> Result<(), MyError> {
        let name = item_dto.name.unwrap_or("".to_string());

        if name.is_empty() {
            return Err(MyError::BadClientData { msg: "nama tidak boleh kosong".to_string() });
        }

        let new_item = ItemDto {
            name: Some(name)
        };
        
        let results_async = web::block(move ||  {
            let mut conn = db_pool.get().expect("error getting db connection");
    
            use crate::schema::items;
            diesel::insert_into(items::table).values(&new_item).execute(&mut conn)
        }).await;

        let _results = match results_async {
            Ok(Ok(items)) => items,
            Ok(Err(err)) => {
                match err {
                    diesel::result::Error::NotFound => {
                        return Err(MyError::NotFound { msg: "tidak ada data di database".to_string() });
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

        return Ok(());
    }

    pub async fn update_item(db_pool: web::Data<DbPool>, target_update_id: i32, item_dto: ItemDto) -> Result<(), MyError> {
        let updated_name = item_dto.name.unwrap_or("".to_string());

        if updated_name.is_empty() {
            return Err(MyError::BadClientData { msg: "nama tidak boleh kosong".to_string() });
        }

        let results_async = web::block(move ||  {
            use crate::schema::items::dsl::*;

            let mut conn = db_pool.get().expect("error getting db connection");
    
            diesel::update(items.find(target_update_id)).set(name.eq(updated_name)).execute(&mut conn)
        }).await;

        let _results = match results_async {
            Ok(Ok(res)) => res,
            Ok(Err(err)) => {
                match err {
                    diesel::result::Error::NotFound => {
                        return Err(MyError::NotFound { msg: "tidak ada data di database".to_string() });
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

        return Ok(());
    }
}