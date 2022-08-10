use actix_web::body::BoxBody;
use actix_web::{
    error, get,
    http::{header::ContentType, StatusCode},
    App, HttpResponse,
};
use derive_more::Display;
use std::fmt::{Display, Formatter};

// Our custom error types.
#[derive(Debug, thiserror::Error)]
pub enum AzureDashboardError {
    #[error("There was an error calling the Azure API: {0}")]
    AzureApiError(String),
    #[error("Internal error")]
    InternalError,
}

impl error::ResponseError for AzureDashboardError {}
