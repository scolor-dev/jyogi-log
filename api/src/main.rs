mod adapter;
mod app;
mod config;
mod domain;
mod error;
mod service;
mod state;

use config::Config;
use state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let config = Config::from_env()?;

    tracing_subscriber::fmt()
        .with_env_filter(&config.rust_log)
        .init();

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(config.db_max_connections)
        .connect(&config.database_url)
        .await?;

    let state = AppState::new(pool);
    let app = app::create_app(state);
    let addr = config.listen_addr()?;
    let listener = tokio::net::TcpListener::bind(addr).await?;

    tracing::info!("server listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}