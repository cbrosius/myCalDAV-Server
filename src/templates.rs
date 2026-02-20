use askama::Template;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use uuid::Uuid;

use crate::models::{Calendar, Event, Share, User};

/// User info for templates
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct UserInfo {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

impl From<&User> for UserInfo {
    fn from(user: &User) -> Self {
        UserInfo {
            id: user.id,
            name: user.name.clone(),
            email: user.email.clone(),
        }
    }
}

/// Calendar info for templates
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CalendarInfo {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub is_public: bool,
}

impl From<&Calendar> for CalendarInfo {
    fn from(cal: &Calendar) -> Self {
        CalendarInfo {
            id: cal.id,
            name: cal.name.clone(),
            description: cal.description.clone(),
            color: cal.color.clone(),
            is_public: cal.is_public,
        }
    }
}

/// Event info for templates
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct EventInfo {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub location: Option<String>,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub is_all_day: bool,
    pub calendar_id: Uuid,
}

impl From<&Event> for EventInfo {
    fn from(event: &Event) -> Self {
        EventInfo {
            id: event.id,
            title: event.title.clone(),
            description: event.description.clone(),
            location: event.location.clone(),
            start_time: event.start_time,
            end_time: event.end_time,
            is_all_day: event.is_all_day,
            calendar_id: event.calendar_id,
        }
    }
}

/// Share info for templates
#[derive(Debug, Clone)]
pub struct ShareInfo {
    pub id: Uuid,
    pub shared_with_email: Option<String>,
    pub permission: String,
}

impl From<&Share> for ShareInfo {
    fn from(share: &Share) -> Self {
        ShareInfo {
            id: share.id,
            shared_with_email: share.shared_with_email.clone(),
            permission: share.permission_level.clone(),
        }
    }
}

/// Calendar form data
#[derive(Debug, Clone, Default)]
pub struct CalendarFormData {
    pub name: Option<String>,
    pub description: Option<String>,
    pub color: Option<String>,
    pub is_public: bool,
}

impl From<&Calendar> for CalendarFormData {
    fn from(cal: &Calendar) -> Self {
        CalendarFormData {
            name: Some(cal.name.clone()),
            description: cal.description.clone(),
            color: cal.color.clone(),
            is_public: cal.is_public,
        }
    }
}

/// Event form data
#[derive(Debug, Clone, Default)]
pub struct EventFormData {
    pub title: Option<String>,
    pub description: Option<String>,
    pub location: Option<String>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub is_all_day: Option<bool>,
}

impl From<&Event> for EventFormData {
    fn from(event: &Event) -> Self {
        EventFormData {
            title: Some(event.title.clone()),
            description: event.description.clone(),
            location: event.location.clone(),
            start_time: Some(event.start_time),
            end_time: Some(event.end_time),
            is_all_day: Some(event.is_all_day),
        }
    }
}

/// Login page template
#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate {
    pub current_user: Option<UserInfo>,
    pub flash_message: Option<String>,
    pub flash_type: Option<String>,
}

/// Register page template
#[derive(Template)]
#[template(path = "register.html")]
pub struct RegisterTemplate {
    pub current_user: Option<UserInfo>,
    pub flash_message: Option<String>,
    pub flash_type: Option<String>,
}

/// Dashboard template
#[derive(Template)]
#[template(path = "dashboard.html")]
pub struct DashboardTemplate {
    pub current_user: Option<UserInfo>,
    pub flash_message: Option<String>,
    pub flash_type: Option<String>,
    pub user_name: String,
    pub user_email: String,
    pub calendar_count: usize,
    pub event_count: usize,
    pub share_count: usize,
    pub calendars: Vec<CalendarInfo>,
    pub upcoming_events: Vec<EventInfo>,
    pub caldav_url: String,
}

/// Calendar list template
#[derive(Template)]
#[template(path = "calendars.html")]
pub struct CalendarsTemplate {
    pub current_user: Option<UserInfo>,
    pub flash_message: Option<String>,
    pub flash_type: Option<String>,
    pub calendars: Vec<CalendarInfo>,
    pub event_counts: HashMap<Uuid, usize>,
}

/// Calendar form template (for create/edit)
#[derive(Template)]
#[template(path = "calendar_form.html")]
pub struct CalendarFormTemplate {
    pub current_user: Option<UserInfo>,
    pub flash_message: Option<String>,
    pub flash_type: Option<String>,
    pub is_edit: bool,
    pub calendar_id: Option<Uuid>,
    pub calendar: CalendarFormData,
}

/// Calendar detail template
#[derive(Template)]
#[template(path = "calendar_detail.html")]
pub struct CalendarDetailTemplate {
    pub current_user: Option<UserInfo>,
    pub flash_message: Option<String>,
    pub flash_type: Option<String>,
    pub calendar: CalendarInfo,
    pub events: Vec<EventInfo>,
    pub shares: Vec<ShareInfo>,
    pub caldav_url: String,
}

/// Events list template
#[derive(Template)]
#[template(path = "events.html")]
pub struct EventsTemplate {
    pub current_user: Option<UserInfo>,
    pub flash_message: Option<String>,
    pub flash_type: Option<String>,
    pub events: Vec<EventInfo>,
    pub calendars: Vec<CalendarInfo>,
    pub calendar_names: HashMap<Uuid, String>,
    pub selected_calendar: Option<Uuid>,
}

impl EventsTemplate {
    pub fn is_calendar_selected(&self, calendar_id: &Uuid) -> bool {
        self.selected_calendar.map_or(false, |id| &id == calendar_id)
    }
}

/// Event form template (for create/edit)
#[derive(Template)]
#[template(path = "event_form.html")]
pub struct EventFormTemplate {
    pub current_user: Option<UserInfo>,
    pub flash_message: Option<String>,
    pub flash_type: Option<String>,
    pub is_edit: bool,
    pub event_id: Option<Uuid>,
    pub event: EventFormData,
    pub calendars: Vec<CalendarInfo>,
    pub selected_calendar_id: Option<Uuid>,
}

impl EventFormTemplate {
    pub fn is_calendar_selected(&self, calendar_id: &Uuid) -> bool {
        self.selected_calendar_id.map_or(false, |id| &id == calendar_id)
    }
}
