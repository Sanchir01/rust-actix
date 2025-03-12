use sqlx::{postgres::PgPool, Pool, Postgres};
use crate::config::Config;

pub async fn init_primary_db(config: &Config) -> Result<Pool<Postgres>, sqlx::Error> {
    let db_config = config.database.as_ref()
        .expect("Database configuration is required");

    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        db_config.username,
        db_config.password,
        db_config.host,
        db_config.port,
        db_config.database
    );

    let pool =PgPool::connect(&database_url).await.expect("Failed to connect to Postgres");

    return  Ok(pool);
}