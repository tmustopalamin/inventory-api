use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::items;

#[derive(Serialize, Deserialize)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::items)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Item {
    pub id: i32,
    pub name: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
#[derive(Insertable)]
#[diesel(table_name = items)]
pub struct ItemDto {
    pub name: Option<String>,
}

