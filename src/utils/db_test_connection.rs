#[cfg(test)]
pub mod db_config {
    use crate::configuration::{get_configuration, DatabaseSettings};
    use sqlx::{Connection, Executor, PgConnection, PgPool};
    use uuid::Uuid;

    pub async fn configure_database() -> PgPool {
        let mut configuration = get_configuration().expect("Failed to read configuration.");
        configuration.database.database_name = Uuid::new_v4().to_string();
        _configure_database(&configuration.database).await
    }

    async fn _configure_database(config: &DatabaseSettings) -> PgPool {
        // Create database
        let mut connection = PgConnection::connect_with(&config.without_db())
            .await
            .expect("Failed to connect to Postgres");
        connection
            .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
            .await
            .expect("Failed to create database.");
        // Migrate database
        let connection_pool = PgPool::connect_with(config.with_db())
            .await
            .expect("Failed to connect to Postgres.");
        sqlx::migrate!("./migrations")
            .run(&connection_pool)
            .await
            .expect("Failed to migrate the database");
        connection_pool
    }
}
