use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::error::ResponseError;
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{post, web, HttpResponse, Responder, Result};
use application::storage::service::Get;
use application::storage;
use derive_more::{Display, Error};
use std::sync::Arc;
use tracing::{debug, error};

#[post("/{id}/files")]
pub async fn upload_file(
    service: web::Data<Arc<Get>>,
    path: web::Path<storage::ID>,
    MultipartForm(form): MultipartForm<UploadForm>,
) -> Result<impl Responder, UploadError> {
    debug!(
        storage_id = tracing::field::debug(&path),
        "upload file to storage"
    );

    let id = path.into_inner();

    let storage = service
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
