mod adapter;
mod app;
mod config;
mod domain;
mod error;
mod service;
mod state;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt().init();

    let state = state::AppState::new();
    let app = app::create_app(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;

    tracing::info!("server listening on 0.0.0.0:8080");

    axum::serve(listener, app).await?;

    Ok(())
}