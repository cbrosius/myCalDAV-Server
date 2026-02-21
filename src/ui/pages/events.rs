use dioxus::prelude::*;
use std::collections::HashMap;
use uuid::Uuid;

use crate::models::{User, Calendar, Event};
use crate::ui::layouts::BaseLayout;
use crate::ui::components::EventListItem;

#[component]
pub fn EventsPage(
    current_user: User,
    events: Vec<Event>,
    calendars: Vec<Calendar>,
    calendar_names: HashMap<Uuid, String>,
    selected_calendar: Option<Uuid>,
) -> Element {
    rsx! {
        BaseLayout {
            current_user: Some(current_user),
            title: Some("Events - My CalDAV Server".to_string()),
            
            div { class: "page-header",
                h1 { "All Events" }
                a { href: "/web/events/new", class: "btn btn-primary", "+ New Event" }
            }
            
            div { class: "filter-bar",
                form { method: "get", action: "/web/events",
                    label { r#for: "calendar", "Filter by Calendar:" }
                    select { 
                        id: "calendar", 
                        name: "calendar",
                        option { value: "", "All Calendars" }
                        for cal in calendars.clone() {
                            option { 
                                value: "{cal.id}",
                                selected: selected_calendar.map_or(false, |id| id == cal.id),
                                "{cal.name}"
                            }
                        }
                    }
                    button { r#type: "submit", class: "btn btn-sm btn-secondary", "Filter" }
                }
            }
            
            if events.is_empty() {
                div { class: "empty-state",
                    div { class: "empty-icon", "📌" }
                    h2 { "No events yet" }
                    p { "Create your first event to get started." }
                    a { href: "/web/events/new", class: "btn btn-primary", "Create Event" }
                }
            } else {
                div { class: "event-list",
                    for event in events {
                        EventListItem { 
                            event: event.clone(), 
                            show_calendar: selected_calendar.is_none(),
                            calendar_name: calendar_names.get(&event.calendar_id).cloned()
                        }
                    }
                }
            }
        }
    }
}
