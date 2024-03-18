use std::net::TcpListener;
use actix_web::{web, App, HttpServer, HttpResponse};
use actix_web::dev::Server;
use serde::{Deserialize, Serialize};

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(Serialize, Deserialize)]
struct Subscriber {
    email: String,
    name: String
}
async fn subscribe(_form: web::Json<Subscriber>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> std::io::Result<Server> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
        .listen(listener)?
        .run();
    Ok(server)
}