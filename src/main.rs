use newsletter_rs::run;
//async fn greet(req: HttpRequest) -> impl Responder {
//    let name = req.match_info().get("name").unwrap_or("World");
//    format!("Hello {}!", &name)
//}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    run()?.await
}
