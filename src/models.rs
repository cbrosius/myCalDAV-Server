use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Calendar {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub is_public: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Event {
    pub id: Uuid,
    pub calendar_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub location: Option<String>,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub is_all_day: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Share {
    pub id: Uuid,
    pub calendar_id: Uuid,
    pub user_id: Uuid,
    pub shared_with_user_id: Option<Uuid>,
    pub shared_with_email: Option<String>,
    pub permission_level: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PermissionLevel {
    Read,
    Write,
    Admin,
}

impl PermissionLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            PermissionLevel::Read => "read",
            PermissionLevel::Write => "write",
            PermissionLevel::Admin => "admin",
        }
    }
    
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "read" => Some(PermissionLevel::Read),
            "write" => Some(PermissionLevel::Write),
            "admin" => Some(PermissionLevel::Admin),
            _ => None,
        }
    }
}

// Request/Response DTOs

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewCalendar {
    pub name: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub is_public: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateCalendar {
    pub name: Option<String>,
    pub description: Option<String>,
    pub color: Option<String>,
    pub is_public: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewEvent {
    pub title: String,
    pub description: Option<String>,
    pub location: Option<String>,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub is_all_day: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateEvent {
    pub title: Option<String>,
    pub description: Option<String>,
    pub location: Option<String>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub is_all_day: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewShare {
    pub shared_with_email: String,
    pub permission: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateShare {
    pub permission_level: PermissionLevel,
}

// iCalendar export structures

#[derive(Debug, Clone)]
pub struct ICalendarEvent {
    pub uid: String,
    pub summary: String,
    pub description: Option<String>,
    pub location: Option<String>,
    pub dtstart: DateTime<Utc>,
    pub dtend: DateTime<Utc>,
}

impl ICalendarEvent {
    pub fn to_ical_string(&self) -> String {
        format!(
            "BEGIN:VEVENT\r\n\
             UID:{}\r\n\
             SUMMARY:{}\r\n\
             DESCRIPTION:{}\r\n\
             LOCATION:{}\r\n\
             DTSTART:{}\r\n\
             DTEND:{}\r\n\
             END:VEVENT\r\n",
            self.uid,
            escape_ical_text(&self.summary),
            self.description.as_ref().map(|d| escape_ical_text(d)).unwrap_or_default(),
            self.location.as_ref().map(|l| escape_ical_text(l)).unwrap_or_default(),
            self.dtstart.format("%Y%m%dT%H%M%SZ"),
            self.dtend.format("%Y%m%dT%H%M%SZ")
        )
    }
}

impl From<&Event> for ICalendarEvent {
    fn from(event: &Event) -> Self {
        Self {
            uid: event.id.to_string(),
            summary: event.title.clone(),
            description: event.description.clone(),
            location: event.location.clone(),
            dtstart: event.start_time,
            dtend: event.end_time,
        }
    }
}

fn escape_ical_text(text: &str) -> String {
    text.replace('\\', "\\\\")
        .replace(';', "\\;")
        .replace(',', "\\,")
        .replace('\n', "\\n")
}
