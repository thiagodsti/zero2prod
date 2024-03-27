use std::net::TcpListener;

use actix_web::dev::Server;
use actix_web::web::{Data};
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use tracing_actix_web::TracingLogger;
use crate::domain::users::repository::UserRepositoryImpl;
use crate::domain::users::service::{UserService, UserServiceImpl};

use crate::domain::users::route;
use crate::domain::users::route::save_new_user;
use crate::routes::{health_check, subscribe};

pub fn run(listener: TcpListener, db_pool: PgPool) -> std::io::Result<Server> {
    // Wrap the connection in a smart pointer
    let db_pool = Data::new(db_pool);

    //let user_route = UserRouteImpl::new(UserServiceImpl::new(UserRepositoryImpl::new(db_pool.clone())));

    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            //.route("/users", web::post().to(save_new_user))
            .service(web::scope("/users").service(save_new_user))
            //.route("/users", web::post().to(|json| user_route.save_new_user(json)))
            //.service(web::scope("/users").service(|json| user_route.save_new_user(json)))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
