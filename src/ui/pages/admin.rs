use dioxus::prelude::*;
use crate::models::{User, UserRole};
use crate::ui::layouts::BaseLayout;

#[derive(Props, PartialEq, Clone)]
pub struct AdminPageProps {
    pub current_user: User,
    pub users: Vec<User>,
    pub flash_message: Option<String>,
    pub flash_type: Option<String>,
}

#[allow(non_snake_case)]
pub fn AdminPage(props: AdminPageProps) -> Element {
    let flash_message = props.flash_message.clone();
    let flash_type = props.flash_type.clone();
    
    rsx! {
        BaseLayout {
            current_user: Some(props.current_user.clone()),
            title: Some("Admin Panel - My CalDAV Server".to_string()),
            flash_message: flash_message,
            flash_type: flash_type,
            
            div {
                class: "admin-page",
                
                // Page header
                div {
                    class: "page-header",
                    div {
                        h1 { "Admin Panel" }
                        p {
                            class: "subtitle",
                            "Manage users and system settings"
                        }
                    }
                }
                
                // Stats
                div {
                    class: "dashboard-stats",
                    
                    div {
                        class: "stat-card",
                        div {
                            class: "stat-icon",
                            "👥"
                        }
                        div {
                            class: "stat-info",
                            span {
                                class: "stat-number",
                                "{props.users.len()}"
                            }
                            span {
                                class: "stat-label",
                                "Total Users"
                            }
                        }
                    }
                    
                    div {
                        class: "stat-card",
                        div {
                            class: "stat-icon",
                            "🔑"
                        }
                        div {
                            class: "stat-info",
                            span {
                                class: "stat-number",
                                "{props.users.iter().filter(|u| u.role == UserRole::Admin).count()}"
                            }
                            span {
                                class: "stat-label",
                                "Admins"
                            }
                        }
                    }
                    
                    div {
                        class: "stat-card",
                        div {
                            class: "stat-icon",
                            "👤"
                        }
                        div {
                            class: "stat-info",
                            span {
                                class: "stat-number",
                                "{props.users.iter().filter(|u| u.role == UserRole::User).count()}"
                            }
                            span {
                                class: "stat-label",
                                "Regular Users"
                            }
                        }
                    }
                }
                
                // Users Table
                div {
                    class: "dashboard-section",
                    
                    div {
                        class: "section-header",
                        h2 { "User Management" }
                    }
                    
                    table {
                        class: "admin-table",
                        thead {
                            tr {
                                th { "Name" }
                                th { "Email" }
                                th { "Role" }
                                th { "Created" }
                                th { "Actions" }
                            }
                        }
                        tbody {
                            for user in props.users.iter() {
                                tr {
                                    td {
                                        class: "user-name",
                                        "{user.name}"
                                    }
                                    td {
                                        class: "user-email",
                                        "{user.email}"
                                    }
                                    td {
                                        span {
                                            class: match user.role {
                                                UserRole::Admin => "badge badge-admin",
                                                UserRole::User => "badge badge-user",
                                            },
                                            "{user.role}"
                                        }
                                    }
                                    td {
                                        class: "user-created",
                                        "{user.created_at.format(\"%Y-%m-%d\")}"
                                    }
                                    td {
                                        class: "user-actions",
                                        if user.id != props.current_user.id {
                                            form {
                                                method: "post",
                                                action: "/web/admin/users/{user.id}/role",
                                                class: "inline-form",
                                                select {
                                                    name: "role",
                                                    class: "role-select",
                                                    option {
                                                        value: "user",
                                                        selected: user.role == UserRole::User,
                                                        "User"
                                                    }
                                                    option {
                                                        value: "admin",
                                                        selected: user.role == UserRole::Admin,
                                                        "Admin"
                                                    }
                                                }
                                                button {
                                                    type: "submit",
                                                    class: "btn btn-primary btn-sm",
                                                    "Update"
                                                }
                                            }
                                        } else {
                                            span {
                                                class: "text-muted",
                                                "(Current user)"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                
                // Back to dashboard link
                div {
                    class: "back-link",
                    a {
                        href: "/web/dashboard",
                        class: "btn btn-outline",
                        "← Back to Dashboard"
                    }
                }
            }
        }
    }
}
