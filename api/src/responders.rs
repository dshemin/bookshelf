use std::fmt::Display;

use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use actix_web::{HttpResponse, ResponseError};
use derive_more::{Display, From};
use serde_json::json;

#[derive(Debug, Display, From)]
pub struct AnyhowErrorResponder(anyhow::Error);

impl ResponseError for AnyhowErrorResponder {}
impl JSONResponseError for AnyhowErrorResponder {}

pub trait JSONResponseError: ResponseError + Display {
    fn error(&self) -> String {
        format!("{}", self)
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(json!({ "error": self.error() }).to_string())
    }
}
