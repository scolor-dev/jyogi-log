use axum::body::Body;
use axum::http::{Request, StatusCode};
use api::{app, config::Config, state::AppState};
use tower::util::ServiceExt;

#[tokio::test]
async fn health_returns_200() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config {
        app_host: "127.0.0.1".to_string(),
        app_port: 3000,
        rust_log: "info".to_string(),
    };

    let state = AppState::new(config);
    let app = app::create_app(state);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .method("GET")
                .body(Body::empty())?,
        )
        .await?;

    assert_eq!(response.status(), StatusCode::OK);

    Ok(())
}
