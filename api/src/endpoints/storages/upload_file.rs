use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::error::ResponseError;
use actix_web::http::StatusCode;
use actix_web::{post, web, HttpResponse, Responder, Result};
use application::storage;
use application::storage::service::{FileUploadError, FileUploader};
use derive_more::{Display, Error};
use serde::Serialize;
use std::sync::Arc;
use tracing::debug;

use crate::responders::JSONResponseError;

#[post("/{id}/files")]
pub async fn upload_file(
    service: web::Data<Arc<FileUploader>>,
    path: web::Path<storage::ID>,
    MultipartForm(form): MultipartForm<UploadForm>,
) -> Result<impl Responder, UploadError> {
    debug!(
        storage_id = tracing::field::debug(&path),
        "upload file to storage"
    );

    let id = path.into_inner();

    let path = form.file.file.into_temp_path();
    let name = form
        .file
        .file_name
        .unwrap_or(uuid::Uuid::new_v4().to_string());

    let mut fp = tokio::fs::File::open(path)
        .await
        .map_err(UploadError::OpenTemporaryFile)?;

    let path = service
        .upload(id, &name, &mut fp)
        .await
        .map_err(UploadError::UploadError)?;

    Ok(HttpResponse::Created().json(UploadResponse { path }))
}

#[derive(Debug, MultipartForm)]
pub struct UploadForm {
    file: TempFile,
}

#[derive(Debug, Serialize)]
pub struct UploadResponse {
    path: storage::Path,
}

#[derive(Debug, Display, Error)]
pub enum UploadError {
    #[display(fmt = "failed to get storage {}", self.0)]
    OpenTemporaryFile(std::io::Error),

    #[display(fmt = "{}", self.0)]
    UploadError(FileUploadError),
}

impl ResponseError for UploadError {
    fn status_code(&self) -> StatusCode {
        match self {
            UploadError::OpenTemporaryFile(_) => StatusCode::INTERNAL_SERVER_ERROR,
            UploadError::UploadError(err) if matches!(err, FileUploadError::StorageNotFound) => {
                StatusCode::NOT_FOUND
            }
            UploadError::UploadError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
impl JSONResponseError for UploadError {}
