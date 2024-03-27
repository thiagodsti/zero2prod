use sqlx::migrate::Migrator;
use sqlx::postgres::PgPoolOptions;

use zero2prod::configuration::get_configuration;
use zero2prod::startup::app;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

static MIGRATOR: Migrator = sqlx::migrate!();

#[tokio::main]
async fn main() {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.database.with_db());

    MIGRATOR.run(&connection_pool).await.unwrap();

    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app(connection_pool)).await.unwrap()
}
