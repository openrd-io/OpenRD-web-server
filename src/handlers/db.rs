use diesel::mysql::MysqlConnection;
use diesel::r2d2::{self, ConnectionManager};
use std::time::Duration;

use crate::handlers::error::AppError;
use crate::{log_error, log_info, log_warn};
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<MysqlConnection>>;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub struct DatabaseManager {
    pool: DbPool,
}

impl DatabaseManager {
    pub fn new(database_url: &str, max_size: u32) -> Result<Self, AppError> {
        log_info!("Initializing database connection pool...");

        let manager = ConnectionManager::<MysqlConnection>::new(database_url);
        let pool = r2d2::Pool::builder()
            .max_size(max_size)
            .connection_timeout(Duration::from_secs(30))
            .connection_customizer(Box::new(ConnectionOptions::default()))
            .test_on_check_out(true)
            .build(manager)
            .map_err(|e| {
                log_error!("Failed to create connection pool: {}", e);
                AppError::InternalServerError
            })?;

        // 运行数据库迁移
        let mut conn = pool.get().map_err(|e| {
            log_error!("Failed to get database connection: {}", e);
            AppError::InternalServerError
        })?;

        Self::run_migrations(&mut conn)?;

        log_info!("Database pool initialized with size: {}", max_size);
        Ok(Self { pool })
    }

    pub fn get_pool(&self) -> DbPool {
        self.pool.clone()
    }

    pub fn run_migrations(conn: &mut MysqlConnection) -> Result<(), AppError> {
        log_info!("Running database migrations...");

        conn.run_pending_migrations(MIGRATIONS).map_err(|e| {
            log_error!("Failed to run migrations: {}", e);
            AppError::InternalServerError
        })?;

        log_info!("Database migrations completed successfully");
        Ok(())
    }

    pub fn check_health(&self) -> Result<(), AppError> {
        let mut conn = self.pool.get().map_err(|e| {
            log_warn!("Health check failed - could not get connection: {}", e);
            AppError::InternalServerError
        })?;

        diesel::sql_query("SELECT 1")
            .execute(&mut *conn)
            .map_err(|e| {
                log_warn!("Health check failed - query failed: {}", e);
                AppError::InternalServerError
            })?;

        Ok(())
    }
}

// 自定义连接选项
#[derive(Debug)]
struct ConnectionOptions {
    timezone: String,
    charset: String,
}

impl Default for ConnectionOptions {
    fn default() -> Self {
        Self {
            timezone: "+08:00".to_string(),
            charset: "utf8mb4".to_string(),
        }
    }
}

impl r2d2::CustomizeConnection<MysqlConnection, diesel::r2d2::Error> for ConnectionOptions {
    fn on_acquire(&self, conn: &mut MysqlConnection) -> Result<(), diesel::r2d2::Error> {
        use diesel::sql_query;

        sql_query(&format!("SET time_zone = '{}'", self.timezone))
            .execute(conn)
            .map_err(diesel::r2d2::Error::QueryError)?;

        sql_query(&format!("SET NAMES {}", self.charset))
            .execute(conn)
            .map_err(diesel::r2d2::Error::QueryError)?;

        Ok(())
    }
}

#[cfg(test)]
pub fn initialize_test_database() -> Result<MysqlConnection, AppError> {
    use dotenv::dotenv;
    use std::env;

    dotenv().ok();
    let database_url =
        env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set for tests");

    let mut conn = MysqlConnection::establish(&database_url).map_err(|e| {
        log_error!("Failed to establish test database connection: {}", e);
        AppError::InternalServerError
    })?;

    DatabaseManager::run_migrations(&mut conn)?;
    Ok(conn)
}
