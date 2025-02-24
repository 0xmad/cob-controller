use actix_web::{web, App, HttpServer};
use std::sync::Arc;

use crate::config::init_envs;
use crate::context::Context;
use crate::health::service::health_check;
use crate::queue::RabbitMQ;
use crate::redis::Redis;

mod address;
mod config;
mod context;
mod health;
mod profile;
mod queue;
mod redis;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_envs(".env");

    let channel = RabbitMQ::connect()
        .await
        .expect("Can't start RabbitMQ consumer");

    let redis_pool = Redis::get_pool();
    let server_context = Arc::new(Context { redis_pool });
    let mq_context = Arc::new(server_context.clone());

    actix_rt::spawn(async move {
        if let Err(e) = RabbitMQ::consume(&mq_context.clone(), channel).await {
            eprintln!("Error in consumer: {:?}", e);
        }
    });

    println!("Service is running");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(server_context.clone()))
            .service(health_check)
            .service(profile::controller::create)
            .service(profile::controller::kill)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
