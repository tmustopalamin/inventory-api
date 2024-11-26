use std::env;
use diesel::{r2d2::ConnectionManager, Connection, SqliteConnection};
use dotenvy::dotenv;
use r2d2::Pool;


pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;
pub fn establish_connection_pool() -> DbPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);

    Pool::builder()
    .build(manager)
    .expect("Failed to create pool.")
}

