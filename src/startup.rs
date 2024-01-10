use crate::routes::health_check;
use crate::routes::subscribe;
use actix_web::{dev::Server, web, App, HttpServer};
use std::net::TcpListener;

pub fn run(listner: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listner)?
    .run();

    return Ok(server);
}
