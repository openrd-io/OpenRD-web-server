use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error, HttpResponse};
use actix_web::dev::{Transform, Service};
use actix_web::body::{BoxBody, MessageBody};
use futures_util::future::{ok, Ready, LocalBoxFuture};
use jsonwebtoken::{decode, encode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use std::rc::Rc;
use std::task::{Context, Poll};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    user_id: String,
    exp: usize,
}

pub struct Auth;

impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Transform = AuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddleware {
            service: Rc::new(service),
        })
    }
}

pub struct AuthMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);

        Box::pin(async move {
            if let Some(authen_header) = req.headers().get("Authorization") {
                if let Ok(authen_str) = authen_header.to_str() {
                    if authen_str.starts_with("Bearer ") {
                        let token = &authen_str[7..];
                        let secret = env::var("SECRET_KEY").unwrap_or_else(|_| "secret".to_string());
                        let token_data = decode::<Claims>(&token, &DecodingKey::from_secret(secret.as_ref()), &Validation::default());

                        if token_data.is_ok() {
                            let res = service.call(req).await?;
                            return Ok(res.map_into_boxed_body());
                        }
                    }
                }
            }
            let un_auth_body =serde_json::json!({"message": "Unauthorized","code":"10001"});
            Ok(req.into_response(HttpResponse::Unauthorized().body(un_auth_body.to_string()).map_into_boxed_body()))
        })
    }
}
