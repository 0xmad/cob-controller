use actix_web::{get, Error, HttpResponse};

#[get("/health")]
pub async fn health_check() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body(format!("Running")))
}
