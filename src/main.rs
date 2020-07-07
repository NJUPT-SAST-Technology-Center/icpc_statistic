#[macro_use]
extern crate redis_async;
extern crate reqwest;

mod util;
mod structure;
mod common;

use std::env;
use dotenv::dotenv;
use common::contest::*;
use actix_web::{web, App, HttpServer, middleware};
use listenfd::ListenFd;
use actix_redis::RedisActor;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    let mut listenfd = ListenFd::from_env();

    let redis_host = env::var("REDIS_HOST").expect("REDIS_HOST is not set in .env file");
    let redis_port = env::var("REDIS_PORT").expect("REDIS_PORT is not set in .env file");

    let redis_addr = RedisActor::start(format!("{}:{}", redis_host, redis_port));

    let mut server = 
        HttpServer::new(move|| App::new().data(redis_addr.clone())
                            .wrap(middleware::Logger::default())
                            .service(web::scope("/contest").configure(contest_config)));

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        let host = env::var("HOST").expect("HOST is not set in .env file");
        let port = env::var("PORT").expect("PORT is not set in .env file");
        server.bind(format!("{}:{}", host, port))?
    };

    server.run().await
}