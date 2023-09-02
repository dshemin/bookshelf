use actix_web::{get, post, web, HttpResponse, Responder};
use crate::AppState;
use tracing::{debug, error};
use serde::Deserialize;
use application::storage::Settings;


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
            error!(err=e.to_string(), "failed to create storage");
            HttpResponse::InternalServerError()
        },
    }
}

#[derive(Debug, Deserialize)]
pub struct StorageCreate {
    name: String,
    settings: Settings,
}

#[get("")]
pub async fn list(state: AppState, query: web::Query<Paging>) -> impl Responder {
    error!(req=tracing::field::debug(&query), "list storages");

    let cursor = query.cursor.clone();

    let res = state.storage_services.list.list(cursor).await;

    match res {
        Ok(v) =>
            HttpResponse::Ok().json(v),
        Err(e) => {
            error!(err=e.to_string(), "failed to list storages");
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Paging {
    cursor: Option<application::Cursor>,
}
