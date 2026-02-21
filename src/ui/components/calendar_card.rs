use dioxus::prelude::*;
use uuid::Uuid;

use crate::models::Calendar;

#[component]
pub fn CalendarCard(calendar: Calendar) -> Element {
    rsx! {
        div { class: "calendar-card",
            h3 { "{calendar.name}" }
            if let Some(desc) = &calendar.description {
                p { class: "calendar-description", "{desc}" }
            }
            div { class: "calendar-actions",
                a { href: "/web/calendars/{calendar.id}", class: "btn btn-sm btn-secondary", "View" }
                a { href: "/web/calendars/{calendar.id}/edit", class: "btn btn-sm btn-outline", "Edit" }
            }
            if calendar.is_public {
                span { class: "badge badge-public", "Public" }
            }
        }
    }
}

#[component]
pub fn CalendarListItem(calendar: Calendar, event_count: usize) -> Element {
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
            }
            div { class: "calendar-actions",
                a { href: "/web/calendars/{calendar.id}", class: "btn btn-sm btn-secondary", "View" }
                a { href: "/web/calendars/{calendar.id}/edit", class: "btn btn-sm btn-outline", "Edit" }
            }
        }
    }
}
