use std::net::TcpListener;

use once_cell::sync::Lazy;
use serde_json::json;
use sqlx::{query, Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;

use zero2prod::configuration::{get_configuration, DatabaseSettings};
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

use crate::utils::startup::spawn_app;

mod utils;

#[tokio::test]
async fn send_new_users_returns_201() {
    // Arrange
    let app = spawn_app().await;
    let _configuration = get_configuration().expect("Failed to read configuration");

    let client = reqwest::Client::new();
    // Act
    let body = json!({
        "name": "Thiago",
        "email": "test@thiago.com",
        "password": "1234512345",
        "roles": ["BASIC", "ADMIN"]
    });
    let response = client
        .post(&format!("{}/users", &app.address))
        .json(&body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(201, response.status().as_u16());

    let saved = query!("SELECT email, name FROM users",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved users.");
    assert_eq!(saved.email, "test@thiago.com");
    assert_eq!(saved.name, "Thiago");
}
