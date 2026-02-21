use dioxus::prelude::*;
use std::collections::HashMap;

use crate::models::{User, Calendar, Event};
use crate::ui::layouts::BaseLayout;
use crate::ui::components::{StatCard, CalendarCard, EventItem};

#[component]
pub fn DashboardPage(
    current_user: User,
    calendar_count: usize,
    event_count: usize,
    share_count: usize,
    calendars: Vec<Calendar>,
    upcoming_events: Vec<Event>,
    caldav_url: String,
) -> Element {
    let user_name = current_user.name.clone();
    let user_email = current_user.email.clone();
    
    rsx! {
        BaseLayout {
            current_user: Some(current_user),
            title: Some("Dashboard - My CalDAV Server".to_string()),
            
            div { class: "dashboard",
                div { class: "dashboard-header",
                    h1 { "Welcome, {user_name}!" }
                    p { class: "subtitle", "Manage your calendars and events" }
                }
                
                div { class: "dashboard-stats",
                    StatCard { icon: "📅".to_string(), number: calendar_count, label: "Calendars".to_string() }
                    StatCard { icon: "📌".to_string(), number: event_count, label: "Events".to_string() }
                    StatCard { icon: "🔗".to_string(), number: share_count, label: "Shares".to_string() }
                }
                
                div { class: "dashboard-section",
                    div { class: "section-header",
                        h2 { "Your Calendars" }
                        a { href: "/web/calendars/new", class: "btn btn-primary", "+ New Calendar" }
                    }
                    
                    if calendars.is_empty() {
                        div { class: "empty-state",
                            p { "You don't have any calendars yet." }
                            a { href: "/web/calendars/new", class: "btn btn-secondary", "Create your first calendar" }
                        }
                    } else {
                        div { class: "calendar-grid",
                            for calendar in calendars {
                                CalendarCard { calendar: calendar }
                            }
                        }
                    }
                }
                
                div { class: "dashboard-section",
                    div { class: "section-header",
                        h2 { "Upcoming Events" }
                        a { href: "/web/events/new", class: "btn btn-primary", "+ New Event" }
                    }
                    
                    if upcoming_events.is_empty() {
                        div { class: "empty-state",
                            p { "No upcoming events." }
                        }
                    } else {
                        div { class: "event-list",
                            for event in upcoming_events {
                                EventItem { event: event }
                            }
                        }
                    }
                }
                
                div { class: "dashboard-section",
                    h2 { "CalDAV Configuration" }
                    div { class: "config-info",
                        p { "Use the following settings to connect your CalDAV client:" }
                        div { class: "config-item",
                            label { "Server URL:" }
                            code { "{caldav_url}" }
                        }
                        div { class: "config-item",
                            label { "Username:" }
                            code { "{user_email}" }
                        }
                        div { class: "config-item",
                            label { "Password:" }
                            code { "Your account password" }
                        }
                    }
                }
            }
        }
    }
}
