use api::{
    adapter::logging::init,
    app,
    config::Config,
    state::AppState,
};

#[tokio::main]
async fn main() -> Result<(), api::error::AppError> {
    let _ = dotenvy::dotenv();

    let config = Config::from_env()?;

    init::init(&config.rust_log);

    tracing::info!("starting server");

    let addr = config.listen_addr()?;
    let state = AppState::new(config);
    let app = app::create_app(state);

    let listener = tokio::net::TcpListener::bind(addr).await?;

    tracing::info!("server listening");

    axum::serve(listener, app).await?;

    Ok(())
}
