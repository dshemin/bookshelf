use actix_web::{get, web, HttpResponse, Responder, Result};
use application::storage::service::Lister;
use application::Cursor;
use serde::Deserialize;
use std::sync::Arc;
use tracing::debug;

use crate::responders::AnyhowErrorResponder;

#[get("")]
pub async fn list(
    service: web::Data<Arc<Lister>>,
    query: web::Query<ListRequest>,
) -> Result<impl Responder, AnyhowErrorResponder> {
    debug!(req = tracing::field::debug(&query), "list storages");

    let cursor = query.cursor.clone();

    let res = service.list(cursor).await?;

    Ok(HttpResponse::Ok().json(res))
}

#[derive(Debug, Deserialize)]
pub struct ListRequest {
    cursor: Option<Cursor>,
}
