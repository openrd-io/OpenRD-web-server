// // redis.rs
// use deadpool_redis::{Config, Pool};
// use dotenv::dotenv;
// use lazy_static::lazy_static;
// use std::env;

// lazy_static! {
//     pub static ref REDIS_POOL: Pool = {
//         dotenv().ok();
//         let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");
//         let mut cfg = Config::from_url(redis_url);
//         cfg.create_pool(Some(deadpool_redis::Runtime::Tokio1)).expect("Failed to create Redis pool")
//     };
// }