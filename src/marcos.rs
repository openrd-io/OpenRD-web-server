// src/macros.rs

// 定义一些常用的日志宏
#[macro_export]
macro_rules! log_error {
    ($($arg:tt)+) => ({
        log::error!("[APP_ERROR] {}", format_args!($($arg)+));
    })
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)+) => ({
        log::warn!("[APP_WARN] {}", format_args!($($arg)+));
    })
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)+) => ({
        log::info!("[APP_INFO] {}", format_args!($($arg)+));
    })
}

#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)+) => ({
        log::debug!("[APP_DEBUG] {}", format_args!($($arg)+));
    })
}
