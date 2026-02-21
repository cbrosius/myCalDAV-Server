use dioxus::prelude::*;

use crate::models::User;
use crate::ui::components::Navbar;

#[component]
pub fn BaseLayout(
    current_user: Option<User>,
    flash_message: Option<String>,
    flash_type: Option<String>,
    title: Option<String>,
    children: Element,
) -> Element {
    let page_title = title.unwrap_or_else(|| "My CalDAV Server".to_string());
    let ftype = flash_type.unwrap_or_else(|| "info".to_string());
    
    rsx! {
        head {
            meta { charset: "UTF-8" }
            meta { name: "viewport", content: "width=device-width, initial-scale=1.0" }
            title { "{page_title}" }
            link { rel: "stylesheet", href: "/static/css/style.css" }
        }
        body {
            Navbar { current_user: current_user.clone() }
            
            main { class: "container",
                if let Some(msg) = flash_message {
                    div { class: "flash-message flash-{ftype}", "{msg}" }
                }
                
                {children}
            }
            
            footer { class: "footer",
                p { "© 2024 My CalDAV Server" }
            }
        }
    }
}

#[component]
pub fn AuthLayout(children: Element) -> Element {
    rsx! {
        head {
            meta { charset: "UTF-8" }
            meta { name: "viewport", content: "width=device-width, initial-scale=1.0" }
            title { "My CalDAV Server" }
            link { rel: "stylesheet", href: "/static/css/style.css" }
        }
        body {
            Navbar { current_user: None }
            
            main { class: "container",
                {children}
            }
            
            footer { class: "footer",
                p { "© 2024 My CalDAV Server" }
            }
        }
    }
}

// Helper function to wrap content in full HTML document
pub fn wrap_html(content: &str) -> String {
    format!(r#"<!DOCTYPE html>
<html lang="en">
{}
</html>"#, content)
}
