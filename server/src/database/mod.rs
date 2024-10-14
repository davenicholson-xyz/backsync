pub mod clients;
pub mod wallpaper;
use std::sync::OnceLock;

use crate::database;
use anyhow::anyhow;
use anyhow::Result;
use sqlx::migrate::MigrateDatabase;
use sqlx::SqlitePool;

use crate::system::config;

pub static DB_POOL: OnceLock<SqlitePool> = OnceLock::new();

pub async fn init() -> Result<()> {
    let storage = config::get::<String>("storage")?.unwrap();
    let conn_string = format!("sqlite://{}/store.db", storage);
    if !sqlx::Sqlite::database_exists(&conn_string).await? {
        sqlx::Sqlite::create_database(&conn_string).await?
    }

    let pool = SqlitePool::connect(&conn_string).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    DB_POOL
        .set(pool)
        .map_err(|_| anyhow!("Database already initialized"))?;

    database::clients::startup_clean().await?;

    Ok(())
}

pub fn pool() -> &'static SqlitePool {
    DB_POOL.get().expect("Database pool is not initialized")
}
