use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::fs;

pub async fn get_pool() -> SqlitePool {
    println!("Getting the DB pool");

    let db_url = "sqlite://hc.db";
    SqlitePoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await
        .unwrap()
}

pub async fn _reset(pool: &SqlitePool) {
    println!("Resetting the DB");
    let query = fs::read_to_string("./create_db.sql").unwrap();
    sqlx::query(&query).execute(pool).await.unwrap();
}
