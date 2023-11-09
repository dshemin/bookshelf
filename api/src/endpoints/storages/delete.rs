use actix_web::{delete, web, HttpResponse, Responder};
use application::storage::service::Delete;
use application::storage;
use std::sync::Arc;
use tracing::{debug, error};

#[delete("/{id}")]
pub async fn delete(service: web::Data<Arc<Delete>>, path: web::Path<storage::ID>) -> impl Responder {
    debug!(req = tracing::field::debug(&path), "delete storage");

    let id = path.into_inner();

    let res = service.delete(id).await;

    match res {
        Ok(()) => HttpResponse::NoContent().finish(),
        Err(e) => {
            error!(err = e.to_string(), "failed to delete storage");
            HttpResponse::InternalServerError().finish()
        }
    }
}
