use std::collections::HashSet;
use std::env;

use actix_web::{dev::ServiceRequest, Error};
use actix_web_httpauth::extractors::bearer::{self, BearerAuth};
use actix_web_httpauth::extractors::AuthenticationError;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

use crate::log_error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // user id
    pub role: String, // role
    pub exp: usize,   // expiration time
}

// 权限提取器
pub async fn extract_permissions_from_token(
    req: &ServiceRequest,
) -> Result<HashSet<String>, Error> {
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                let token = auth_str.trim_start_matches("Bearer ");
                let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
                let key = DecodingKey::from_secret(secret.as_bytes());

                match decode::<Claims>(token, &key, &Validation::new(Algorithm::HS256)) {
                    Ok(token_data) => {
                        // 从 token 中提取角色，并转换为权限列表
                        let role = if token_data.claims.role.is_empty() {
                            "USER".to_string()
                        } else {
                            token_data.claims.role
                        };
                        return Ok(HashSet::from([role]));
                    }
                    Err(_) => {
                        // 如果 token 无效，返回 401 提示无权限
                        let config = req
                            .app_data::<bearer::Config>()
                            .cloned()
                            .unwrap_or_default();
                        return Err(AuthenticationError::from(config).into());
                    }
                }
            }
        }
    }
    Ok(HashSet::from([])) // 如果没有token或token无效，返回空权限列表
}

// 生成带角色的token
pub fn generate_token(user_id: &str, role: &str) -> Result<String, jsonwebtoken::errors::Error> {
    use jsonwebtoken::{encode, EncodingKey, Header};
    use std::time::{SystemTime, UNIX_EPOCH};

    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize
        + 24 * 3600;

    let claims = Claims {
        sub: user_id.to_string(),
        role: role.to_owned(),
        exp: expiration,
    };

    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

pub async fn validate_token(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let config = req
        .app_data::<bearer::Config>()
        .cloned()
        .unwrap_or_default();

    let token = credentials.token();

    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let key = DecodingKey::from_secret(secret.as_bytes());

    match decode::<Claims>(token, &key, &Validation::new(Algorithm::HS256)) {
        Ok(_claims) => Ok(req),
        Err(e) => {
            log_error!("Token validation failed: {}", e);
            Err((AuthenticationError::from(config).into(), req))
        }
    }
}
