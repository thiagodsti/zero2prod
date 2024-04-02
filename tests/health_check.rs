use axum::body::Body;
use axum::http::Request;

use crate::utils::startup::spawn_app;

mod utils;

use tower::ServiceExt;

// for `call`, `oneshot`, and `ready`
#[tokio::test]
async fn health_check_works() {
    let (app, _) = spawn_app().await;

    let response = app
        .oneshot(Request::get("/health_check").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert!(response.status().is_success());
}
