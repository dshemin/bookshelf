use actix_web::{put, web, HttpResponse, Responder};
use application::storage::service::Update;
use application::storage::{self, Settings};
use serde::Deserialize;
use std::sync::Arc;
use tracing::{debug, error};

#[put("/{id}")]
pub async fn update(
    service: web::Data<Arc<Update>>,
    path: web::Path<storage::ID>,
    req: web::Json<StorageUpdate>,
) -> impl Responder {
    debug!(
        req = tracing::field::debug(&req),
        path = tracing::field::debug(&path),
        "update storage"
    );

    let id = path.into_inner();

    let res = service
        .update(id, req.name.clone(), req.settings.clone())
        .await;

    match res {
        Ok(v) => HttpResponse::Ok().json(v),
        Err(e) => {
            error!(err = e.to_string(), "failed to update storage");
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct StorageUpdate {
    name: String,
    settings: Settings,
}
