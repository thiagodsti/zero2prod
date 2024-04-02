use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::Utc;
use sqlx::types::Uuid;
use sqlx::PgPool;

use crate::domain::NewSubscriber;
use crate::startup::AppState;

#[derive(Serialize, Deserialize)]
pub struct NewSubscriberDto {
    email: String,
    name: String,
}

impl TryFrom<NewSubscriberDto> for NewSubscriber {
    type Error = String;
    fn try_from(value: NewSubscriberDto) -> Result<Self, Self::Error> {
        let name = value.name;
        let email = value.email;
        NewSubscriber::new(email, name)
    }
}

#[tracing::instrument(
name = "Adding a new subscriber",
skip(form, state),
fields(
subscriber_email = % form.email,
subscriber_name = % form.name
)
)]
pub async fn subscribe(
    State(state): State<AppState>,
    Json(form): Json<NewSubscriberDto>,
) -> StatusCode {
    let new_subscriber: NewSubscriber = match form.try_into() {
        Ok(subscriber) => subscriber,
        Err(_) => return StatusCode::BAD_REQUEST,
    };
    match insert_subscriber(&state.pool, &new_subscriber).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(new_subscriber, pool)
)]
pub async fn insert_subscriber(
    pool: &PgPool,
    new_subscriber: &NewSubscriber,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        new_subscriber.email,
        new_subscriber.name,
        Utc::now()
    )
    // We use `get_ref` to get an immutable reference to the `PgConnection`
    // wrapped by `web::Data`.
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}
