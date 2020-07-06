use crate::util::codeforces::*;
use crate::structure::contest::Contest;
use actix_web::{web, HttpResponse, Error as AWError};
use actix::prelude::*;
use futures::future::join_all;
use redis_async::resp::RespValue;
use actix_redis::{Command, RedisActor};

async fn request_incomming_contests() -> Vec<Contest> {
    let mut contests: Vec<Contest> = vec![];

    contests.append(&mut request_incomming_contests_codeforces().await);

    contests
}

async fn get_incomming_contests(redis: web::Data<Addr<RedisActor>>) -> Result<HttpResponse, AWError> {
    let res = redis.send(Command(resp_array!["SMEMBERS", "incomming_contest"])).await?;
    let mut contests: Vec<Contest> = vec![]; 

    match res {
        Ok(RespValue::Array(xs)) => {
            if xs.len() == 0 {
                contests = request_incomming_contests().await;
                contests.sort_by(|a, b| a.start_time_seconds.cmp(&b.start_time_seconds));
                
                let res: Vec<Result<RespValue, AWError>> = 
                    join_all(contests.clone().into_iter().map(|contest| {
                        redis.send(Command(resp_array!["SADD", "incomming_contest", serde_json::to_string(&contest).unwrap()]))
                    })).await.into_iter().map(|item| {
                        item.map_err(AWError::from)
                            .and_then(|res| res.map_err(AWError::from))
                    }).collect();
    
                if !res.iter().all(|res| match res {
                    Ok(RespValue::Integer(x)) if *x == 1 => true,
                     _ => false,
                }) {
                    return Ok(HttpResponse::InternalServerError().finish());
                }

                let expire_set = redis.send(Command(resp_array!["EXPIRE", "incomming_contest", 600])).await??;

                match expire_set {
                    RespValue::Integer(0) => {
                        return Ok(HttpResponse::InternalServerError().finish());
                    }
                    _ => {}
                }
            } else {
                for x in xs {
                    match x {
                        RespValue::BulkString(s) => {
                            let s = String::from_utf8(s).unwrap();
                            contests.push(serde_json::from_str(&s)?);
                        }
                        _ => {
                            println!("---->{:?}", x);
                            return Ok(HttpResponse::InternalServerError().finish());
                        }
                    }
                }
            }
        }
        _ => {
            return Ok(HttpResponse::InternalServerError().finish());
        }
    };

    let response: Vec<String> = contests.into_iter().map(|o| { serde_json::to_string(&o).unwrap() }).collect();
    Ok(HttpResponse::Ok().body(serde_json::to_string(&response)?))
}

pub fn contest_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/incomming")
            .route(web::get().to(get_incomming_contests))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    );
}