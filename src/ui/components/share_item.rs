use dioxus::prelude::*;
use uuid::Uuid;

use crate::models::Share;

#[component]
pub fn ShareItem(share: Share) -> Element {
    let email = share.shared_with_email.clone().unwrap_or_else(|| "Unknown".to_string());
    
    rsx! {
        div { class: "share-item",
            div { class: "share-info",
                span { class: "share-email", "{email}" }
                span { class: "share-permission badge", "{share.permission_level}" }
            }
            form { action: "/web/shares/{share.id}/delete", method: "post", class: "inline-form",
                button { type: "submit", class: "btn btn-sm btn-danger", "Remove" }
            }
        }
    }
}
