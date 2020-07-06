#[macro_use]
extern crate redis_async;
extern crate reqwest;

mod util;
mod structure;
mod common;

use common::contest::*;
use actix_web::{web, App, HttpServer, middleware};
use listenfd::ListenFd;
use actix_redis::RedisActor;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=trace,actix_redis=trace");
    env_logger::init();
    let mut listenfd = ListenFd::from_env();

    let redis_addr = RedisActor::start("127.0.0.1:6379");

    let mut server = 
        HttpServer::new(move|| App::new().data(redis_addr.clone())
                            .wrap(middleware::Logger::default())
                            .service(web::scope("/contest").configure(contest_config)));

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:8088")?
    };

    server.run().await
}