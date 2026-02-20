use axum::{
    extract::{Path, State, Extension},
    http::{header, StatusCode, Uri},
    response::{Html, IntoResponse, Response},
    body::Body,
    Json,
};
use uuid::Uuid;
use crate::models::*;
use crate::services::CalendarService;
use crate::error::AppError;

pub mod auth;

// Root endpoint
pub async fn root() -> impl IntoResponse {
    let body = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>My CalDAV Server</title>
</head>
<body>
    <h1>My CalDAV Server</h1>
    <p>Welcome to your CalDAV server!</p>
    <p>API is running and ready to accept requests.</p>
    <h2>Available Endpoints:</h2>
    <ul>
        <li><code>GET /health</code> - Health check</li>
        <li><code>POST /api/auth/register</code> - Register new user</li>
        <li><code>POST /api/auth/login</code> - Login</li>
        <li><code>GET /api/auth/calendars</code> - Get user calendars</li>
        <li><code>POST /api/auth/calendars</code> - Create calendar</li>
        <li><code>GET /api/auth/events/:id</code> - Get event by ID</li>
        <li><code>POST /api/auth/events</code> - Create event</li>
        <li><code>GET /.well-known/caldav</code> - CalDAV discovery</li>
    </ul>
</body>
</html>"#;
    
    Html(body)
}

// Health check endpoint
pub async fn health() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

// User endpoints
pub async fn get_user_by_id(
    State(service): State<CalendarService>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<User>, AppError> {
    let user = service.get_user_by_id(user_id).await?
        .ok_or(AppError::NotFoundError("User not found".to_string()))?;
    Ok(Json(user))
}

// Calendar endpoints
pub async fn get_calendar_by_id(
    State(service): State<CalendarService>,
    Path(calendar_id): Path<Uuid>,
) -> Result<Json<Calendar>, AppError> {
    let calendar = service.get_calendar_by_id(calendar_id).await?
        .ok_or(AppError::NotFoundError("Calendar not found".to_string()))?;
    Ok(Json(calendar))
}

pub async fn update_calendar(
    State(service): State<CalendarService>,
    Extension(user_id): Extension<Uuid>,
    Path(calendar_id): Path<Uuid>,
    Json(updates): Json<UpdateCalendar>,
) -> Result<Json<Calendar>, AppError> {
    // Check ownership
    let calendar = service.get_calendar_by_id(calendar_id).await?
        .ok_or(AppError::NotFoundError("Calendar not found".to_string()))?;
    
    if calendar.user_id != user_id {
        return Err(AppError::AuthenticationError("You don't own this calendar".to_string()));
    }
    
    let updated = service.update_calendar(calendar_id, updates).await?;
    Ok(Json(updated))
}

pub async fn delete_calendar(
    State(service): State<CalendarService>,
    Extension(user_id): Extension<Uuid>,
    Path(calendar_id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    // Check ownership
    let calendar = service.get_calendar_by_id(calendar_id).await?
        .ok_or(AppError::NotFoundError("Calendar not found".to_string()))?;
    
    if calendar.user_id != user_id {
        return Err(AppError::AuthenticationError("You don't own this calendar".to_string()));
    }
    
    service.delete_calendar(calendar_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

// Event endpoints
pub async fn get_event_by_id(
    State(service): State<CalendarService>,
    Path(event_id): Path<Uuid>,
) -> Result<Json<Event>, AppError> {
    let event = service.get_event_by_id(event_id).await?
        .ok_or(AppError::NotFoundError("Event not found".to_string()))?;
    Ok(Json(event))
}

pub async fn update_event(
    State(service): State<CalendarService>,
    Extension(user_id): Extension<Uuid>,
    Path(event_id): Path<Uuid>,
    Json(updates): Json<UpdateEvent>,
) -> Result<Json<Event>, AppError> {
    // Check ownership
    let event = service.get_event_by_id(event_id).await?
        .ok_or(AppError::NotFoundError("Event not found".to_string()))?;
    
    let calendar = service.get_calendar_by_id(event.calendar_id).await?
        .ok_or(AppError::NotFoundError("Calendar not found".to_string()))?;
    
    if calendar.user_id != user_id {
        return Err(AppError::AuthenticationError("You don't have access to this event".to_string()));
    }
    
    let updated = service.update_event(event_id, updates).await?;
    Ok(Json(updated))
}

pub async fn delete_event(
    State(service): State<CalendarService>,
    Extension(user_id): Extension<Uuid>,
    Path(event_id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    // Check ownership
    let event = service.get_event_by_id(event_id).await?
        .ok_or(AppError::NotFoundError("Event not found".to_string()))?;
    
    let calendar = service.get_calendar_by_id(event.calendar_id).await?
        .ok_or(AppError::NotFoundError("Calendar not found".to_string()))?;
    
    if calendar.user_id != user_id {
        return Err(AppError::AuthenticationError("You don't have access to this event".to_string()));
    }
    
    service.delete_event(event_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

// Share endpoints
pub async fn get_calendar_shares(
    State(service): State<CalendarService>,
    Extension(user_id): Extension<Uuid>,
    Path(calendar_id): Path<Uuid>,
) -> Result<Json<Vec<Share>>, AppError> {
    // Check ownership
    let calendar = service.get_calendar_by_id(calendar_id).await?
        .ok_or(AppError::NotFoundError("Calendar not found".to_string()))?;
    
    if calendar.user_id != user_id {
        return Err(AppError::AuthenticationError("You don't own this calendar".to_string()));
    }
    
    let shares = service.get_shares_by_calendar_id(calendar_id).await?;
    Ok(Json(shares))
}

pub async fn create_share(
    State(service): State<CalendarService>,
    Extension(user_id): Extension<Uuid>,
    Path(calendar_id): Path<Uuid>,
    Json(new_share): Json<NewShare>,
) -> Result<Json<Share>, AppError> {
    // Check ownership
    let calendar = service.get_calendar_by_id(calendar_id).await?
        .ok_or(AppError::NotFoundError("Calendar not found".to_string()))?;
    
    if calendar.user_id != user_id {
        return Err(AppError::AuthenticationError("You don't own this calendar".to_string()));
    }
    
    let share = service.create_share(calendar_id, user_id, new_share).await?;
    Ok(Json(share))
}

pub async fn delete_share(
    State(service): State<CalendarService>,
    Extension(_user_id): Extension<Uuid>,
    Path(share_id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    service.delete_share(share_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

// CalDAV Protocol Handlers

/// CalDAV well-known discovery endpoint
pub async fn caldav_discovery() -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/plain")
        .body(Body::from("/calendars/"))
        .unwrap()
}

/// Handle CalDAV PROPFIND requests
pub async fn caldav_propfind(
    State(service): State<CalendarService>,
    Extension(user_id): Extension<Uuid>,
    _uri: Uri,
) -> Result<Response, AppError> {
    let calendars = service.get_calendars_by_user_id(user_id).await?;
    
    let mut responses = String::new();
    
    for calendar in calendars {
        let calendar_url = format!("/calendars/{}/", calendar.id);
        responses.push_str(&format!(
            r#"<d:response>
                <d:href>{}</d:href>
                <d:propstat>
                    <d:prop>
                        <d:resourcetype>
                            <d:collection/>
                            <cal:calendar/>
                        </d:resourcetype>
                        <d:displayname>{}</d:displayname>
                        <cal:supported-calendar-component-set>
                            <cal:comp name="VEVENT"/>
                            <cal:comp name="VTODO"/>
                        </cal:supported-calendar-component-set>
                    </d:prop>
                    <d:status>HTTP/1.1 200 OK</d:status>
                </d:propstat>
            </d:response>"#,
            calendar_url,
            calendar.name
        ));
    }
    
    let body = format!(
        r#"<?xml version="1.0" encoding="utf-8"?>
<d:multistatus xmlns:d="DAV:" xmlns:cal="urn:ietf:params:xml:ns:caldav">
    {}
</d:multistatus>"#,
        responses
    );
    
    Ok(Response::builder()
        .status(StatusCode::MULTI_STATUS)
        .header(header::CONTENT_TYPE, "application/xml; charset=utf-8")
        .body(Body::from(body))
        .unwrap())
}

/// Handle CalDAV REPORT requests for calendar queries
pub async fn caldav_report(
    State(service): State<CalendarService>,
    Extension(user_id): Extension<Uuid>,
    _body: String,
) -> Result<Response, AppError> {
    let calendars = service.get_calendars_by_user_id(user_id).await?;
    
    let mut responses = String::new();
    
    for calendar in calendars {
        let events = service.get_events_by_calendar_id(calendar.id).await?;
        
        for event in events {
            let event_url = format!("/calendars/{}/{}.ics", calendar.id, event.id);
            let ical_event = ICalendarEvent::from(&event);
            
            responses.push_str(&format!(
                r#"<d:response>
                    <d:href>{}</d:href>
                    <d:propstat>
                        <d:prop>
                            <d:getetag>"{}"</d:getetag>
                            <cal:calendar-data>{}</cal:calendar-data>
                        </d:prop>
                        <d:status>HTTP/1.1 200 OK</d:status>
                    </d:propstat>
                </d:response>"#,
                event_url,
                event.id,
                escape_xml(&ical_event.to_ical_string())
            ));
        }
    }
    
    let response_body = format!(
        r#"<?xml version="1.0" encoding="utf-8"?>
<d:multistatus xmlns:d="DAV:" xmlns:cal="urn:ietf:params:xml:ns:caldav">
    {}
</d:multistatus>"#,
        responses
    );
    
    Ok(Response::builder()
        .status(StatusCode::MULTI_STATUS)
        .header(header::CONTENT_TYPE, "application/xml; charset=utf-8")
        .body(Body::from(response_body))
        .unwrap())
}

/// Handle CalDAV GET requests for calendar data
pub async fn caldav_get(
    State(service): State<CalendarService>,
    user_id: Uuid,
    path: &str,
) -> Result<Response, AppError> {
    // Parse path like /calendars/{calendar_id}/ or /calendars/{calendar_id}/{event_id}.ics
    let parts: Vec<&str> = path.trim_start_matches('/').split('/').collect();
    
    if parts.len() < 2 {
        return Err(AppError::ValidationError("Invalid calendar path".to_string()));
    }
    
    let calendar_id = Uuid::parse_str(parts[1])?;
    let calendar = service.get_calendar_by_id(calendar_id).await?
        .ok_or(AppError::NotFoundError("Calendar not found".to_string()))?;
    
    // Check access
    if calendar.user_id != user_id && !calendar.is_public {
        return Err(AppError::AuthenticationError("Access denied".to_string()));
    }
    
    if parts.len() == 2 || parts[2].is_empty() {
        // Return entire calendar
        let events = service.get_events_by_calendar_id(calendar_id).await?;
        let mut ical_content = format!(
            "BEGIN:VCALENDAR\r\n\
             VERSION:2.0\r\n\
             PRODID:-//My CalDAV Server//EN\r\n\
             CALSCALE:GREGORIAN\r\n\
             X-WR-CALNAME:{}\r\n",
            calendar.name
        );
        
        for event in events {
            let ical_event = ICalendarEvent::from(&event);
            ical_content.push_str(&ical_event.to_ical_string());
        }
        
        ical_content.push_str("END:VCALENDAR\r\n");
        
        return Ok(Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "text/calendar; charset=utf-8")
            .body(Body::from(ical_content))
            .unwrap());
    }
    
    // Return specific event
    let event_filename = parts[2];
    let event_id_str = event_filename.trim_end_matches(".ics");
    let event_id = Uuid::parse_str(event_id_str)?;
    
    let event = service.get_event_by_id(event_id).await?
        .ok_or(AppError::NotFoundError("Event not found".to_string()))?;
    
    let ical_event = ICalendarEvent::from(&event);
    let ical_content = format!(
        "BEGIN:VCALENDAR\r\n\
         VERSION:2.0\r\n\
         PRODID:-//My CalDAV Server//EN\r\n\
         {}\
         END:VCALENDAR\r\n",
        ical_event.to_ical_string()
    );
    
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/calendar; charset=utf-8")
        .header("ETag", format!("\"{}\"", event.id))
        .body(Body::from(ical_content))
        .unwrap())
}

/// Handle CalDAV PUT requests to create/update events
pub async fn caldav_put(
    State(service): State<CalendarService>,
    user_id: Uuid,
    path: &str,
    body: String,
) -> Result<Response, AppError> {
    // Parse path like /calendars/{calendar_id}/{event_id}.ics
    let parts: Vec<&str> = path.trim_start_matches('/').split('/').collect();
    
    if parts.len() < 3 {
        return Err(AppError::ValidationError("Invalid event path".to_string()));
    }
    
    let calendar_id = Uuid::parse_str(parts[1])?;
    let calendar = service.get_calendar_by_id(calendar_id).await?
        .ok_or(AppError::NotFoundError("Calendar not found".to_string()))?;
    
    // Check ownership
    if calendar.user_id != user_id {
        return Err(AppError::AuthenticationError("You don't own this calendar".to_string()));
    }
    
    // Parse iCalendar data
    let new_event = parse_icalendar(&body)?;
    let event = service.create_event(calendar_id, new_event).await?;
    
    Ok(Response::builder()
        .status(StatusCode::CREATED)
        .header(header::LOCATION, format!("/calendars/{}/{}.ics", calendar_id, event.id))
        .header("ETag", format!("\"{}\"", event.id))
        .body(Body::from(""))
        .unwrap())
}

/// Handle CalDAV DELETE requests
pub async fn caldav_delete(
    State(service): State<CalendarService>,
    user_id: Uuid,
    path: &str,
) -> Result<Response, AppError> {
    // Parse path like /calendars/{calendar_id}/{event_id}.ics
    let parts: Vec<&str> = path.trim_start_matches('/').split('/').collect();
    
    if parts.len() < 3 {
        return Err(AppError::ValidationError("Invalid event path".to_string()));
    }
    
    let event_filename = parts[2];
    let event_id_str = event_filename.trim_end_matches(".ics");
    let event_id = Uuid::parse_str(event_id_str)?;
    
    // Check ownership
    let event = service.get_event_by_id(event_id).await?
        .ok_or(AppError::NotFoundError("Event not found".to_string()))?;
    
    let calendar = service.get_calendar_by_id(event.calendar_id).await?
        .ok_or(AppError::NotFoundError("Calendar not found".to_string()))?;
    
    if calendar.user_id != user_id {
        return Err(AppError::AuthenticationError("You don't have access to this event".to_string()));
    }
    
    service.delete_event(event_id).await?;
    
    Ok(Response::builder()
        .status(StatusCode::NO_CONTENT)
        .body(Body::from(""))
        .unwrap())
}

/// Parse iCalendar VEVENT data into NewEvent
fn parse_icalendar(data: &str) -> Result<NewEvent, AppError> {
    let mut title = None;
    let mut description = None;
    let mut location = None;
    let mut start_time = None;
    let mut end_time = None;
    let mut is_all_day = false;
    
    for line in data.lines() {
        let line = line.trim();
        
        if line.starts_with("SUMMARY:") {
            title = Some(line[8..].to_string());
        } else if line.starts_with("DESCRIPTION:") {
            description = Some(line[12..].to_string());
        } else if line.starts_with("LOCATION:") {
            location = Some(line[9..].to_string());
        } else if line.starts_with("DTSTART") {
            start_time = Some(parse_ical_datetime(&line.split(':').last().unwrap_or(""))?);
        } else if line.starts_with("DTEND") {
            end_time = Some(parse_ical_datetime(&line.split(':').last().unwrap_or(""))?);
        } else if line.contains("VALUE=DATE") {
            is_all_day = true;
        }
    }
    
    let title = title.ok_or(AppError::ValidationError("Missing SUMMARY".to_string()))?;
    let start_time = start_time.ok_or(AppError::ValidationError("Missing DTSTART".to_string()))?;
    let end_time = end_time.ok_or(AppError::ValidationError("Missing DTEND".to_string()))?;
    
    Ok(NewEvent {
        title,
        description,
        location,
        start_time,
        end_time,
        is_all_day,
    })
}

/// Parse iCalendar datetime format
fn parse_ical_datetime(date_str: &str) -> Result<chrono::DateTime<chrono::Utc>, AppError> {
    // Handle both DATE and DATE-TIME formats
    let date_str = date_str.trim();
    
    if date_str.len() == 8 {
        // DATE format (YYYYMMDD)
        chrono::NaiveDate::parse_from_str(date_str, "%Y%m%d")
            .map(|d| d.and_hms_opt(0, 0, 0).unwrap().and_utc())
            .map_err(|_| AppError::ValidationError("Invalid date format".to_string()))
    } else if date_str.ends_with('Z') {
        // UTC DATE-TIME format (YYYYMMDDTHHMMSSZ)
        chrono::NaiveDateTime::parse_from_str(&date_str[..15], "%Y%m%dT%H%M%S")
            .map(|dt| dt.and_utc())
            .map_err(|_| AppError::ValidationError("Invalid datetime format".to_string()))
    } else {
        // Local DATE-TIME format (YYYYMMDDTHHMMSS)
        chrono::NaiveDateTime::parse_from_str(date_str, "%Y%m%dT%H%M%S")
            .map(|dt| dt.and_utc())
            .map_err(|_| AppError::ValidationError("Invalid datetime format".to_string()))
    }
}

/// Escape XML special characters
fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
     .replace('<', "&lt;")
     .replace('>', "&gt;")
     .replace('"', "&quot;")
     .replace('\'', "&apos;")
}
