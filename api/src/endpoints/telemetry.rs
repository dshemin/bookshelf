use actix_web::{get, HttpResponse, Responder};

#[get("/healthz")]
async fn healthz() -> impl Responder {
    HttpResponse::Ok()
}
