use axum::body::Body;
use axum::http::Request;
use serde_json::json;
use sqlx::{query};


use crate::utils::startup::{spawn_app};

mod utils;

use tower::{ServiceExt};

// for `call`, `oneshot`, and `ready`
#[tokio::test]
async fn health_check_works() {
    let (app, _) = spawn_app().await;

    let response = app.oneshot(Request::get("/health_check").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert!(response.status().is_success());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let (app, pg_pool) = spawn_app().await;

    // Act
    let body = json!({
    "email": "test@thiago.com",
    "name": "Thiago"
    }).to_string();
    let payload = Body::from(body);
    let response = app.oneshot(Request::post("/subscriptions")
        .header("Content-Type", "application/json")
        .body(payload)
        .unwrap())
        .await
        .unwrap();

    // Assert
    assert_eq!(200, response.status());

    let saved = query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&pg_pool)
        .await
        .expect("Failed to fetch saved subscription.");
    assert_eq!(saved.email, "test@thiago.com");
    assert_eq!(saved.name, "Thiago");
}

#[tokio::test]
async fn subscribe_returns_a_422_when_data_unprocessable() {
    // Arrange
    let (app, _) = spawn_app().await;
    let test_cases = vec![
        (json!({"age": 123}), "missing the email"),
        (json!({"name": "thiago"}), "missing the name"),
        (json!({}), "missing both name and email"),
    ];
    for (invalid_body, error_message) in test_cases {
        // Act
        let response = app.clone().oneshot(Request::post("/subscriptions")
            .header("Content-Type", "application/json")
            .body(Body::from(invalid_body.to_string()))
            .unwrap())
            .await
            .unwrap();
        // Assert
        assert_eq!(
            422,
            response.status().as_u16(),
            // Additional customised error message on test failure
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}

#[tokio::test]
async fn subscribe_returns_a_200_when_fields_are_present_but_empty() {
    // Arrange
    let (app, _) = spawn_app().await;
    let test_cases = vec![
        (json!({"email": "test@thiago.com","name": ""}), "empty name"),
        (json!({"email": "","name": "Thiago"}), "empty email"),
        (
            json!({"email": "definitely-not-an-email","name": "Thiago"}),
            "invalid email",
        ),
    ];
    for (body, description) in test_cases {
        // Act
        let response = app.clone().oneshot(Request::post("/subscriptions")
            .header("Content-Type", "application/json")
            .body(Body::from(body.to_string()))
            .unwrap())
            .await
            .unwrap();
        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not return a 200 OK when the payload was {}.",
            description
        );
    }
}
