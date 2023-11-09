use actix_web::{get, web, HttpResponse, Responder};
use application::storage::service::List;
use application::Cursor;
use serde::Deserialize;
use std::sync::Arc;
use tracing::{debug, error};

#[get("")]
pub async fn list(service: web::Data<Arc<List>>, query: web::Query<Paging>) -> impl Responder {
    debug!(req = tracing::field::debug(&query), "list storages");

    let cursor = query.cursor.clone();

    let res = service.list(cursor).await;

    match res {
        Ok(v) => HttpResponse::Ok().json(v),
        Err(e) => {
            error!(err = e.to_string(), "failed to list storages");
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Paging {
    cursor: Option<Cursor>,
}
