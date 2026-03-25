use axum::body::Body;
use axum::http::{Request, StatusCode};
use api::{app, state::AppState};
use tower::util::ServiceExt;

const TEST_DATABASE_URL: &str = "postgres://localhost/test";

fn test_state() -> Result<AppState, Box<dyn std::error::Error>> {
    let db = sqlx::PgPool::connect_lazy(TEST_DATABASE_URL)?;
    Ok(AppState::new(db))
}

#[tokio::test]
async fn health_returns_200() -> Result<(), Box<dyn std::error::Error>> {
    let state = test_state()?;
    let app = app::create_app(state);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/health")
                .method("GET")
                .body(Body::empty())?,
        )
        .await?;

    assert_eq!(response.status(), StatusCode::OK);

    Ok(())
}

#[tokio::test]
async fn legacy_health_returns_404() -> Result<(), Box<dyn std::error::Error>> {
    let state = test_state()?;
    let app = app::create_app(state);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .method("GET")
                .body(Body::empty())?,
        )
        .await?;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    Ok(())
}
