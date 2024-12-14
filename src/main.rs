use std::sync::Arc;

use actix_web::{middleware, App, HttpServer};
use actix_web_grants::GrantsMiddleware;
use handlers::{
    auth::extract_permissions_from_token, config::Config, db::DatabaseManager, logger::init_logger,
};
mod handlers;
mod models;
mod routes;
mod schema;

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

    app_info!("Starting HTTP server at http://{}", config.server_address());
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
