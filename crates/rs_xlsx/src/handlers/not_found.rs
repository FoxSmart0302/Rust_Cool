use actix_web::http::StatusCode;
use actix_web::{HttpRequest, HttpResponse, Responder};

pub async fn not_found(req: HttpRequest) -> impl Responder {
    println!("404 not found: {:?} {}", req.path(), req.method());

    HttpResponse::build(StatusCode::NOT_FOUND)
        .content_type("text/html; charset=utf-8")
        .body("<h1>Error 404</h1>")
}
