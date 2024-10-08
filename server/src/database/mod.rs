pub mod stream;
use std::sync::OnceLock;

use anyhow::anyhow;
use anyhow::Result;
use sqlx::migrate::MigrateDatabase;
use sqlx::SqlitePool;

pub static DB_POOL: OnceLock<SqlitePool> = OnceLock::new();

pub async fn init() -> Result<()> {
    //let storage = config::get::<String>("storage")?.unwrap();
    //dbg!(&storage);
    //let pool = SqlitePool::connect(("sqlite://{}/store.db", storage)).await?;
    let conn_string = format!("/Users/dave/.config/backsync-server/store.db");
    if !sqlx::Sqlite::database_exists(&conn_string).await? {
        sqlx::Sqlite::create_database(&conn_string).await?
    }

    let pool = SqlitePool::connect("sqlite:///Users/dave/.config/backsync-server/store.db").await?;
    DB_POOL
        .set(pool)
        .map_err(|_| anyhow!("Database already initialized"))?;
    startup().await?;

    Ok(())
}

pub fn pool() -> &'static SqlitePool {
    DB_POOL.get().expect("Database pool is not initialized")
}

async fn startup() -> Result<()> {
    let pool = pool();
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS streams (
            id INTEGER PRIMARY KEY,
            addr TEXT NOT NULL UNIQUE,
            hostname TEXT NOT NULL,
            connected_at TEXT
        )
        "#,
    )
    .execute(pool)
    .await?;
    Ok(())
}
