use api::{
    adapter::logging::init,
    app,
    config::Config,
    state::AppState,
};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), api::error::AppError> {
    let _ = dotenvy::dotenv();

    let config = Config::from_env()?;

    init::init(&config.rust_log);

    tracing::info!("connecting to database");

    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await?;

    tracing::info!("starting server");

    let addr = config.listen_addr()?;
    let state = AppState::new(config, db);
    let app = app::create_app(state);

    let listener = tokio::net::TcpListener::bind(addr).await?;

    tracing::info!("server listening");

    axum::serve(listener, app).await?;

    Ok(())
}
