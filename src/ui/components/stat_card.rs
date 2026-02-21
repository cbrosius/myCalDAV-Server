use dioxus::prelude::*;

#[component]
pub fn StatCard(icon: String, number: usize, label: String) -> Element {
    rsx! {
        div { class: "stat-card",
            div { class: "stat-icon", "{icon}" }
            div { class: "stat-info",
                span { class: "stat-number", "{number}" }
                span { class: "stat-label", "{label}" }
            }
        }
    }
}
