use actix_web::http::StatusCode;
use actix_web::{put, web, HttpResponse, Responder, ResponseError, Result};
use application::storage::service::{UpdateError, Updater as UpdateService};
use application::storage::{self, Settings};
use derive_more::{Display, From};
use serde::Deserialize;
use std::sync::Arc;
use tracing::debug;

use crate::responders::JSONResponseError;

#[put("/{id}")]
pub async fn update(
    service: web::Data<Arc<UpdateService>>,
    path: web::Path<storage::ID>,
    req: web::Json<Update>,
) -> Result<impl Responder, WrappedUpdateError> {
    debug!(
        req = tracing::field::debug(&req),
        path = tracing::field::debug(&path),
        "update storage"
    );

    let id = path.into_inner();

    let res = service
        .update(id, req.name.clone(), req.settings.clone())
        .await?;

    Ok(HttpResponse::Ok().json(res))
}

#[derive(Debug, Deserialize)]
pub struct Update {
    name: String,
    settings: Settings,
}

#[derive(Debug, Display, From)]
pub struct WrappedUpdateError(UpdateError);

impl ResponseError for WrappedUpdateError {
    fn status_code(&self) -> StatusCode {
        match self.0 {
            UpdateError::DB(_) => StatusCode::INTERNAL_SERVER_ERROR,
            UpdateError::NotFound() => StatusCode::NOT_FOUND,
        }
    }
}
impl JSONResponseError for WrappedUpdateError {}
