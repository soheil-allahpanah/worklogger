use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

/// Builds a PostgreSQL pool from `DATABASE_URL` and runs embedded migrations.
pub async fn connect(database_url: &str) -> Result<PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;
    Ok(pool)
}
