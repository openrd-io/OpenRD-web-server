use chrono::Local;
use env_logger::{Builder, Target};
use futures_util::future::LocalBoxFuture;
use log::LevelFilter;
use std::io::Write;

// 用于记录请求和响应的中间件
use actix_web::dev::Service;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::Error;
use std::future::{ready, Ready};

use crate::log_info;

pub fn init_logger(log_level: &str) {
    let level = match log_level.to_lowercase().as_str() {
        "trace" => LevelFilter::Trace,
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        _ => LevelFilter::Info,
    };

    Builder::new()
        .target(Target::Stdout)
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {} - {}:{} - {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.target(),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                record.args()
            )
        })
        .filter(None, level)
        .init();

    log::info!("Logger initialized with level: {}", log_level);
}

pub struct RequestLogger;

impl<S> actix_web::dev::Transform<S, ServiceRequest> for RequestLogger
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type Transform = RequestLoggerMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RequestLoggerMiddleware { service }))
    }
}

pub struct RequestLoggerMiddleware<S> {
    service: S,
}

impl<S> Service<ServiceRequest> for RequestLoggerMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let start_time = Local::now();
        let method = req.method().to_string();
        let uri = req.uri().to_string();

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            let duration = Local::now()
                .signed_duration_since(start_time)
                .num_milliseconds();

            log_info!(
                "Request: {} {} - Status: {} - Duration: {}ms",
                method,
                uri,
                res.status().as_u16(),
                duration
            );

            Ok(res)
        })
    }
}
