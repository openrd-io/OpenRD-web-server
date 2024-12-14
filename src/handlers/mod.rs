pub mod auth;
pub mod config;
pub mod db;
pub mod error;
pub mod logger;

// Re-export commonly used macros
pub use crate::{app_debug, app_error, app_info, app_warn};
