use crate::routes::health_check;
use crate::routes::subscribe;
use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(listner: TcpListener, connection: PgPool) -> Result<Server, std::io::Error> {
    let connection = web::Data::new(connection);

    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(connection.clone())
    })
    .listen(listner)?
    .run();

    return Ok(server);
}
