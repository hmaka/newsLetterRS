use newsletter_rs::{configuration::get_configuration, configuration::Settings, startup::run};
use std::net::TcpListener;
//async fn greet(req: HttpRequest) -> impl Responder {
//    let name = req.match_info().get("name").unwrap_or("World");
//    format!("Hello {}!", &name)
//}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration: Settings = get_configuration().expect("Failed to read configuration");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listner = TcpListener::bind(address).expect("Failed to bind random port");

    run(listner)?.await
}
