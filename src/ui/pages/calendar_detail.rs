use dioxus::prelude::*;

use crate::models::{User, Calendar, Event, Share};
use crate::ui::layouts::BaseLayout;
use crate::ui::components::{EventListItem, ShareItem};

#[component]
pub fn CalendarDetailPage(
    current_user: User,
    calendar: Calendar,
    events: Vec<Event>,
    shares: Vec<Share>,
    caldav_url: String,
    flash_message: Option<String>,
    flash_type: Option<String>,
) -> Element {
    let calendar_id = calendar.id;
    let is_public = calendar.is_public;
    let events_count = events.len();
    let shares_count = shares.len();
    let calendar_name = calendar.name.clone();
    let calendar_description = calendar.description.clone();
    
    rsx! {
        BaseLayout {
            current_user: Some(current_user),
            title: Some(format!("{} - My CalDAV Server", calendar_name)),
            flash_message: flash_message,
            flash_type: flash_type,
            
            div { class: "page-header",
                div { class: "calendar-header",
                    div { class: "calendar-title",
                        h1 { "{calendar_name}" }
                    }
                    div { class: "page-actions",
                        a { href: "/web/calendars/{calendar_id}/edit", class: "btn btn-outline", "Edit Calendar" }
                        a { href: "/web/events/new?calendar={calendar_id}", class: "btn btn-primary", "+ New Event" }
                    }
                }
                if let Some(desc) = calendar_description {
                    p { class: "calendar-description", "{desc}" }
                }
            }

            div { class: "calendar-info-bar",
                div { class: "info-item",
                    span { class: "info-label", "Status:" }
                    if is_public {
                        span { class: "badge badge-public", "Public" }
                    } else {
                        span { class: "badge badge-private", "Private" }
                    }
                }
                div { class: "info-item",
                    span { class: "info-label", "Events:" }
                    span { class: "info-value", "{events_count}" }
                }
                div { class: "info-item",
                    span { class: "info-label", "Shares:" }
                    span { class: "info-value", "{shares_count}" }
                }
            }

            div { class: "tabs",
                button { class: "tab-btn active", "data-tab": "events", "Events" }
                button { class: "tab-btn", "data-tab": "shares", "Shares" }
                button { class: "tab-btn", "data-tab": "settings", "Settings" }
            }

            div { class: "tab-content active", id: "events-tab",
                if events.is_empty() {
                    div { class: "empty-state",
                        div { class: "empty-icon", "📌" }
                        h2 { "No events yet" }
                        p { "This calendar doesn't have any events." }
                        a { href: "/web/events/new?calendar={calendar_id}", class: "btn btn-primary", "Create Event" }
                    }
                } else {
                    div { class: "event-list",
                        for event in events {
                            EventListItem { 
                                event: event, 
                                show_calendar: false, 
                                calendar_name: None 
                            }
                        }
                    }
                }
            }

            div { class: "tab-content", id: "shares-tab",
                div { class: "section-header",
                    h3 { "Calendar Shares" }
                    button { class: "btn btn-primary", "+ Add Share" }
                }
                
                if shares.is_empty() {
                    div { class: "empty-state",
                        p { "This calendar is not shared with anyone." }
                    }
                } else {
                    div { class: "share-list",
                        for share in shares {
                            ShareItem { share: share }
                        }
                    }
                }
                
                // Share Modal placeholder
                div { id: "share-modal", class: "modal",
                    div { class: "modal-content",
                        div { class: "modal-header",
                            h3 { "Share Calendar" }
                            button { class: "modal-close", "×" }
                        }
                        form { action: "/web/calendars/{calendar_id}/shares", method: "post",
                            div { class: "form-group",
                                label { r#for: "shared_with_email", "Email Address" }
                                input {
                                    r#type: "email",
                                    id: "shared_with_email",
                                    name: "shared_with_email",
                                    required: true,
                                    placeholder: "Enter email address"
                                }
                            }
                            div { class: "form-group",
                                label { r#for: "permission", "Permission" }
                                select { id: "permission", name: "permission",
                                    option { value: "read", "Read Only" }
                                    option { value: "write", "Read & Write" }
                                    option { value: "admin", "Full Access" }
                                }
                            }
                            div { class: "form-actions",
                                button { r#type: "button", class: "btn btn-secondary", "Cancel" }
                                button { r#type: "submit", class: "btn btn-primary", "Share" }
                            }
                        }
                    }
                }
            }

            div { class: "tab-content", id: "settings-tab",
                div { class: "settings-section",
                    h3 { "CalDAV Access" }
                    p { "Use these settings to access this calendar from your CalDAV client:" }
                    div { class: "config-item",
                        label { "Calendar URL:" }
                        code { "{caldav_url}/calendars/{calendar_id}/" }
                    }
                }
            }
        }
    }
}
