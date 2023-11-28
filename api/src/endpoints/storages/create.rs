use actix_web::{post, web, HttpResponse, Responder, Result};
use application::storage::service::Creator;
use application::storage::Settings;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::debug;

use crate::responders::AnyhowErrorResponder;

#[post("")]
pub async fn create(
    service: web::Data<Arc<Creator>>,
    req: web::Json<CreateRequest>,
) -> Result<impl Responder, AnyhowErrorResponder> {
    debug!(req = tracing::field::debug(&req), "create storage");

    let id = service
        .create(req.name.clone(), req.settings.clone())
        .await?;

    Ok(HttpResponse::Created().json(CreateResponse { id: id.to_string() }))
}

#[derive(Debug, Deserialize)]
pub struct CreateRequest {
    name: String,
    settings: Settings,
}

#[derive(Debug, Serialize)]
pub struct CreateResponse {
    id: String,
}
