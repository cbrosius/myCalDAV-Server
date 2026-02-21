use dioxus::prelude::*;

#[component]
pub fn FlashMessage(message: String, flash_type: String) -> Element {
    rsx! {
        div { class: "flash-message flash-{flash_type}",
            "{message}"
        }
    }
}
