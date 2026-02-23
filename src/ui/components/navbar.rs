use dioxus::prelude::*;

use crate::models::{User, UserRole};

#[component]
pub fn Navbar(current_user: Option<User>) -> Element {
    rsx! {
        nav { class: "navbar",
            div { class: "nav-brand",
                a { href: "/", "My CalDAV Server" }
            }
            div { class: "nav-menu",
                if let Some(user) = current_user {
                    a { href: "/web/dashboard", "Dashboard" }
                    a { href: "/web/calendars", "Calendars" }
                    a { href: "/web/events", "Events" }
                    if user.role == UserRole::Admin {
                        a { href: "/web/admin", class: "nav-admin", "Admin" }
                    }
                    a { href: "/web/logout", class: "nav-logout", "Logout" }
                } else {
                    a { href: "/web/login", "Login" }
                    a { href: "/web/register", "Register" }
                }
            }
        }
    }
}
