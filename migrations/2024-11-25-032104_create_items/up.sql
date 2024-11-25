-- Your SQL goes here
create table items(
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name text,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
)