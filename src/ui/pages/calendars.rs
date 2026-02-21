use dioxus::prelude::*;
use std::collections::HashMap;
use uuid::Uuid;

use crate::models::{User, Calendar};
use crate::ui::layouts::BaseLayout;

#[component]
pub fn CalendarsPage(
    current_user: User,
    calendars: Vec<Calendar>,
    event_counts: HashMap<Uuid, usize>,
) -> Element {
    rsx! {
        BaseLayout {
            current_user: Some(current_user),
            title: Some("Calendars - My CalDAV Server".to_string()),
            
            div { class: "page-header",
                h1 { "My Calendars" }
                a { href: "/web/calendars/new", class: "btn btn-primary", "+ New Calendar" }
            }
            
            if calendars.is_empty() {
                div { class: "empty-state",
                    div { class: "empty-icon", "📅" }
                    h2 { "No calendars yet" }
                    p { "Create your first calendar to get started." }
                    a { href: "/web/calendars/new", class: "btn btn-primary", "Create Calendar" }
                }
            } else {
                div { class: "calendar-list",
                    for calendar in calendars {
                        CalendarListItem { 
                            calendar: calendar.clone(), 
                            event_count: *event_counts.get(&calendar.id).unwrap_or(&0)
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn CalendarListItem(calendar: Calendar, event_count: usize) -> Element {
    rsx! {
        div { class: "calendar-list-item",
            div { class: "calendar-info",
                h3 { "{calendar.name}" }
                if let Some(desc) = &calendar.description {
                    p { class: "calendar-description", "{desc}" }
                }
            }
            div { class: "calendar-stats",
                span { "{event_count} events" }
                if calendar.is_public {
                    span { class: "badge badge-public", "Public" }
                }
            }
            div { class: "calendar-actions",
                a { href: "/web/calendars/{calendar.id}", class: "btn btn-sm btn-secondary", "View" }
                a { href: "/web/calendars/{calendar.id}/edit", class: "btn btn-sm btn-outline", "Edit" }
            }
        }
    }
}
