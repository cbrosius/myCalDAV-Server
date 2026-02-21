use dioxus::prelude::*;
use chrono::{DateTime, Utc};

use crate::models::Event;

#[component]
pub fn EventItem(event: Event) -> Element {
    let start_day = event.start_time.format("%d").to_string();
    let start_month = event.start_time.format("%b").to_string();
    let start_time = event.start_time.format("%H:%M").to_string();
    let end_time = event.end_time.format("%H:%M").to_string();
    
    rsx! {
        div { class: "event-item",
            div { class: "event-date",
                span { class: "event-day", "{start_day}" }
                span { class: "event-month", "{start_month}" }
            }
            div { class: "event-info",
                h4 { "{event.title}" }
                p { class: "event-time", "{start_time} - {end_time}" }
                if let Some(loc) = &event.location {
                    p { class: "event-location", "📍 {loc}" }
                }
            }
            div { class: "event-actions",
                a { href: "/web/events/{event.id}/edit", class: "btn btn-sm btn-outline", "Edit" }
            }
        }
    }
}

#[component]
pub fn EventListItem(event: Event, show_calendar: bool, calendar_name: Option<String>) -> Element {
    let start_date = event.start_time.format("%Y-%m-%d").to_string();
    let start_time = event.start_time.format("%H:%M").to_string();
    let end_time = event.end_time.format("%H:%M").to_string();
    
    rsx! {
        div { class: "event-list-item",
            div { class: "event-info",
                h4 { 
                    a { href: "/web/events/{event.id}", "{event.title}" }
                }
                p { class: "event-time", "{start_date} {start_time} - {end_time}" }
                if show_calendar {
                    if let Some(name) = calendar_name {
                        p { class: "event-calendar", "📅 {name}" }
                    }
                }
                if let Some(loc) = &event.location {
                    p { class: "event-location", "📍 {loc}" }
                }
            }
            div { class: "event-actions",
                a { href: "/web/events/{event.id}/edit", class: "btn btn-sm btn-outline", "Edit" }
                form { action: "/web/events/{event.id}/delete", method: "post", class: "inline-form",
                    button { type: "submit", class: "btn btn-sm btn-danger", "Delete" }
                }
            }
        }
    }
}
