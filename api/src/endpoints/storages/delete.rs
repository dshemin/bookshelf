use actix_web::{delete, web, HttpResponse, Responder, Result};
use application::storage;
use application::storage::service::Deleter;
use std::sync::Arc;
use tracing::debug;

use crate::responders::AnyhowErrorResponder;

#[delete("/{id}")]
pub async fn delete(
    service: web::Data<Arc<Deleter>>,
    path: web::Path<storage::ID>,
) -> Result<impl Responder, AnyhowErrorResponder> {
    debug!(req = tracing::field::debug(&path), "delete storage");

    let id = path.into_inner();

    service.delete(id).await?;

    Ok(HttpResponse::NoContent().finish())
}
