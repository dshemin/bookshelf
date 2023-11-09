use actix_web::{post, web, HttpResponse, Responder};
use application::storage::service::Create;
use application::storage::Settings;
use serde::Deserialize;
use std::sync::Arc;
use tracing::{debug, error};

#[post("")]
pub async fn create(
    service: web::Data<Arc<Create>>,
    req: web::Json<StorageCreate>,
) -> impl Responder {
    debug!(req = tracing::field::debug(&req), "create storage");

    let res = service
        .create(req.name.clone(), req.settings.clone())
        .await;

    match res {
        Ok(_) => HttpResponse::Created(),
        Err(e) => {
            error!(err = e.to_string(), "failed to create storage");
            HttpResponse::InternalServerError()
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct StorageCreate {
    name: String,
    settings: Settings,
}
