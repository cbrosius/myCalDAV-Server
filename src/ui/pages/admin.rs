use dioxus::prelude::*;
use crate::models::{User, UserRole};

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
        div {
            class: "min-h-screen bg-gray-100",
            
            // Flash message
            if let Some(message) = flash_message {
                div {
                    class: match flash_type.as_deref() {
                        Some("success") => "bg-green-100 border-l-4 border-green-500 text-green-700 p-4 mb-4",
                        Some("error") => "bg-red-100 border-l-4 border-red-500 text-red-700 p-4 mb-4",
                        _ => "bg-blue-100 border-l-4 border-blue-500 text-blue-700 p-4 mb-4",
                    },
                    p { "{message}" }
                }
            }
            
            // Header
            div {
                class: "bg-white shadow",
                div {
                    class: "max-w-7xl mx-auto px-4 py-6 sm:px-6 lg:px-8",
                    h1 {
                        class: "text-3xl font-bold text-gray-900",
                        "Admin Panel"
                    }
                    p {
                        class: "mt-1 text-sm text-gray-500",
                        "Manage users and system settings"
                    }
                }
            }
            
            // Content
            div {
                class: "max-w-7xl mx-auto px-4 py-8 sm:px-6 lg:px-8",
                
                // Stats
                div {
                    class: "grid grid-cols-1 md:grid-cols-3 gap-6 mb-8",
                    
                    div {
                        class: "bg-white rounded-lg shadow p-6",
                        div {
                            class: "flex items-center",
                            div {
                                class: "p-3 rounded-full bg-blue-100 text-blue-600",
                                svg {
                                    class: "w-6 h-6",
                                    fill: "none",
                                    stroke: "currentColor",
                                    view_box: "0 0 24 24",
                                    path {
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        d: "M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197m13.5-9a2.5 2.5 0 11-5 0 2.5 2.5 0 015 0z"
                                    }
                                }
                            }
                            div {
                                class: "ml-4",
                                p {
                                    class: "text-sm font-medium text-gray-500",
                                    "Total Users"
                                }
                                p {
                                    class: "text-2xl font-semibold text-gray-900",
                                    "{props.users.len()}"
                                }
                            }
                        }
                    }
                    
                    div {
                        class: "bg-white rounded-lg shadow p-6",
                        div {
                            class: "flex items-center",
                            div {
                                class: "p-3 rounded-full bg-green-100 text-green-600",
                                svg {
                                    class: "w-6 h-6",
                                    fill: "none",
                                    stroke: "currentColor",
                                    view_box: "0 0 24 24",
                                    path {
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        d: "M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
                                    }
                                }
                            }
                            div {
                                class: "ml-4",
                                p {
                                    class: "text-sm font-medium text-gray-500",
                                    "Admins"
                                }
                                p {
                                    class: "text-2xl font-semibold text-gray-900",
                                    "{props.users.iter().filter(|u| u.role == UserRole::Admin).count()}"
                                }
                            }
                        }
                    }
                    
                    div {
                        class: "bg-white rounded-lg shadow p-6",
                        div {
                            class: "flex items-center",
                            div {
                                class: "p-3 rounded-full bg-purple-100 text-purple-600",
                                svg {
                                    class: "w-6 h-6",
                                    fill: "none",
                                    stroke: "currentColor",
                                    view_box: "0 0 24 24",
                                    path {
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        d: "M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"
                                    }
                                }
                            }
                            div {
                                class: "ml-4",
                                p {
                                    class: "text-sm font-medium text-gray-500",
                                    "Regular Users"
                                }
                                p {
                                    class: "text-2xl font-semibold text-gray-900",
                                    "{props.users.iter().filter(|u| u.role == UserRole::User).count()}"
                                }
                            }
                        }
                    }
                }
                
                // Users Table
                div {
                    class: "bg-white shadow rounded-lg overflow-hidden",
                    div {
                        class: "px-6 py-4 border-b border-gray-200",
                        h2 {
                            class: "text-lg font-medium text-gray-900",
                            "User Management"
                        }
                    }
                    
                    table {
                        class: "min-w-full divide-y divide-gray-200",
                        thead {
                            class: "bg-gray-50",
                            tr {
                                th {
                                    class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider",
                                    "Name"
                                }
                                th {
                                    class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider",
                                    "Email"
                                }
                                th {
                                    class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider",
                                    "Role"
                                }
                                th {
                                    class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider",
                                    "Created"
                                }
                                th {
                                    class: "px-6 py-3 text-right text-xs font-medium text-gray-500 uppercase tracking-wider",
                                    "Actions"
                                }
                            }
                        }
                        tbody {
                            class: "bg-white divide-y divide-gray-200",
                            for user in props.users.iter() {
                                tr {
                                    td {
                                        class: "px-6 py-4 whitespace-nowrap",
                                        div {
                                            class: "text-sm font-medium text-gray-900",
                                            "{user.name}"
                                        }
                                    }
                                    td {
                                        class: "px-6 py-4 whitespace-nowrap",
                                        div {
                                            class: "text-sm text-gray-500",
                                            "{user.email}"
                                        }
                                    }
                                    td {
                                        class: "px-6 py-4 whitespace-nowrap",
                                        span {
                                            class: match user.role {
                                                UserRole::Admin => "px-2 inline-flex text-xs leading-5 font-semibold rounded-full bg-green-100 text-green-800",
                                                UserRole::User => "px-2 inline-flex text-xs leading-5 font-semibold rounded-full bg-blue-100 text-blue-800",
                                            },
                                            "{user.role}"
                                        }
                                    }
                                    td {
                                        class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500",
                                        "{user.created_at.format(\"%Y-%m-%d\")}"
                                    }
                                    td {
                                        class: "px-6 py-4 whitespace-nowrap text-right text-sm font-medium",
                                        if user.id != props.current_user.id {
                                            form {
                                                method: "post",
                                                action: "/web/admin/users/{user.id}/role",
                                                class: "inline",
                                                select {
                                                    name: "role",
                                                    class: "text-sm border rounded px-2 py-1 mr-2",
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
                                                    class: "text-blue-600 hover:text-blue-900",
                                                    "Update Role"
                                                }
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
                    class: "mt-6",
                    a {
                        href: "/web/dashboard",
                        class: "text-blue-600 hover:text-blue-900",
                        "← Back to Dashboard"
                    }
                }
            }
        }
    }
}
