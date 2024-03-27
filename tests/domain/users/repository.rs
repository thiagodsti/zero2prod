use axum::body::Body;
use axum::http::Request;
use serde_json::json;
use sqlx::query;
use tower::ServiceExt;

use crate::utils::startup::spawn_app;

#[tokio::test]
async fn send_new_users_returns_201() {
    // Arrange
    let (app, pg_pool) = spawn_app().await;

    let body = json!({
        "name": "Thiago",
        "email": "test@thiago.com",
        "password": "1234512345",
        "roles": ["BASIC", "ADMIN"]
    }).to_string();

    let payload = Body::from(body);
    let response = app.oneshot(Request::post("/users")
        .header("Content-Type", "application/json")
        .body(payload)
        .unwrap())
        .await
        .unwrap();
    // Act

    // Assert
    assert_eq!(201, response.status().as_u16());

    let saved = query!("SELECT email, name FROM users",)
        .fetch_one(&pg_pool)
        .await
        .expect("Failed to fetch saved users.");
    assert_eq!(saved.email, "test@thiago.com");
    assert_eq!(saved.name, "Thiago");
}
