use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use chrono::Local;
use futures::future::LocalBoxFuture;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::{
    future::{ready, Ready},
    time::Instant,
};

// 日志记录器结构体
pub struct RequestLogger {
    log_file_path: String,
}

impl RequestLogger {
    pub fn new(log_file_path: String) -> Self {
        RequestLogger { log_file_path }
    }
}

// 中间件工厂实现
impl<S, B> Transform<S, ServiceRequest> for RequestLogger
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = RequestLoggerMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RequestLoggerMiddleware {
            service,
            log_file_path: self.log_file_path.clone(),
        }))
    }
}

// 中间件服务实现
pub struct RequestLoggerMiddleware<S> {
    service: S,
    log_file_path: String,
}

impl<S, B> Service<ServiceRequest> for RequestLoggerMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let start_time = Instant::now();
        let path = req.path().to_owned();
        let method = req.method().to_string();
        let headers = format!("{:?}", req.headers());
        let log_file_path = self.log_file_path.clone();

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            let duration = start_time.elapsed();
            let status = res.status().as_u16();
            let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();

            // 构建日志消息
            let log_message = format!(
                "[{}] {} {} - Status: {} - Duration: {:?}\nHeaders: {}\n",
                timestamp, method, path, status, duration, headers
            );

            // 控制台输出
            println!("{}", log_message);

            // 写入文件
            if let Ok(mut file) = OpenOptions::new()
                .create(true)
                .append(true)
                .open(&log_file_path)
            {
                if let Err(e) = writeln!(file, "{}", log_message) {
                    eprintln!("Failed to write to log file: {}", e);
                }
            }

            Ok(res)
        })
    }
}
