
pub mod auth;
pub mod db;
pub mod config;
pub mod error;
pub mod logger;

// Re-export commonly used macros
pub use crate::{app_error, app_warn, app_info, app_debug};