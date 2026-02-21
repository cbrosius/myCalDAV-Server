use axum::{
    extract::{Form, Path, Query, State, Extension},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect, Response},
};
use serde::Deserialize;
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{Duration, Utc};
use dioxus::prelude::*;

use crate::services::CalendarService;
use crate::error::AppError;
use crate::models::{NewCalendar, NewEvent, NewShare, NewUser, UpdateCalendar, UpdateEvent};
use crate::middleware::OptionalUser;
use crate::ui::*;

/// Query parameters for flash messages
#[derive(Debug, Deserialize)]
pub struct FlashQuery {
    pub message: Option<String>,
    pub flash_type: Option<String>,
}

/// Query parameters for event filtering
#[derive(Debug, Deserialize)]
pub struct EventFilterQuery {
    pub calendar: Option<Uuid>,
}

/// Login form data
#[derive(Debug, Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

/// Register form data
#[derive(Debug, Deserialize)]
pub struct RegisterForm {
    pub name: String,
    pub email: String,
    pub password: String,
    #[serde(rename = "confirm_password")]
    pub confirm_password: String,
}

/// Calendar form data
#[derive(Debug, Deserialize)]
pub struct CalendarFormInput {
    pub name: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub is_public: Option<String>,
}

/// Event form data
#[derive(Debug, Deserialize)]
pub struct EventFormInput {
    pub title: String,
    pub calendar_id: Uuid,
    pub start_time: String,
    pub end_time: String,
    pub is_all_day: Option<String>,
    pub location: Option<String>,
    pub description: Option<String>,
}

/// Share form data
#[derive(Debug, Deserialize)]
pub struct ShareFormInput {
    pub shared_with_email: String,
    pub permission: String,
}

// Helper function to render Dioxus component to HTML using dioxus_ssr
fn render_to_html(element: Element) -> Result<String, AppError> {
    Ok(dioxus_ssr::render_element(element))
}

// ============== Login/Register Pages ==============

/// Show login page
pub async fn login_page(
    Extension(user): Extension<OptionalUser>,
    Query(query): Query<FlashQuery>,
) -> Result<Html<String>, AppError> {
    // If already logged in, redirect to dashboard
    if user.0.is_some() {
        return Ok(Html("<script>window.location.href='/web/dashboard';</script>".to_string()));
    }
    
    let html = render_to_html(
        rsx! {
            LoginPage { 
                flash_message: query.message,
                flash_type: query.flash_type
            }
        }
    )?;
    
    Ok(Html(html))
}

/// Handle login form submission
pub async fn login_handler(
    State(service): State<CalendarService>,
    Form(form): Form<LoginForm>,
) -> Result<Response, AppError> {
    tracing::info!("Login attempt for email: {}", form.email);
    
    let user = match service.get_user_by_email(&form.email).await? {
        Some(u) => u,
        None => {
            tracing::warn!("User not found: {}", form.email);
            return Ok(Redirect::to("/web/login?message=Invalid credentials&flash_type=error").into_response());
        }
    };
    
    tracing::info!("User found: {}", user.email);
    
    // Verify password
    let valid = match bcrypt::verify(&form.password, &user.password_hash) {
        Ok(v) => v,
        Err(e) => {
            tracing::error!("Password verification error: {:?}", e);
            return Ok(Redirect::to("/web/login?message=Invalid credentials&flash_type=error").into_response());
        }
    };
    
    if !valid {
        tracing::warn!("Invalid password for user: {}", form.email);
        return Ok(Redirect::to("/web/login?message=Invalid credentials&flash_type=error").into_response());
    }
    
    tracing::info!("Password verified for user: {}", form.email);
    
    // Generate JWT token
    let token = service.generate_jwt(user.id)?;
    
    tracing::info!("JWT generated, redirecting to dashboard");
    
    // Set cookie and redirect
    Ok(Response::builder()
        .status(StatusCode::FOUND)
        .header("Location", "/web/dashboard")
        .header("Set-Cookie", format!("auth_token={}; Path=/; HttpOnly; SameSite=Strict", token))
        .body(axum::body::Body::empty())
        .unwrap()
        .into_response())
}

/// Show register page
pub async fn register_page(
    Extension(user): Extension<OptionalUser>,
    Query(query): Query<FlashQuery>,
) -> Result<Html<String>, AppError> {
    // If already logged in, redirect to dashboard
    if user.0.is_some() {
        return Ok(Html("<script>window.location.href='/web/dashboard';</script>".to_string()));
    }
    
    let html = render_to_html(
        rsx! {
            RegisterPage { 
                flash_message: query.message,
                flash_type: query.flash_type
            }
        }
    )?;
    
    Ok(Html(html))
}

/// Handle register form submission
pub async fn register_handler(
    State(service): State<CalendarService>,
    Form(form): Form<RegisterForm>,
) -> Result<Response, AppError> {
    // Validate passwords match
    if form.password != form.confirm_password {
        return Ok(Redirect::to("/web/register?message=Passwords do not match&flash_type=error").into_response());
    }
    
    // Validate password length
    if form.password.len() < 6 {
        return Ok(Redirect::to("/web/register?message=Password must be at least 6 characters&flash_type=error").into_response());
    }
    
    // Check if user already exists
    if service.get_user_by_email(&form.email).await?.is_some() {
        return Ok(Redirect::to("/web/register?message=Email already registered&flash_type=error").into_response());
    }
    
    // Create user
    let new_user = NewUser {
        name: form.name,
        email: form.email,
        password: form.password,
    };
    
    let user = service.create_user(new_user).await?;
    
    // Generate JWT token
    let token = service.generate_jwt(user.id)?;
    
    // Set cookie and redirect
    Ok(Response::builder()
        .status(StatusCode::FOUND)
        .header("Location", "/web/dashboard")
        .header("Set-Cookie", format!("auth_token={}; Path=/; HttpOnly; SameSite=Strict", token))
        .body(axum::body::Body::empty())
        .unwrap()
        .into_response())
}

/// Handle logout
pub async fn logout_handler() -> Response {
    Response::builder()
        .status(StatusCode::FOUND)
        .header("Location", "/web/login")
        .header("Set-Cookie", "auth_token=; Path=/; HttpOnly; SameSite=Strict; Max-Age=0")
        .body(axum::body::Body::empty())
        .unwrap()
}

// ============== Dashboard ==============

/// Show dashboard page
pub async fn dashboard_page(
    State(service): State<CalendarService>,
    Extension(user): Extension<Uuid>,
) -> Result<Html<String>, AppError> {
    let user_model = service.get_user_by_id(user).await?
        .ok_or_else(|| AppError::AuthenticationError("User not found".to_string()))?;
    
    let calendars = service.get_calendars_by_user_id(user).await?;
    let calendar_count = calendars.len();
    
    // Get all events and count
    let mut all_events = Vec::new();
    for cal in &calendars {
        let events = service.get_events_by_calendar_id(cal.id).await?;
        all_events.extend(events);
    }
    
    // Get upcoming events (next 7 days)
    let now = Utc::now();
    let week_later = now + Duration::days(7);
    let upcoming_events: Vec<_> = all_events
        .iter()
        .filter(|e| e.start_time >= now && e.start_time <= week_later)
        .take(10)
        .cloned()
        .collect();
    
    let event_count = all_events.len();
    
    // Count shares
    let mut share_count = 0;
    for cal in &calendars {
        let shares = service.get_shares_by_calendar_id(cal.id).await?;
        share_count += shares.len();
    }
    
    let html = render_to_html(
        rsx! {
            DashboardPage {
                current_user: user_model,
                calendar_count: calendar_count,
                event_count: event_count,
                share_count: share_count,
                calendars: calendars,
                upcoming_events: upcoming_events,
                caldav_url: "/".to_string(),
            }
        }
    )?;
    
    Ok(Html(html))
}

// ============== Calendar Pages ==============

/// Show calendars list page
pub async fn calendars_page(
    State(service): State<CalendarService>,
    Extension(user): Extension<Uuid>,
) -> Result<Html<String>, AppError> {
    let user_model = service.get_user_by_id(user).await?
        .ok_or_else(|| AppError::AuthenticationError("User not found".to_string()))?;
    
    let calendars = service.get_calendars_by_user_id(user).await?;
    
    // Get event counts
    let mut event_counts = HashMap::new();
    for cal in &calendars {
        let events = service.get_events_by_calendar_id(cal.id).await?;
        event_counts.insert(cal.id, events.len());
    }
    
    let html = render_to_html(
        rsx! {
            CalendarsPage {
                current_user: user_model,
                calendars: calendars,
                event_counts: event_counts,
            }
        }
    )?;
    
    Ok(Html(html))
}

/// Show new calendar form
pub async fn new_calendar_page(
    State(service): State<CalendarService>,
    Extension(user): Extension<Uuid>,
) -> Result<Html<String>, AppError> {
    let user_model = service.get_user_by_id(user).await?
        .ok_or_else(|| AppError::AuthenticationError("User not found".to_string()))?;
    
    let html = render_to_html(
        rsx! {
            CalendarFormPage {
                current_user: user_model,
                is_edit: false,
                calendar_id: None,
                calendar: None,
            }
        }
    )?;
    
    Ok(Html(html))
}

/// Handle new calendar form submission
pub async fn create_calendar_handler(
    State(service): State<CalendarService>,
    Extension(user): Extension<Uuid>,
    Form(form): Form<CalendarFormInput>,
) -> Result<Response, AppError> {
    let new_calendar = NewCalendar {
        name: form.name,
        description: form.description,
        color: form.color,
        is_public: form.is_public == Some("on".to_string()),
    };
    
    let calendar = service.create_calendar(user, new_calendar).await?;
    
    Ok(Redirect::to(&format!("/web/calendars/{}", calendar.id)).into_response())
}

/// Show calendar detail page
pub async fn calendar_detail_page(
    State(service): State<CalendarService>,
    Extension(user): Extension<Uuid>,
    Path(calendar_id): Path<Uuid>,
    Query(query): Query<FlashQuery>,
) -> Result<Html<String>, AppError> {
    let user_model = service.get_user_by_id(user).await?
        .ok_or_else(|| AppError::AuthenticationError("User not found".to_string()))?;
    
    let calendar = service.get_calendar_by_id(calendar_id).await?
        .ok_or_else(|| AppError::NotFoundError("Calendar not found".to_string()))?;
    
    // Check ownership
    if calendar.user_id != user {
        return Err(AppError::AuthenticationError("Access denied".to_string()));
    }
    
    let events = service.get_events_by_calendar_id(calendar_id).await?;
    let shares = service.get_shares_by_calendar_id(calendar_id).await?;
    
    let html = render_to_html(
        rsx! {
            CalendarDetailPage {
                current_user: user_model,
                calendar: calendar,
                events: events,
                shares: shares,
                caldav_url: "/".to_string(),
                flash_message: query.message,
                flash_type: query.flash_type,
            }
        }
    )?;
    
    Ok(Html(html))
}

/// Show edit calendar form
pub async fn edit_calendar_page(
    State(service): State<CalendarService>,
    Extension(user): Extension<Uuid>,
    Path(calendar_id): Path<Uuid>,
) -> Result<Html<String>, AppError> {
    let user_model = service.get_user_by_id(user).await?
        .ok_or_else(|| AppError::AuthenticationError("User not found".to_string()))?;
    
    let calendar = service.get_calendar_by_id(calendar_id).await?
        .ok_or_else(|| AppError::NotFoundError("Calendar not found".to_string()))?;
    
    // Check ownership
    if calendar.user_id != user {
        return Err(AppError::AuthenticationError("Access denied".to_string()));
    }
    
    let html = render_to_html(
        rsx! {
            CalendarFormPage {
                current_user: user_model,
                is_edit: true,
                calendar_id: Some(calendar_id),
                calendar: Some(calendar),
            }
        }
    )?;
    
    Ok(Html(html))
}

/// Handle edit calendar form submission
pub async fn update_calendar_handler(
    State(service): State<CalendarService>,
    Extension(user): Extension<Uuid>,
    Path(calendar_id): Path<Uuid>,
    Form(form): Form<CalendarFormInput>,
) -> Result<Response, AppError> {
    let calendar = service.get_calendar_by_id(calendar_id).await?
        .ok_or_else(|| AppError::NotFoundError("Calendar not found".to_string()))?;
    
    // Check ownership
    if calendar.user_id != user {
        return Err(AppError::AuthenticationError("Access denied".to_string()));
    }
    
    let update = UpdateCalendar {
        name: Some(form.name),
        description: form.description,
        color: form.color,
        is_public: Some(form.is_public == Some("on".to_string())),
    };
    
    service.update_calendar(calendar_id, update).await?;
    
    Ok(Redirect::to(&format!("/web/calendars/{}?message=Calendar updated&flash_type=success", calendar_id)).into_response())
}

/// Handle delete calendar
pub async fn delete_calendar_handler(
    State(service): State<CalendarService>,
    Extension(user): Extension<Uuid>,
    Path(calendar_id): Path<Uuid>,
) -> Result<Response, AppError> {
    let calendar = service.get_calendar_by_id(calendar_id).await?
        .ok_or_else(|| AppError::NotFoundError("Calendar not found".to_string()))?;
    
    // Check ownership
    if calendar.user_id != user {
        return Err(AppError::AuthenticationError("Access denied".to_string()));
    }
    
    service.delete_calendar(calendar_id).await?;
    
    Ok(Redirect::to("/web/calendars?message=Calendar deleted&flash_type=success").into_response())
}

// ============== Event Pages ==============

/// Show events list page
pub async fn events_page(
    State(service): State<CalendarService>,
    Extension(user): Extension<Uuid>,
    Query(query): Query<EventFilterQuery>,
) -> Result<Html<String>, AppError> {
    let user_model = service.get_user_by_id(user).await?
        .ok_or_else(|| AppError::AuthenticationError("User not found".to_string()))?;
    
    let calendars = service.get_calendars_by_user_id(user).await?;
    let calendar_names: HashMap<Uuid, String> = calendars
        .iter()
        .map(|c| (c.id, c.name.clone()))
        .collect();
    
    // Get all events from user's calendars
    let mut all_events = Vec::new();
    for cal in &calendars {
        let events = service.get_events_by_calendar_id(cal.id).await?;
        all_events.extend(events);
    }
    
    // Filter by calendar if specified
    let filtered_events: Vec<_> = if let Some(cal_id) = query.calendar {
        all_events
            .iter()
            .filter(|e| e.calendar_id == cal_id)
            .cloned()
            .collect()
    } else {
        all_events
    };
    
    let html = render_to_html(
        rsx! {
            EventsPage {
                current_user: user_model,
                events: filtered_events,
                calendars: calendars,
                calendar_names: calendar_names,
                selected_calendar: query.calendar,
            }
        }
    )?;
    
    Ok(Html(html))
}

/// Show new event form
pub async fn new_event_page(
    State(service): State<CalendarService>,
    Extension(user): Extension<Uuid>,
    Query(query): Query<EventFilterQuery>,
) -> Result<Html<String>, AppError> {
    let user_model = service.get_user_by_id(user).await?
        .ok_or_else(|| AppError::AuthenticationError("User not found".to_string()))?;
    
    let calendars = service.get_calendars_by_user_id(user).await?;
    
    let html = render_to_html(
        rsx! {
            EventFormPage {
                current_user: user_model,
                is_edit: false,
                event_id: None,
                event: None,
                calendars: calendars,
                selected_calendar_id: query.calendar,
            }
        }
    )?;
    
    Ok(Html(html))
}

/// Handle new event form submission
pub async fn create_event_handler(
    State(service): State<CalendarService>,
    Extension(user): Extension<Uuid>,
    Form(form): Form<EventFormInput>,
) -> Result<Response, AppError> {
    // Verify calendar ownership
    let calendar = service.get_calendar_by_id(form.calendar_id).await?
        .ok_or_else(|| AppError::NotFoundError("Calendar not found".to_string()))?;
    
    if calendar.user_id != user {
        return Err(AppError::AuthenticationError("Access denied".to_string()));
    }
    
    // Parse datetime
    let start_time = chrono::NaiveDateTime::parse_from_str(&form.start_time, "%Y-%m-%dT%H:%M")
        .map(|dt| dt.and_utc())
        .map_err(|_| AppError::ValidationError("Invalid start time format".to_string()))?;
    
    let end_time = chrono::NaiveDateTime::parse_from_str(&form.end_time, "%Y-%m-%dT%H:%M")
        .map(|dt| dt.and_utc())
        .map_err(|_| AppError::ValidationError("Invalid end time format".to_string()))?;
    
    let new_event = NewEvent {
        title: form.title,
        description: form.description,
        location: form.location,
        start_time,
        end_time,
        is_all_day: form.is_all_day == Some("on".to_string()),
    };
    
    let event = service.create_event(form.calendar_id, new_event).await?;
    
    Ok(Redirect::to(&format!("/web/calendars/{}?message=Event created&flash_type=success", event.calendar_id)).into_response())
}

/// Show edit event form
pub async fn edit_event_page(
    State(service): State<CalendarService>,
    Extension(user): Extension<Uuid>,
    Path(event_id): Path<Uuid>,
) -> Result<Html<String>, AppError> {
    let user_model = service.get_user_by_id(user).await?
        .ok_or_else(|| AppError::AuthenticationError("User not found".to_string()))?;
    
    let event = service.get_event_by_id(event_id).await?
        .ok_or_else(|| AppError::NotFoundError("Event not found".to_string()))?;
    
    let calendar = service.get_calendar_by_id(event.calendar_id).await?
        .ok_or_else(|| AppError::NotFoundError("Calendar not found".to_string()))?;
    
    // Check ownership
    if calendar.user_id != user {
        return Err(AppError::AuthenticationError("Access denied".to_string()));
    }
    
    let calendars = service.get_calendars_by_user_id(user).await?;
    let selected_calendar_id = event.calendar_id;
    
    let html = render_to_html(
        rsx! {
            EventFormPage {
                current_user: user_model,
                is_edit: true,
                event_id: Some(event_id),
                event: Some(event),
                calendars: calendars,
                selected_calendar_id: Some(selected_calendar_id),
            }
        }
    )?;
    
    Ok(Html(html))
}

/// Handle edit event form submission
pub async fn update_event_handler(
    State(service): State<CalendarService>,
    Extension(user): Extension<Uuid>,
    Path(event_id): Path<Uuid>,
    Form(form): Form<EventFormInput>,
) -> Result<Response, AppError> {
    let event = service.get_event_by_id(event_id).await?
        .ok_or_else(|| AppError::NotFoundError("Event not found".to_string()))?;
    
    let calendar = service.get_calendar_by_id(event.calendar_id).await?
        .ok_or_else(|| AppError::NotFoundError("Calendar not found".to_string()))?;
    
    // Check ownership
    if calendar.user_id != user {
        return Err(AppError::AuthenticationError("Access denied".to_string()));
    }
    
    // Parse datetime
    let start_time = chrono::NaiveDateTime::parse_from_str(&form.start_time, "%Y-%m-%dT%H:%M")
        .map(|dt| dt.and_utc())
        .map_err(|_| AppError::ValidationError("Invalid start time format".to_string()))?;
    
    let end_time = chrono::NaiveDateTime::parse_from_str(&form.end_time, "%Y-%m-%dT%H:%M")
        .map(|dt| dt.and_utc())
        .map_err(|_| AppError::ValidationError("Invalid end time format".to_string()))?;
    
    let update = UpdateEvent {
        title: Some(form.title),
        description: form.description,
        location: form.location,
        start_time: Some(start_time),
        end_time: Some(end_time),
        is_all_day: Some(form.is_all_day == Some("on".to_string())),
    };
    
    service.update_event(event_id, update).await?;
    
    Ok(Redirect::to(&format!("/web/calendars/{}?message=Event updated&flash_type=success", form.calendar_id)).into_response())
}

/// Handle delete event
pub async fn delete_event_handler(
    State(service): State<CalendarService>,
    Extension(user): Extension<Uuid>,
    Path(event_id): Path<Uuid>,
) -> Result<Response, AppError> {
    let event = service.get_event_by_id(event_id).await?
        .ok_or_else(|| AppError::NotFoundError("Event not found".to_string()))?;
    
    let calendar = service.get_calendar_by_id(event.calendar_id).await?
        .ok_or_else(|| AppError::NotFoundError("Calendar not found".to_string()))?;
    
    // Check ownership
    if calendar.user_id != user {
        return Err(AppError::AuthenticationError("Access denied".to_string()));
    }
    
    let calendar_id = event.calendar_id;
    service.delete_event(event_id).await?;
    
    Ok(Redirect::to(&format!("/web/calendars/{}?message=Event deleted&flash_type=success", calendar_id)).into_response())
}

// ============== Share Handlers ==============

/// Handle create share
pub async fn create_share_handler(
    State(service): State<CalendarService>,
    Extension(user): Extension<Uuid>,
    Path(calendar_id): Path<Uuid>,
    Form(form): Form<ShareFormInput>,
) -> Result<Response, AppError> {
    let calendar = service.get_calendar_by_id(calendar_id).await?
        .ok_or_else(|| AppError::NotFoundError("Calendar not found".to_string()))?;
    
    // Check ownership
    if calendar.user_id != user {
        return Err(AppError::AuthenticationError("Access denied".to_string()));
    }
    
    let new_share = NewShare {
        shared_with_email: form.shared_with_email,
        permission: form.permission,
    };
    
    service.create_share(calendar_id, user, new_share).await?;
    
    Ok(Redirect::to(&format!("/web/calendars/{}?message=Share created&flash_type=success", calendar_id)).into_response())
}

/// Handle delete share
pub async fn delete_share_handler(
    State(service): State<CalendarService>,
    Extension(user): Extension<Uuid>,
    Path(share_id): Path<Uuid>,
) -> Result<Response, AppError> {
    // Get share to find calendar_id for redirect
    let shares = service.get_all_shares().await?;
    let share = shares.iter()
        .find(|s| s.id == share_id)
        .ok_or_else(|| AppError::NotFoundError("Share not found".to_string()))?;
    
    let calendar_id = share.calendar_id;
    
    // Verify ownership of the calendar
    let calendar = service.get_calendar_by_id(calendar_id).await?
        .ok_or_else(|| AppError::NotFoundError("Calendar not found".to_string()))?;
    
    if calendar.user_id != user {
        return Err(AppError::AuthenticationError("Access denied".to_string()));
    }
    
    service.delete_share(share_id).await?;
    
    Ok(Redirect::to(&format!("/web/calendars/{}?message=Share removed&flash_type=success", calendar_id)).into_response())
}
