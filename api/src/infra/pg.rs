use anyhow::Context;
use sqlx::postgres::PgPoolOptions;
use tokio::time::{sleep, Duration};

pub async fn pool(dsn: &str) -> anyhow::Result<sqlx::Pool<sqlx::Postgres>> {
    let max_attempts = std::env::var("PG_CONNECT_ATTEMPTS")
        .ok()
        .and_then(|v| v.parse::<u32>().ok())
        .unwrap_or(10);
    let backoff_ms = std::env::var("PG_CONNECT_BACKOFF_MS")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(1_000);

    let mut attempt = 0;
    loop {
        attempt += 1;
        match PgPoolOptions::new().max_connections(10).connect(dsn).await {
            Ok(pool) => return Ok(pool),
            Err(err) if attempt < max_attempts => {
                tracing::warn!(attempt, max_attempts, error = %err, "failed to connect to postgres; retrying");
                sleep(Duration::from_millis(backoff_ms)).await;
            }
            Err(err) => {
                return Err(anyhow::anyhow!(
                    "failed to connect to postgres after {attempt} attempts: {err}"
                ));
            }
        }
    }
}

pub async fn migrate(pool: &sqlx::Pool<sqlx::Postgres>) -> anyhow::Result<()> {
    static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");
    MIGRATOR
        .run(pool)
        .await
        .context("running database migrations")
}
