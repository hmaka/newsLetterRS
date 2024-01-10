use newsletter_rs::startup::run;
use std::net::TcpListener;
//async fn greet(req: HttpRequest) -> impl Responder {
//    let name = req.match_info().get("name").unwrap_or("World");
//    format!("Hello {}!", &name)
//}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listner = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");

    run(listner)?.await
}
