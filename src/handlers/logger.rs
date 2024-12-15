// src/logger.rs

use fern::Dispatch;
use futures::FutureExt;
use log::LevelFilter;
use chrono::Local;
use std::{boxed, io};
use actix_web::{dev::{Service, ServiceRequest, ServiceResponse, Transform}, Error};
use futures_util::future::{ready, Ready};
use std::task::{Context, Poll};

use crate::{log_error, log_info, utils::api_response::ApiResponse};

/// 初始化日志记录器，配置日志同时输出到控制台和文件
pub fn init_logger(log_file_path: &str, log_level: &str) {
    let logfile = fern::log_file(log_file_path)
        .unwrap_or_else(|_| panic!("无法创建日志文件: {}", log_file_path));

    let level = match log_level.to_lowercase().as_str() {
        "trace" => LevelFilter::Trace,
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        _ => LevelFilter::Info,
    };

    Dispatch::new()
        .level(level)
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} [{}] - {} - {}:{} - {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.target(),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                message
            ))
        })
        .chain(io::stdout())
        .chain(logfile)
        .apply()
        .expect("无法初始化日志记录器");
}

/// 自定义请求日志中间件
pub struct RequestLoggerMiddleware;

impl<S, B> Transform<S, ServiceRequest> for RequestLoggerMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = RequestLoggerMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RequestLoggerMiddlewareService { service }))
    }
}

pub struct RequestLoggerMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for RequestLoggerMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = futures_util::future::LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let start_time = Local::now();
        let method = req.method().to_string();
        let uri = req.uri().to_string();

        let fut = self.service.call(req);

        fut.map(move |res | {
            match res {
                Ok(res) => {  
                    let status = res.status().as_u16();
                    let duration = Local::now()
                    .signed_duration_since(start_time)
                    .num_milliseconds();

                    if status >= 200 && status < 300 {
                        log::info!(
                            "Request: {} {} - Status: {} - Duration: {}ms",
                            method,
                            uri,
                            status,
                            duration
                        );
                    } else {
                        log::error!(
                            "Request: {} {} - Status: {} - Duration: {}ms,msg={}",
                            method,
                            uri,
                            status,
                            duration,
                            format!("{:?}", res.response().error())
                        );
                        
                    }                      
                    Ok(res)

                },
                Err(err) => {
                    log_error!(
                        "Request: {} {} - Status: 500 - Duration: {}ms, err={}",
                        method,
                        uri,
                        Local::now()
                            .signed_duration_since(start_time)
                            .num_milliseconds(),
                        err.to_string()
                    );
                    Err(err)
                
            }
        }}).boxed_local().into()
    }
}
