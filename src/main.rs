use actix_web::{web, App, HttpServer};
use std::sync::Arc;

use crate::config::init_envs;
use crate::context::Context;
use crate::health::service::health_check;
use crate::redis::Redis;

mod config;
mod context;
mod health;
mod redis;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_envs(".env");

    let redis_pool = Redis::get_pool();
    let context = Arc::new(Context { redis_pool });

    println!("Service is running");

    HttpServer::new(move || App::new().app_data(web::Data::new(context.clone())).service(health_check))
        .bind(("127.0.0.1", 3000))?
        .run()
        .await
}
