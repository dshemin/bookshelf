use actix_web::{get, web, HttpResponse, Responder, Result};
use application::storage;
use application::storage::service::Getter;
use std::sync::Arc;
use tracing::debug;

use crate::responders::AnyhowErrorResponder;

#[get("/{id}")]
pub async fn get(
    service: web::Data<Arc<Getter>>,
    path: web::Path<storage::ID>,
) -> Result<impl Responder, AnyhowErrorResponder> {
    debug!(req = tracing::field::debug(&path), "get storage");

    let id = path.into_inner();

    let s = service.get(id).await?;

    Ok(match s {
        Some(v) => HttpResponse::Ok().json(v),
        None => HttpResponse::NotFound().finish(),
    })
}
