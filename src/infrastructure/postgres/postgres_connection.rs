use anyhow::Result;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

pub async fn establish_connection(database_url: &str) -> Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(5) // ปรับค่าตามต้องการ
        .connect(database_url)
        .await?;
    Ok(pool)
}
