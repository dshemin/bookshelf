use actix_web::{post, web, HttpResponse, Responder};
use crate::AppState;
use tracing::{debug, error};
use serde::Deserialize;
use application::storage::Settings;

#[derive(Debug, Deserialize)]
pub struct StorageCreate {
    name: String,
    settings: Settings,
}

#[post("")]
pub async fn create(state: AppState, req: web::Json<StorageCreate>) -> impl Responder {
    debug!(req=tracing::field::debug(&req), "create storage");

    let res = state.storage_services.create.create(
        req.name.clone(),
        req.settings.clone(),
    ).await;

    match res {
        Ok(_) => HttpResponse::Ok(),
        Err(e) => {
            error!(err=e.to_string(), "failed to create storage to our database");
            HttpResponse::InternalServerError()
        },
    }
}
