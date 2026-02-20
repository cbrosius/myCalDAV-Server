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

/// Wrapper for optional user ID from authentication
#[derive(Debug, Clone)]
pub struct OptionalUser(pub Option<Uuid>);

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
    let is_public_route = path.starts_with("/public") 
        || path.starts_with("/health")
        || path.starts_with("/api/auth/login")
        || path.starts_with("/api/auth/register")
        || path.starts_with("/web/login")
        || path.starts_with("/web/register")
        || path.starts_with("/static")
        || path == "/";
    
    // Check if this is a web route that requires authentication
    let is_web_route = path.starts_with("/web/") && !path.starts_with("/web/login") && !path.starts_with("/web/register");
    
    // Check if this is an API route that requires authentication
    let is_api_route = path.starts_with("/api/auth/") && !path.starts_with("/api/auth/login") && !path.starts_with("/api/auth/register");
    
    // Check if this is a CalDAV route
    let is_caldav = is_caldav_endpoint(path);
    
    let auth_required = is_web_route || is_api_route || is_caldav;
    
    // Try to get token from Authorization header or Cookie
    let token = if let Some(auth_header) = req.headers().get(header::AUTHORIZATION) {
        let auth_str = auth_header.to_str().unwrap_or_default();
        if auth_str.starts_with("Bearer ") {
            Some(auth_str["Bearer ".len()..].to_string())
        } else {
            None
        }
    } else if let Some(cookie_header) = req.headers().get(header::COOKIE) {
        // Parse cookie for auth_token
        let cookie_str = cookie_header.to_str().unwrap_or_default();
        parse_auth_cookie(cookie_str)
    } else {
        None
    };
    
    // Try to authenticate with token
    if let Some(token) = token {
        let validation = Validation::new(jsonwebtoken::Algorithm::HS256);
        
        match decode::<Claims>(
            &token,
            &DecodingKey::from_secret(auth_config.jwt_secret.as_bytes()),
            &validation
        ) {
            Ok(decoded) => {
                // Parse user_id from claims
                if let Ok(user_id) = Uuid::parse_str(&decoded.claims.sub) {
                    // Add user_id to request extensions
                    req.extensions_mut().insert(user_id);
                    req.extensions_mut().insert(OptionalUser(Some(user_id)));
                    return next.run(req).await;
                }
            }
            Err(e) => {
                info!("Token validation failed: {}", e);
            }
        }
    }
    
    // Try Basic Auth (primarily for CalDAV endpoints)
    if let Some(auth_header) = req.headers().get(header::AUTHORIZATION) {
        if let Some(credentials) = parse_basic_auth(auth_header.to_str().unwrap_or_default()) {
            // Store credentials in request extensions for handlers to use
            req.extensions_mut().insert(credentials);
            req.extensions_mut().insert(OptionalUser(None));
            return next.run(req).await;
        }
    }
    
    // Add OptionalUser(None) for unauthenticated requests
    req.extensions_mut().insert(OptionalUser(None));
    
    // Handle unauthenticated requests
    if !auth_required {
        return next.run(req).await;
    }
    
    // For CalDAV endpoints, return 401 with WWW-Authenticate header
    if is_caldav {
        return Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .header("WWW-Authenticate", "Basic realm=\"CalDAV Server\"")
            .body(axum::body::Body::from("Authentication required"))
            .unwrap();
    }
    
    // For web routes, redirect to login
    if is_web_route {
        return Response::builder()
            .status(StatusCode::FOUND)
            .header("Location", "/web/login")
            .body(axum::body::Body::empty())
            .unwrap();
    }
    
    // For API routes, return 401
    (StatusCode::UNAUTHORIZED, "Authentication required").into_response()
}

/// Parse auth_token from cookie string
fn parse_auth_cookie(cookie_str: &str) -> Option<String> {
    for cookie in cookie_str.split(';') {
        let cookie = cookie.trim();
        if cookie.starts_with("auth_token=") {
            return Some(cookie["auth_token=".len()..].to_string());
        }
    }
    None
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
