use std::fmt::format;
use std::net::TcpListener;
use serde_json::json;

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    // We return the application address to the caller!
    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app();

    // We need to bring in `reqwest`
    // to perform HTTP requests against our app
    let client = reqwest::Client::new();
    // Act
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let body = json!({
        "email": "test@thiago.com",
        "name": "Thiago"
        });
    let response = client
        .post(&format!("{}/subscriptions", &app_address))
        .json(&body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
// Arrange
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        (json!({"age": 123}), "missing the email"),
        (json!({"name": "thiago"}), "missing the name"),
        (json!({}), "missing both name and email"),
    ];
    for (invalid_body, error_message) in test_cases {
// Act
        let response = client
            .post(&format!("{}/subscriptions", &app_address))
            .json(&invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");
// Assert
        assert_eq!(
            400,
            response.status().as_u16(),
// Additional customised error message on test failure
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}