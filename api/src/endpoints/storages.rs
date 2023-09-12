use actix_web::{delete, get, post, web, HttpResponse, Responder};
use crate::AppState;
use tracing::{debug, error};
use serde::Deserialize;
use application::storage::{self, Settings};


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

#[get("/{id}")]
pub async fn get(state: AppState, path: web::Path<storage::ID>) -> impl Responder {
    error!(req=tracing::field::debug(&path), "get storage");

    let id = path.into_inner();

    let res = state.storage_services.get.get(id).await;

    match res {
        Ok(v) => match v {
            Some(v) => HttpResponse::Ok().json(v),
            None => HttpResponse::NotFound().finish(),
        },
        Err(e) => {
            error!(err=e.to_string(), "failed to get storage");
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[delete("/{id}")]
pub async fn delete(state: AppState, path: web::Path<storage::ID>) -> impl Responder {
    error!(req=tracing::field::debug(&path), "delete storage");

    let id = path.into_inner();

    let res = state.storage_services.delete.delete(id).await;

    match res {
        Ok(()) => HttpResponse::NoContent().finish(),
        Err(e) => {
            error!(err=e.to_string(), "failed to delete storage");
            HttpResponse::InternalServerError().finish()
        }
    }
}
