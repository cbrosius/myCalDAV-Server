use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row};
use uuid::Uuid;

// Helper to parse UUID from string
fn parse_uuid(s: &str) -> Result<Uuid, uuid::Error> {
    Uuid::parse_str(s)
}

// Helper to convert UUID to string for database storage
#[allow(dead_code)]
fn uuid_to_string(id: Uuid) -> String {
    id.to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl FromRow<'_, sqlx::sqlite::SqliteRow> for User {
    fn from_row(row: &'_ sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        let id_str: String = row.try_get("id")?;
        let id = parse_uuid(&id_str).map_err(|e| sqlx::Error::ColumnDecode {
            index: "id".to_string(),
            source: Box::new(e),
        })?;
        
        Ok(User {
            id,
            name: row.try_get("name")?,
            email: row.try_get("email")?,
            password_hash: row.try_get("password_hash")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

impl FromRow<'_, sqlx::sqlite::SqliteRow> for Calendar {
    fn from_row(row: &'_ sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        let id_str: String = row.try_get("id")?;
        let id = parse_uuid(&id_str).map_err(|e| sqlx::Error::ColumnDecode {
            index: "id".to_string(),
            source: Box::new(e),
        })?;
        
        let user_id_str: String = row.try_get("user_id")?;
        let user_id = parse_uuid(&user_id_str).map_err(|e| sqlx::Error::ColumnDecode {
            index: "user_id".to_string(),
            source: Box::new(e),
        })?;
        
        Ok(Calendar {
            id,
            user_id,
            name: row.try_get("name")?,
            description: row.try_get("description")?,
            color: row.try_get("color")?,
            is_public: row.try_get::<i32, _>("is_public")? != 0,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

impl FromRow<'_, sqlx::sqlite::SqliteRow> for Event {
    fn from_row(row: &'_ sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        let id_str: String = row.try_get("id")?;
        let id = parse_uuid(&id_str).map_err(|e| sqlx::Error::ColumnDecode {
            index: "id".to_string(),
            source: Box::new(e),
        })?;
        
        let calendar_id_str: String = row.try_get("calendar_id")?;
        let calendar_id = parse_uuid(&calendar_id_str).map_err(|e| sqlx::Error::ColumnDecode {
            index: "calendar_id".to_string(),
            source: Box::new(e),
        })?;
        
        Ok(Event {
            id,
            calendar_id,
            title: row.try_get("title")?,
            description: row.try_get("description")?,
            location: row.try_get("location")?,
            start_time: row.try_get("start_time")?,
            end_time: row.try_get("end_time")?,
            is_all_day: row.try_get::<i32, _>("is_all_day")? != 0,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Share {
    pub id: Uuid,
    pub calendar_id: Uuid,
    pub user_id: Uuid,
    pub shared_with_user_id: Option<Uuid>,
    pub shared_with_email: Option<String>,
    pub permission_level: String,
    pub created_at: DateTime<Utc>,
}

impl FromRow<'_, sqlx::sqlite::SqliteRow> for Share {
    fn from_row(row: &'_ sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        let id_str: String = row.try_get("id")?;
        let id = parse_uuid(&id_str).map_err(|e| sqlx::Error::ColumnDecode {
            index: "id".to_string(),
            source: Box::new(e),
        })?;
        
        let calendar_id_str: String = row.try_get("calendar_id")?;
        let calendar_id = parse_uuid(&calendar_id_str).map_err(|e| sqlx::Error::ColumnDecode {
            index: "calendar_id".to_string(),
            source: Box::new(e),
        })?;
        
        let user_id_str: String = row.try_get("user_id")?;
        let user_id = parse_uuid(&user_id_str).map_err(|e| sqlx::Error::ColumnDecode {
            index: "user_id".to_string(),
            source: Box::new(e),
        })?;
        
        let shared_with_user_id: Option<String> = row.try_get("shared_with_user_id")?;
        let shared_with_user_id = shared_with_user_id
            .as_ref()
            .map(|s| parse_uuid(s))
            .transpose()
            .map_err(|e| sqlx::Error::ColumnDecode {
                index: "shared_with_user_id".to_string(),
                source: Box::new(e),
            })?;
        
        Ok(Share {
            id,
            calendar_id,
            user_id,
            shared_with_user_id,
            shared_with_email: row.try_get("shared_with_email")?,
            permission_level: row.try_get("permission_level")?,
            created_at: row.try_get("created_at")?,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PermissionLevel {
    Read,
    Write,
    Admin,
}

#[allow(dead_code)]
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
