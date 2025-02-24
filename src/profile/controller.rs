use actix_web::{delete, post, web, Error, HttpRequest, HttpResponse};
use serde_json::json;
use std::sync::Arc;

use crate::{address::check_address, context::Context};

use super::service::{CreateArgs, KillArgs};

#[post("/profile/create")]
pub async fn create(
    context: web::Data<Arc<Context>>,
    req: HttpRequest,
    data: web::Json<CreateArgs>,
) -> Result<HttpResponse, Error> {
    let mut connection = context.redis_pool.get().map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Failed to get Redis connection: {}", e))
    })?;

    check_address(&req, data.address.clone())?;

    match super::service::create(
        &mut connection,
        CreateArgs {
            address: data.address.clone(),
        },
    ) {
        Ok(true) => {
            Ok(HttpResponse::Ok().json(json!({ "message": "Profile created successfully" })))
        }
        Ok(false) => {
            Ok(HttpResponse::BadRequest().json(json!({ "message": "Failed to create profile" })))
        }
        Err(err) => Err(actix_web::error::ErrorInternalServerError(format!(
            "Service error: {}",
            err
        ))),
    }
}

#[delete("/profile/kill")]
pub async fn kill(
    context: web::Data<Arc<Context>>,
    req: HttpRequest,
    data: web::Json<KillArgs>,
) -> Result<HttpResponse, Error> {
    let mut connection = context.redis_pool.get().map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Failed to get Redis connection: {}", e))
    })?;

    check_address(&req, data.address.clone())?;

    match super::service::kill(
        &mut connection,
        KillArgs {
            address: data.address.clone(),
            pid: data.pid.clone(),
        },
    ) {
        Ok(true) => {
            Ok(HttpResponse::Ok().json(json!({ "message": "Profile deleted successfully" })))
        }
        Ok(false) => {
            Ok(HttpResponse::BadRequest().json(json!({ "message": "Failed to delete profile" })))
        }
        Err(err) => Err(actix_web::error::ErrorInternalServerError(format!(
            "Service error: {}",
            err
        ))),
    }
}
