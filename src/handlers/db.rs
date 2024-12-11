use diesel::r2d2::ConnectionManager;
use diesel::mysql::MysqlConnection;
use diesel::r2d2::Pool;
use std::env;


pub fn init_pool() -> Pool<ConnectionManager<MysqlConnection>>  {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}