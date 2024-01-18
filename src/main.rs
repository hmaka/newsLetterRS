use newsletter_rs::{configuration::get_configuration, configuration::Settings, startup};
use sqlx::PgPool;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration: Settings = get_configuration().expect("Failed to read configuration");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listner = TcpListener::bind(address).expect("Failed to bind to port in config");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    startup::run(listner, connection_pool)?.await
}
