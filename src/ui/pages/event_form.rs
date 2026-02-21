use dioxus::prelude::*;
use uuid::Uuid;

use crate::models::{User, Calendar, Event};
use crate::ui::layouts::BaseLayout;

#[component]
pub fn EventFormPage(
    current_user: User,
    is_edit: bool,
    event_id: Option<Uuid>,
    event: Option<Event>,
    calendars: Vec<Calendar>,
    selected_calendar_id: Option<Uuid>,
) -> Element {
    let title = if is_edit { "Edit Event" } else { "New Event" };
    let action = if is_edit {
        format!("/web/events/{}/edit", event_id.unwrap())
    } else {
        "/web/events/new".to_string()
    };
    
    let event_title = event.as_ref().map(|e| e.title.clone()).unwrap_or_default();
    let description = event.as_ref().and_then(|e| e.description.clone()).unwrap_or_default();
    let location = event.as_ref().and_then(|e| e.location.clone()).unwrap_or_default();
    let start_time = event.as_ref()
        .map(|e| e.start_time.format("%Y-%m-%dT%H:%M").to_string())
        .unwrap_or_default();
    let end_time = event.as_ref()
        .map(|e| e.end_time.format("%Y-%m-%dT%H:%M").to_string())
        .unwrap_or_default();
    let is_all_day = event.as_ref().map(|e| e.is_all_day).unwrap_or(false);
    let calendar_id = event.as_ref()
        .map(|e| e.calendar_id)
        .or(selected_calendar_id)
        .unwrap_or_default();
    let calendars_clone = calendars.clone();
    
    rsx! {
        BaseLayout {
            current_user: Some(current_user),
            title: Some(format!("{} - My CalDAV Server", title)),
            
            div { class: "page-header",
                h1 { "{title}" }
            }

            div { class: "form-container",
                form { action: "{action}", method: "post",
                    div { class: "form-group",
                        label { r#for: "title", "Event Title *" }
                        input {
                            r#type: "text",
                            id: "title",
                            name: "title",
                            required: true,
                            value: "{event_title}",
                            placeholder: "Enter event title"
                        }
                    }
                    
                    div { class: "form-group",
                        label { r#for: "calendar_id", "Calendar *" }
                        select { id: "calendar_id", name: "calendar_id", required: true,
                            for cal in calendars_clone {
                                option { 
                                    value: "{cal.id}",
                                    selected: cal.id == calendar_id,
                                    "{cal.name}"
                                }
                            }
                        }
                    }
                    
                    div { class: "form-row",
                        div { class: "form-group",
                            label { r#for: "start_time", "Start Time *" }
                            input {
                                r#type: "datetime-local",
                                id: "start_time",
                                name: "start_time",
                                required: true,
                                value: "{start_time}"
                            }
                        }
                        div { class: "form-group",
                            label { r#for: "end_time", "End Time *" }
                            input {
                                r#type: "datetime-local",
                                id: "end_time",
                                name: "end_time",
                                required: true,
                                value: "{end_time}"
                            }
                        }
                    }
                    
                    div { class: "form-group",
                        label { class: "checkbox-label",
                            input {
                                r#type: "checkbox",
                                name: "is_all_day",
                                id: "is_all_day",
                                checked: is_all_day
                            }
                            span { "All-day event" }
                        }
                    }
                    
                    div { class: "form-group",
                        label { r#for: "location", "Location" }
                        input {
                            r#type: "text",
                            id: "location",
                            name: "location",
                            value: "{location}",
                            placeholder: "Enter location (optional)"
                        }
                    }
                    
                    div { class: "form-group",
                        label { r#for: "description", "Description" }
                        textarea {
                            id: "description",
                            name: "description",
                            rows: "4",
                            placeholder: "Enter event description (optional)",
                            "{description}"
                        }
                    }
                    
                    div { class: "form-actions",
                        a { href: "/web/dashboard", class: "btn btn-secondary", "Cancel" }
                        button { r#type: "submit", class: "btn btn-primary", 
                            if is_edit {
                                "Update Event"
                            } else {
                                "Create Event"
                            }
                        }
                    }
                }
            }

            if is_edit {
                if let Some(id) = event_id {
                    div { class: "danger-zone",
                        h3 { "Danger Zone" }
                        p { "Deleting this event cannot be undone." }
                        form { 
                            action: "/web/events/{id}/delete", 
                            method: "post",
                            button { 
                                r#type: "submit", 
                                class: "btn btn-danger",
                                "Delete Event" 
                            }
                        }
                    }
                }
            }
        }
    }
}
