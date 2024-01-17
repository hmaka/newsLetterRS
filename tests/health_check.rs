use newsletter_rs::configuration::{get_configuration, DataBaseSettings};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

async fn spawn_app() -> TestApp {
    let listner = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listner.local_addr().unwrap().port();

    let mut configuration = get_configuration().expect("Failed to read configuration.");
    configuration.database.database_name = uuid::Uuid::new_v4().to_string();
    let connection_pool = configure_database(&configuration.database).await;
    let server = newsletter_rs::startup::run(listner, connection_pool.clone())
        .expect("Failed to bind address");
    let _ = tokio::spawn(server);

    return TestApp {
        address: format!("http://127.0.0.1:{}", port),
        db_pool: connection_pool,
    };
}

pub async fn configure_database(config: &DataBaseSettings) -> PgPool {
    let mut connection = PgConnection::connect(&config.connection_string_without_db())
        .await
        .expect("Failed to connect to Postgres.");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database.");

    let connection_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    return connection_pool;
}

#[actix_web::test]
async fn health_check_works() {
    let test_app: TestApp = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health_check", test_app.address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[actix_web::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let test_app = spawn_app().await;

    let client = reqwest::Client::new();
    let mock_body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    let response = client
        .post(format!("{}/subscriptions", test_app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(mock_body)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&test_app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[actix_web::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_reason) in test_cases {
        let response = client
            .post(format!("{}/subscriptions", test_app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Reqest when the payload was {error_reason}"
        )
    }
}
