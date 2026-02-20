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
use base64::{engine::general_purpose::STANDARD as BASE64_STANDARD, Engine};

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

/// Result of parsing Basic Auth credentials
#[derive(Debug, Clone)]
pub struct BasicAuthCredentials {
    pub email: String,
    pub password: String,
}

/// Parse Basic Auth header into credentials
fn parse_basic_auth(header_value: &str) -> Option<BasicAuthCredentials> {
    if !header_value.starts_with("Basic ") {
        return None;
    }
    
    let encoded = &header_value["Basic ".len()..];
    let decoded = BASE64_STANDARD.decode(encoded).ok()?;
    let decoded_str = String::from_utf8(decoded).ok()?;
    
    let parts: Vec<&str> = decoded_str.splitn(2, ':').collect();
    if parts.len() != 2 {
        return None;
    }
    
    Some(BasicAuthCredentials {
        email: parts[0].to_string(),
        password: parts[1].to_string(),
    })
}

/// Check if the path is a CalDAV endpoint that should support Basic Auth
fn is_caldav_endpoint(path: &str) -> bool {
    path.starts_with("/calendars") 
        || path == "/.well-known/caldav"
        || path.starts_with("/dav")
        || path.starts_with("/principals")
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
    
    // Extract Authorization header
    let auth_header = match req.headers().get(header::AUTHORIZATION) {
        Some(header) => header.to_str().unwrap_or_default().to_string(),
        None => {
            // For CalDAV endpoints, return 401 with WWW-Authenticate header
            if is_caldav_endpoint(path) {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .header("WWW-Authenticate", "Basic realm=\"CalDAV Server\"")
                    .body(axum::body::Body::from("Authentication required"))
                    .unwrap();
            }
            return (StatusCode::UNAUTHORIZED, "Missing token").into_response();
        }
    };
    
    // Try Bearer token first (for API endpoints)
    if auth_header.starts_with("Bearer ") {
        let token = auth_header["Bearer ".len()..].to_string();
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
                return next.run(req).await;
            }
            Err(e) => {
                info!("Token validation failed: {}", e);
                return (StatusCode::UNAUTHORIZED, "Invalid token").into_response();
            }
        }
    }
    
    // Try Basic Auth (primarily for CalDAV endpoints)
    if let Some(credentials) = parse_basic_auth(&auth_header) {
        // Store credentials in request extensions for handlers to use
        req.extensions_mut().insert(credentials);
        return next.run(req).await;
    }
    
    // No valid authentication method found
    if is_caldav_endpoint(path) {
        return Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .header("WWW-Authenticate", "Basic realm=\"CalDAV Server\"")
            .body(axum::body::Body::from("Authentication required"))
            .unwrap();
    }
    
    (StatusCode::UNAUTHORIZED, "Invalid authentication method").into_response()
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
