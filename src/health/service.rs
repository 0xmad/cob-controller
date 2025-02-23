use actix_web::{error, get, web, Error, HttpResponse};
use std::{any::Any, sync::Arc};

use crate::context::Context;

#[get("/health")]
pub async fn health_check() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body(format!("Running")))
}
