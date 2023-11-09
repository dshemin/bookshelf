use actix_web::{get, web, HttpResponse, Responder};
use application::storage;
use application::storage::service::Get;
use std::sync::Arc;
use tracing::{debug, error};

#[get("/{id}")]
pub async fn get(service: web::Data<Arc<Get>>, path: web::Path<storage::ID>) -> impl Responder {
    debug!(req = tracing::field::debug(&path), "get storage");

    let id = path.into_inner();

    let res = service.get(id).await;

    match res {
        Ok(v) => match v {
            Some(v) => HttpResponse::Ok().json(v),
            None => HttpResponse::NotFound().finish(),
        },
        Err(e) => {
            error!(err = e.to_string(), "failed to get storage");
            HttpResponse::InternalServerError().finish()
        }
    }
}
