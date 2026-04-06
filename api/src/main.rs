mod adapter;
mod app;
mod config;
mod domain;
mod error;
mod service;
mod state;

#[tokio::main]
async fn main() {
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .expect("failed to install CryptoProvider");

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    dotenvy::dotenv().ok();

    let cfg = config::AppConfig::from_env();
    let app = app::build_app(cfg).await;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}