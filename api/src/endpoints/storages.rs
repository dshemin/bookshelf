use crate::AppState;
use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::error::ResponseError;
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder, Result};
use application::storage::{self, Settings};
use derive_more::{Display, Error};
use serde::Deserialize;
use tracing::{debug, error};

#[post("")]
pub async fn create(state: AppState, req: web::Json<StorageCreate>) -> impl Responder {
    debug!(req = tracing::field::debug(&req), "create storage");

    let res = state
        .storage_services
        .create
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

#[get("")]
pub async fn list(state: AppState, query: web::Query<Paging>) -> impl Responder {
    debug!(req = tracing::field::debug(&query), "list storages");

    let cursor = query.cursor.clone();

    let res = state.storage_services.list.list(cursor).await;

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
    cursor: Option<application::Cursor>,
}

#[get("/{id}")]
pub async fn get(state: AppState, path: web::Path<storage::ID>) -> impl Responder {
    debug!(req = tracing::field::debug(&path), "get storage");

    let id = path.into_inner();

    let res = state.storage_services.get.get(id).await;

    match res {
        Ok(v) => match v {
            Some(v) => HttpResponse::Ok().json(v),
            None => HttpResponse::NotFound().finish(),
        },
        Err(e) => {
            error!(err = e.to_string(), "failed to get storage");
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[put("/{id}")]
pub async fn update(
    state: AppState,
    path: web::Path<storage::ID>,
    req: web::Json<StorageCreate>,
) -> impl Responder {
    debug!(
        req = tracing::field::debug(&req),
        path = tracing::field::debug(&path),
        "update storage"
    );

    let id = path.into_inner();

    let res = state
        .storage_services
        .update
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

#[delete("/{id}")]
pub async fn delete(state: AppState, path: web::Path<storage::ID>) -> impl Responder {
    debug!(req = tracing::field::debug(&path), "delete storage");

    let id = path.into_inner();

    let res = state.storage_services.delete.delete(id).await;

    match res {
        Ok(()) => HttpResponse::NoContent().finish(),
        Err(e) => {
            error!(err = e.to_string(), "failed to delete storage");
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/{id}/files")]
pub async fn upload_file(
    state: AppState,
    path: web::Path<storage::ID>,
    MultipartForm(form): MultipartForm<UploadForm>,
) -> Result<impl Responder, UploadError> {
    debug!(
        storage_id = tracing::field::debug(&path),
        "upload file to storage"
    );

    let id = path.into_inner();

    let storage = state
        .storage_services
        .get
        .get(id)
        .await
        .map_err(|err| {
            error!(err = err.to_string(), "failed to get storage");
            UploadError::FailedToGetStorage
        })?
        .ok_or(UploadError::StorageNotFound)?;

    let engine = storage.connect().await.map_err(|err| {
        error!(err = err.to_string(), "failed to connect");
        UploadError::FailedToConnect
    })?;

    let path = form.file.file.into_temp_path();
    let name = form
        .file
        .file_name
        .unwrap_or(uuid::Uuid::new_v4().to_string());

    let mut fp = tokio::fs::File::open(path).await.map_err(|err| {
        error!(err = err.to_string(), "failed to open temporary file");
        UploadError::FailedToOpenTemporaryFile
    })?;

    let path = engine.put(&name, &mut fp).await.map_err(|err| {
        error!(err = err.to_string(), "failed to put new file to storage");
        UploadError::FailedToPutFileToStorage
    })?;

    Ok(HttpResponse::Created().json(path))
}

#[derive(Debug, Display, Error)]
pub enum UploadError {
    #[display(fmt = "failed to get storage")]
    FailedToGetStorage,

    #[display(fmt = "storage not found")]
    StorageNotFound,

    #[display(fmt = "failed to connect")]
    FailedToConnect,

    #[display(fmt = "failed to open temporary file")]
    FailedToOpenTemporaryFile,

    #[display(fmt = "failed to put file to storage")]
    FailedToPutFileToStorage,
}

impl ResponseError for UploadError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        match *self {
            UploadError::StorageNotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[derive(Debug, MultipartForm)]
pub struct UploadForm {
    file: TempFile,
}
