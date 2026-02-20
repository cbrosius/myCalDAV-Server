use axum::{
    routing::{get, post, put, delete, any},
    Router,
    middleware::from_fn,
    Extension,
};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::info;
use tower_http::trace::TraceLayer;
use tower_http::services::ServeDir;

mod config;
mod error;
mod handlers;
mod models;
mod services;
mod middleware;
mod state;
mod database;
mod templates;

pub use crate::config::Config;
pub use crate::error::AppError;
pub use crate::services::CalendarService;

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Load configuration
    let config = Config::from_env().unwrap_or_default();
    
    // Ensure data directory exists
    std::fs::create_dir_all("./data")?;
    
    // Create database connection pool
    let pool = sqlx::sqlite::SqlitePool::connect(&config.database_url).await?;
    
    // Initialize database
    database::initialize_database(&pool).await?;
    
    info!("Database initialized successfully");
    
    let service = services::CalendarService::new(pool);
    let auth_config = middleware::AuthConfig::new(config.jwt_secret.clone());
    
    // Build the application with routes
    let app = Router::new()
        // Public routes (no authentication required)
        .route("/", get(handlers::root))
        .route("/health", get(handlers::health))
        .route("/.well-known/caldav", get(handlers::caldav_discovery))
        .route("/api/auth/login", post(handlers::auth::login))
        .route("/api/auth/register", post(handlers::auth::register))
        // User routes
        .route("/api/users/{id}", get(handlers::get_user_by_id))
        // Calendar routes
        .route("/api/calendars/{id}", get(handlers::get_calendar_by_id))
        .route("/api/auth/calendars", get(handlers::auth::get_user_calendars).post(handlers::auth::create_calendar))
        .route("/api/auth/calendars/{id}", put(handlers::update_calendar).delete(handlers::delete_calendar))
        .route("/api/auth/calendars/{id}/events", get(handlers::auth::get_events))
        // Event routes
        .route("/api/events/{id}", get(handlers::get_event_by_id))
        .route("/api/auth/events", post(handlers::auth::create_event))
        .route("/api/auth/events/{id}", get(handlers::auth::get_event).put(handlers::update_event).delete(handlers::delete_event))
        // Share routes
        .route("/api/auth/calendars/{id}/shares", get(handlers::get_calendar_shares).post(handlers::create_share))
        .route("/api/auth/shares/{id}", delete(handlers::delete_share))
        // CalDAV routes (support both JWT and Basic Auth)
        .route("/calendars", any(handlers::caldav_propfind))
        .route("/calendars/", any(handlers::caldav_propfind))
        .route("/calendars/{id}", any(handlers::caldav_get))
        .route("/calendars/{id}/", any(handlers::caldav_get))
        .route("/calendars/{id}/{event}", any(handlers::caldav_get))
        // MKCOL for creating calendars via CalDAV
        .route("/calendars/new", axum::routing::method_routing::on(axum::http::Method::from_bytes(b"MKCOL").unwrap(), handlers::caldav_mkcol))
        // Web UI routes - Authentication
        .route("/web/login", get(handlers::web::login_page).post(handlers::web::login_handler))
        .route("/web/register", get(handlers::web::register_page).post(handlers::web::register_handler))
        .route("/web/logout", get(handlers::web::logout_handler))
        // Web UI routes - Dashboard
        .route("/web/dashboard", get(handlers::web::dashboard_page))
        // Web UI routes - Calendars
        .route("/web/calendars", get(handlers::web::calendars_page))
        .route("/web/calendars/new", get(handlers::web::new_calendar_page).post(handlers::web::create_calendar_handler))
        .route("/web/calendars/{id}", get(handlers::web::calendar_detail_page))
        .route("/web/calendars/{id}/edit", get(handlers::web::edit_calendar_page).post(handlers::web::update_calendar_handler))
        .route("/web/calendars/{id}/delete", post(handlers::web::delete_calendar_handler))
        // Web UI routes - Events
        .route("/web/events", get(handlers::web::events_page))
        .route("/web/events/new", get(handlers::web::new_event_page).post(handlers::web::create_event_handler))
        .route("/web/events/{id}/edit", get(handlers::web::edit_event_page).post(handlers::web::update_event_handler))
        .route("/web/events/{id}/delete", post(handlers::web::delete_event_handler))
        // Web UI routes - Shares
        .route("/web/calendars/{id}/shares", post(handlers::web::create_share_handler))
        .route("/web/shares/{id}/delete", post(handlers::web::delete_share_handler))
        // Static files
        .nest_service("/static", ServeDir::new("static"))
        .with_state(service)
        .layer(TraceLayer::new_for_http())
        .layer(from_fn(middleware::cors_middleware))
        .layer(from_fn(middleware::logging_middleware))
        .layer(from_fn(middleware::auth_middleware))
        .layer(Extension(auth_config));

    // Run server
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    info!("Listening on {}", addr);
    
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
