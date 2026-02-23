use axum::{
    extract::{Path, State, Extension, Query},
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::models::*;
use crate::services::CalendarService;
use crate::error::AppError;
use crate::middleware::UserRoleExt;
use bcrypt::verify;
use jsonwebtoken::{encode, Header, EncodingKey};
use chrono::Utc;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserResponse,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub role: UserRole,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            name: user.name,
            role: user.role,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,   // Subject (user id)
    exp: usize,    // Expiration time
    iat: usize,    // Issued at
    role: Option<String>,  // User role
}

pub async fn login(
    State(service): State<CalendarService>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    tracing::info!("API login attempt for email: {}", payload.email);
    
    let user = match service.get_user_by_email(&payload.email).await? {
        Some(u) => u,
        None => {
            tracing::warn!("User not found: {}", payload.email);
            return Err(AppError::AuthenticationError("Invalid credentials".to_string()));
        }
    };
    
    tracing::info!("User found: {}, verifying password", user.email);

    match verify(&payload.password, &user.password_hash) {
        Ok(true) => {
            tracing::info!("Password verified for user: {}", user.email);
        }
        Ok(false) => {
            tracing::warn!("Invalid password for user: {}", user.email);
            return Err(AppError::AuthenticationError("Invalid credentials".to_string()));
        }
        Err(e) => {
            tracing::error!("Password verification error for user {}: {:?}", user.email, e);
            return Err(AppError::PasswordHashError(e));
        }
    }

    let now = Utc::now().timestamp() as usize;
    let claims = Claims {
        sub: user.id.to_string(),
        iat: now,
        exp: now + (24 * 60 * 60), // 24 hours
        role: Some(user.role.as_str().to_string()),
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(service.get_jwt_secret().as_bytes()),
    )?;

    tracing::info!("Login successful for user: {}", user.email);

    Ok(Json(LoginResponse { 
        token,
        user: UserResponse::from(user),
    }))
}

pub async fn register(
    State(service): State<CalendarService>,
    Json(payload): Json<NewUser>,
) -> Result<Json<UserResponse>, AppError> {
    // Check if user already exists
    if service.get_user_by_email(&payload.email).await?.is_some() {
        return Err(AppError::ValidationError("Email already registered".to_string()));
    }
    
    let user = service.create_user(payload).await?;
    Ok(Json(UserResponse::from(user)))
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct GetCalendarsParams {
    pub include_events: Option<bool>,
}

pub async fn get_user_calendars(
    State(service): State<CalendarService>,
    Extension(user_id): Extension<Uuid>,
    Query(_params): Query<GetCalendarsParams>,
) -> Result<Json<Vec<Calendar>>, AppError> {
    let calendars = service.get_calendars_by_user_id(user_id).await?;
    Ok(Json(calendars))
}

pub async fn create_calendar(
    State(service): State<CalendarService>,
    Extension(user_id): Extension<Uuid>,
    Json(payload): Json<NewCalendar>,
) -> Result<Json<Calendar>, AppError> {
    let calendar = service.create_calendar(user_id, payload).await?;
    Ok(Json(calendar))
}

#[derive(Debug, Deserialize)]
pub struct CreateEventRequest {
    pub calendar_id: Uuid,
    pub event: NewEvent,
}

pub async fn create_event(
    State(service): State<CalendarService>,
    Extension(user_id): Extension<Uuid>,
    Json(payload): Json<CreateEventRequest>,
) -> Result<Json<Event>, AppError> {
    // Validate user owns the calendar
    let calendar = service.get_calendar_by_id(payload.calendar_id).await?.ok_or(
        AppError::NotFoundError("Calendar not found".to_string()))?;
    
    if calendar.user_id != user_id {
        return Err(AppError::AuthenticationError("You don't own this calendar".to_string()));
    }
    
    let event = service.create_event(payload.calendar_id, payload.event).await?;
    Ok(Json(event))
}

pub async fn get_event(
    State(service): State<CalendarService>,
    Extension(user_id): Extension<Uuid>,
    Path(event_id): Path<Uuid>,
) -> Result<Json<Event>, AppError> {
    let event = service.get_event_by_id(event_id).await?.ok_or(
        AppError::NotFoundError("Event not found".to_string()))?;
    
    // Check if user has access to this event
    let calendar = service.get_calendar_by_id(event.calendar_id).await?.ok_or(
        AppError::NotFoundError("Calendar not found".to_string()))?;
    
    if calendar.user_id != user_id {
        return Err(AppError::AuthenticationError("You don't have access to this event".to_string()));
    }
    
    Ok(Json(event))
}

pub async fn get_events(
    State(service): State<CalendarService>,
    Extension(user_id): Extension<Uuid>,
    Path(calendar_id): Path<Uuid>,
) -> Result<Json<Vec<Event>>, AppError> {
    let calendar = service.get_calendar_by_id(calendar_id).await?.ok_or(
        AppError::NotFoundError("Calendar not found".to_string()))?;
    
    if calendar.user_id != user_id {
        return Err(AppError::AuthenticationError("You don't have access to this calendar".to_string()));
    }
    
    let events = service.get_events_by_calendar_id(calendar_id).await?;
    Ok(Json(events))
}

// Admin-only endpoints

/// Get all users (admin only)
pub async fn admin_get_all_users(
    State(service): State<CalendarService>,
    Extension(_user_id): Extension<Uuid>,
    Extension(role): Extension<UserRoleExt>,
) -> Result<Json<Vec<UserResponse>>, AppError> {
    if !role.is_admin() {
        return Err(AppError::AuthenticationError("Admin access required".to_string()));
    }
    
    let users = service.get_all_users().await?;
    Ok(Json(users.into_iter().map(UserResponse::from).collect()))
}

/// Create user with specific role (admin only)
#[derive(Debug, Deserialize)]
pub struct AdminCreateUserRequest {
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: Option<String>,
}

pub async fn admin_create_user(
    State(service): State<CalendarService>,
    Extension(_user_id): Extension<Uuid>,
    Extension(role): Extension<UserRoleExt>,
    Json(payload): Json<AdminCreateUserRequest>,
) -> Result<Json<UserResponse>, AppError> {
    if !role.is_admin() {
        return Err(AppError::AuthenticationError("Admin access required".to_string()));
    }
    
    // Check if user already exists
    if service.get_user_by_email(&payload.email).await?.is_some() {
        return Err(AppError::ValidationError("Email already registered".to_string()));
    }
    
    let new_user = NewUser {
        name: payload.name,
        email: payload.email,
        password: payload.password,
    };
    
    let user_role = payload.role
        .map(|r| UserRole::from_str(&r))
        .unwrap_or_default();
    
    let user = service.create_user_with_role(new_user, user_role).await?;
    Ok(Json(UserResponse::from(user)))
}

/// Update user role (admin only)
#[derive(Debug, Deserialize)]
pub struct AdminUpdateRoleRequest {
    pub role: String,
}

pub async fn admin_update_user_role(
    State(service): State<CalendarService>,
    Extension(_user_id): Extension<Uuid>,
    Extension(role): Extension<UserRoleExt>,
    Path(target_user_id): Path<Uuid>,
    Json(payload): Json<AdminUpdateRoleRequest>,
) -> Result<Json<UserResponse>, AppError> {
    if !role.is_admin() {
        return Err(AppError::AuthenticationError("Admin access required".to_string()));
    }
    
    let new_role = UserRole::from_str(&payload.role);
    let user = service.update_user_role(target_user_id, new_role).await?;
    Ok(Json(UserResponse::from(user)))
}

/// Delete user (admin only)
pub async fn admin_delete_user(
    State(service): State<CalendarService>,
    Extension(_user_id): Extension<Uuid>,
    Extension(role): Extension<UserRoleExt>,
    Path(target_user_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, AppError> {
    if !role.is_admin() {
        return Err(AppError::AuthenticationError("Admin access required".to_string()));
    }
    
    service.delete_user(target_user_id).await?;
    Ok(Json(serde_json::json!({ "success": true, "message": "User deleted" })))
}
