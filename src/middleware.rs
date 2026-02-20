use axum::{
    extract::Request,
    http::{header, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Extension,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Deserialize;
use tracing::info;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize)]
pub struct Claims {
    pub sub: String,  // Subject (user id)
    pub exp: usize,   // Expiration time
    pub iat: usize,   // Issued at
}

#[derive(Clone)]
pub struct AuthConfig {
    pub jwt_secret: String,
}

impl AuthConfig {
    pub fn new(jwt_secret: String) -> Self {
        Self { jwt_secret }
    }
}

pub async fn auth_middleware(
    Extension(auth_config): Extension<AuthConfig>,
    mut req: Request,
    next: Next,
) -> Response {
    // Skip authentication for certain routes
    let path = req.uri().path();
    let auth_required = !path.starts_with("/public") 
        && !path.starts_with("/health")
        && !path.starts_with("/api/auth/login")
        && !path.starts_with("/api/auth/register")
        && path != "/";
    
    if !auth_required {
        return next.run(req).await;
    }
    
    // Extract token from Authorization header
    let token = match req.headers().get(header::AUTHORIZATION) {
        Some(header) => {
            let header_str = header.to_str().unwrap_or_default();
            if header_str.starts_with("Bearer ") {
                Some(header_str["Bearer ".len()..].to_string())
            } else {
                None
            }
        }
        None => None,
    };

    match token {
        Some(token) => {
            let validation = Validation::new(jsonwebtoken::Algorithm::HS256);
            
            match decode::<Claims>(
                &token,
                &DecodingKey::from_secret(auth_config.jwt_secret.as_bytes()),
                &validation
            ) {
                Ok(decoded) => {
                    // Parse user_id from claims
                    let user_id = match Uuid::parse_str(&decoded.claims.sub) {
                        Ok(id) => id,
                        Err(_) => {
                            return (StatusCode::UNAUTHORIZED, "Invalid user ID in token").into_response();
                        }
                    };
                    
                    // Add user_id to request extensions
                    req.extensions_mut().insert(user_id);
                    next.run(req).await
                }
                Err(e) => {
                    info!("Token validation failed: {}", e);
                    (StatusCode::UNAUTHORIZED, "Invalid token").into_response()
                }
            }
        }
        None => {
            (StatusCode::UNAUTHORIZED, "Missing token").into_response()
        }
    }
}

/// Middleware to add CORS headers to responses
pub async fn cors_middleware(req: Request, next: Next) -> Response {
    let response = next.run(req).await;
    
    let (mut parts, body) = response.into_parts();
    
    parts.headers.insert(
        header::ACCESS_CONTROL_ALLOW_ORIGIN,
        "*".parse().unwrap(),
    );
    parts.headers.insert(
        header::ACCESS_CONTROL_ALLOW_METHODS,
        "GET, POST, PUT, DELETE, OPTIONS, PROPFIND, REPORT, MKCOL".parse().unwrap(),
    );
    parts.headers.insert(
        header::ACCESS_CONTROL_ALLOW_HEADERS,
        "Authorization, Content-Type, Accept, Depth, Prefer".parse().unwrap(),
    );
    
    Response::from_parts(parts, body)
}

/// Middleware for logging requests
pub async fn logging_middleware(req: Request, next: Next) -> Response {
    info!("Incoming request: {} {}", req.method(), req.uri().path());
    let response = next.run(req).await;
    info!("Response status: {}", response.status());
    response
}
