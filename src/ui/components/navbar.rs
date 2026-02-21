use dioxus::prelude::*;

use crate::models::User;

#[component]
pub fn Navbar(current_user: Option<User>) -> Element {
    rsx! {
        nav { class: "navbar",
            div { class: "nav-brand",
                a { href: "/", "My CalDAV Server" }
            }
            div { class: "nav-menu",
                if let Some(_user) = current_user {
                    a { href: "/web/dashboard", "Dashboard" }
                    a { href: "/web/calendars", "Calendars" }
                    a { href: "/web/events", "Events" }
                    a { href: "/web/logout", class: "nav-logout", "Logout" }
                } else {
                    a { href: "/web/login", "Login" }
                    a { href: "/web/register", "Register" }
                }
            }
        }
    }
}
