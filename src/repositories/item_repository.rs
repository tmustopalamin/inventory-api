use diesel::{r2d2::ConnectionManager, result::Error, QueryDsl, RunQueryDsl, SelectableHelper, SqliteConnection};
use r2d2::PooledConnection;

use crate::models::item::Item;

pub fn get_all(conn: &mut PooledConnection<ConnectionManager<SqliteConnection>>) -> Result<Vec<Item>, Error> {
    use crate::schema::items::dsl::*;
    items.select(Item::as_select()).load(conn)
}