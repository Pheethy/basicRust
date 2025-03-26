use std::sync::Arc;

use basic_rust::config::config_loader;
use basic_rust::domain::repositories::adventurer::AdventurerRepository;
use basic_rust::infrastructure::axum_http::http_serve::start;
use basic_rust::infrastructure::postgres::postgres_connection::establish_connection;
use basic_rust::infrastructure::postgres::repositories::adventurer::AdventurerPostgres;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let dot_env = match config_loader::load() {
        Ok(env) => env,
        Err(e) => {
            tracing::error!("Failed to load .env file: {}", e);
            std::process::exit(1);
        }
    };

    let pool = match establish_connection(&dot_env.database.url).await {
        Ok(pool) => pool,
        Err(e) => {
            tracing::error!("Failed to pool connection: {}", e);
            std::process::exit(1);
        }
    };

    tracing::info!("Connected to database");

    let pool_arc = Arc::new(pool);
    let adventurer_repo = AdventurerPostgres::new(Arc::clone(&pool_arc));

    match adventurer_repo.fetch_adventurer_by_id(1).await {
        Ok(adventurer) => tracing::info!("Found adventurer: {:?}", adventurer),
        Err(e) => tracing::error!("Failed to fetch adventurer: {}", e),
    }

    start(Arc::new(dot_env))
        .await
        .expect("Failed to start server");
}
