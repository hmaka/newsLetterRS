use actix_web::{HttpRequest, HttpResponse, Responder};

pub async fn subscribe(_req: HttpRequest) -> impl Responder {
    return HttpResponse::Ok();
}
