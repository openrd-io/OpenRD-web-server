#[macro_use]
extern crate openRD_web_server;
#[macro_use]
extern crate diesel;

use std::sync::Arc;

use actix_web::{middleware, App, HttpServer};
use actix_web_grants::GrantsMiddleware;
use openRD_web_server::handlers::{
    auth::extract_permissions_from_token, config::Config, db::DatabaseManager, logger::init_logger,
};

use openRD_web_server::log_info;
use openRD_web_server::routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 加载配置
    let config = Config::from_env().expect("Failed to load configuration");

    // 初始化日志系统
    init_logger(&config.log.level);

    // 初始化数据库连接池
    let db_manager = Arc::new(
        DatabaseManager::new(&config.database.url, config.database.pool_size)
            .expect("Failed to initialize database"),
    );

    // 健康检查
    db_manager
        .check_health()
        .expect("Database health check failed");

    log_info!("Starting HTTP server at http://{}", config.server_address());
    // 启动服务器
    let db_manager = db_manager.clone();
    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(db_manager.get_pool()))
            .wrap(middleware::Logger::default())
            .wrap(GrantsMiddleware::with_extractor(
                extract_permissions_from_token,
            ))
            .wrap(middleware::NormalizePath::trim())
            .configure(routes::api::configure_routes)
    })
    .bind(config.server_address())?
    .run()
    .await
}
