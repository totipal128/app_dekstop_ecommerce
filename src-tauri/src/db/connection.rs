use lazy_static::lazy_static;
use sqlx::{PgPool, postgres::PgPoolOptions, Error as SqlxError};
use sqlx::migrate::Migrator;
use std::env;
use std::time::Duration;
use std::path::Path;
use tokio::sync::OnceCell;


lazy_static! {
    static ref DB_CONN: OnceCell<PgPool> = OnceCell::new();
}

pub async fn get_db_pool() -> Result<&'static PgPool, SqlxError> {
    if let Some(pool) = DB_CONN.get() {
        return Ok(pool);
    }

    let database_url = env::var("DATABASE_URL")
        .map_err(|_| SqlxError::Configuration("DATABASE_URL must be set".into()))?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    DB_CONN.set(pool)
        .map_err(|_| SqlxError::Configuration("Pool already initialized".into()))?;

    Ok(DB_CONN.get().unwrap())
}

pub async fn get_db() -> Option<&'static PgPool> { 
    DB_CONN.get()
}


pub async fn run_migrations() -> Result<(), SqlxError> {
    let pool = get_db_pool().await?;
    
    // ✅ Load migrations secara runtime dengan path yang benar
    let migrations_path = format!("{}/../migrations", env!("CARGO_MANIFEST_DIR"));

    println!("Running migrations from path: {}", migrations_path);
    let migrator = Migrator::new(Path::new(&migrations_path)).await?;
    
    migrator.run(pool).await?;
    Ok(())
}