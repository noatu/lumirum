use sqlx::{
    PgPool,
    Result,
    postgres::PgPoolOptions,
};

use std::time::Duration;

pub async fn setup_pool(url: &str) -> Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .connect(url)
        .await?;
    tracing::debug!("database pool connected");

    sqlx::migrate!().run(&pool).await?;
    tracing::debug!("finished running migrations");

    Ok(pool)
}
