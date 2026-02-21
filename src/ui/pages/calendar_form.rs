use dioxus::prelude::*;
use uuid::Uuid;

use crate::models::{User, Calendar};
use crate::ui::layouts::BaseLayout;

#[component]
pub fn CalendarFormPage(
    current_user: User,
    is_edit: bool,
    calendar_id: Option<Uuid>,
    calendar: Option<Calendar>,
) -> Element {
    let title = if is_edit { "Edit Calendar" } else { "New Calendar" };
    let action = if is_edit {
        format!("/web/calendars/{}", calendar_id.unwrap())
    } else {
        "/web/calendars/new".to_string()
    };
    
    let name = calendar.as_ref().map(|c| c.name.clone()).unwrap_or_default();
    let description = calendar.as_ref().and_then(|c| c.description.clone()).unwrap_or_default();
    let color = calendar.as_ref().and_then(|c| c.color.clone()).unwrap_or_else(|| "#3B82F6".to_string());
    let is_public = calendar.as_ref().map(|c| c.is_public).unwrap_or(false);
    
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
                        label { r#for: "name", "Calendar Name *" }
                        input {
                            r#type: "text",
                            id: "name",
                            name: "name",
                            required: true,
                            value: "{name}",
                            placeholder: "Enter calendar name"
                        }
                    }
                    
                    div { class: "form-group",
                        label { r#for: "description", "Description" }
                        textarea {
                            id: "description",
                            name: "description",
                            rows: "3",
                            placeholder: "Enter calendar description (optional)",
                            "{description}"
                        }
                    }
                    
                    div { class: "form-group",
                        label { r#for: "color", "Color" }
                        div { class: "color-picker",
                            input {
                                r#type: "color",
                                id: "color",
                                name: "color",
                                value: "{color}"
                            }
                        }
                    }
                    
                    div { class: "form-group",
                        label { class: "checkbox-label",
                            input {
                                r#type: "checkbox",
                                name: "is_public",
                                checked: is_public
                            }
                            span { "Make this calendar public" }
                        }
                        p { class: "form-hint", "Public calendars can be viewed by anyone with the link." }
                    }
                    
                    div { class: "form-actions",
                        a { href: "/web/calendars", class: "btn btn-secondary", "Cancel" }
                        button { r#type: "submit", class: "btn btn-primary", 
                            if is_edit {
                                "Update Calendar"
                            } else {
                                "Create Calendar"
                            }
                        }
                    }
                }
            }

            if is_edit {
                if let Some(id) = calendar_id {
                    div { class: "danger-zone",
                        h3 { "Danger Zone" }
                        p { "Deleting a calendar will permanently remove all its events. This action cannot be undone." }
                        form { 
                            action: "/web/calendars/{id}/delete", 
                            method: "post",
                            button { 
                                r#type: "submit", 
                                class: "btn btn-danger",
                                "Delete Calendar" 
                            }
                        }
                    }
                }
            }
        }
    }
}
