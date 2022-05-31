use email_newsletter::configuration::{get_configuration, DatabaseSettings};
use sqlx::{Connection, PgConnection, PgPool, Pool, Postgres};
use std::net::TcpListener;
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub db_pool: Pool<Postgres>,
}

pub async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to a random port");

    let port = listener.local_addr().unwrap().port();

    let mut configuration = get_configuration().expect("Failed to read configuration");
    configuration.database.database_name = Uuid::new_v4().to_string();

    let db_pool = configure_database(&configuration.database).await;

    let server = email_newsletter::startup::run(db_pool.clone(), listener)
        .expect("failed to build `Email Newsletter` server");

    let _ = tokio::spawn(server);

    TestApp {
        address: format!("http://127.0.0.1:{}", port),
        db_pool,
    }
}

pub async fn configure_database(settings: &DatabaseSettings) -> Pool<Postgres> {
    let mut connection = PgConnection::connect(&settings.connection_string_without_db())
        .await
        .expect("Failed to connect to Postgres");

    let query = format!(r#"CREATE DATABASE "{}";"#, settings.database_name);

    sqlx::query(&query)
        .execute(&mut connection)
        .await
        .expect("Failed to create logical database");

    let connection_pool = PgPool::connect(&settings.connection_string())
        .await
        .expect("Failed to connect co Postgres");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate database");

    connection_pool
}
