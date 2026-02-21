use dioxus::prelude::*;

#[component]
pub fn TextInput(
    name: String,
    label: String,
    value: Option<String>,
    placeholder: Option<String>,
    required: bool,
    input_type: Option<String>,
) -> Element {
    let input_type = input_type.unwrap_or_else(|| "text".to_string());
    let value = value.unwrap_or_default();
    let placeholder = placeholder.unwrap_or_default();
    
    rsx! {
        div { class: "form-group",
            label { r#for: "{name}", "{label}" }
            input {
                type: "{input_type}",
                id: "{name}",
                name: "{name}",
                required: required,
                value: "{value}",
                placeholder: "{placeholder}"
            }
        }
    }
}

#[component]
pub fn TextArea(
    name: String,
    label: String,
    value: Option<String>,
    placeholder: Option<String>,
    rows: Option<i32>,
) -> Element {
    let value = value.unwrap_or_default();
    let placeholder = placeholder.unwrap_or_default();
    let rows = rows.unwrap_or(3);
    
    rsx! {
        div { class: "form-group",
            label { r#for: "{name}", "{label}" }
            textarea {
                id: "{name}",
                name: "{name}",
                rows: "{rows}",
                placeholder: "{placeholder}",
                "{value}"
            }
        }
    }
}

#[component]
pub fn Select(
    name: String,
    label: String,
    options: Vec<(String, String)>,
    selected: Option<String>,
    required: bool,
) -> Element {
    rsx! {
        div { class: "form-group",
            label { r#for: "{name}", "{label}" }
            select {
                id: "{name}",
                name: "{name}",
                required: required,
                for (value, text) in options {
                    option {
                        value: "{value}",
                        selected: selected.as_ref().map_or(false, |s| s == &value),
                        "{text}"
                    }
                }
            }
        }
    }
}

#[component]
pub fn Checkbox(
    name: String,
    label: String,
    checked: bool,
) -> Element {
    rsx! {
        div { class: "form-group",
            label { class: "checkbox-label",
                input {
                    r#type: "checkbox",
                    name: "{name}",
                    checked: checked
                }
                span { "{label}" }
            }
        }
    }
}

#[component]
pub fn ColorInput(
    name: String,
    label: String,
    value: Option<String>,
) -> Element {
    let value = value.unwrap_or_else(|| "#3B82F6".to_string());
    
    rsx! {
        div { class: "form-group",
            label { r#for: "{name}", "{label}" }
            div { class: "color-picker",
                input {
                    r#type: "color",
                    id: "{name}",
                    name: "{name}",
                    value: "{value}"
                }
            }
        }
    }
}

#[component]
pub fn DateTimeInput(
    name: String,
    label: String,
    value: Option<String>,
    required: bool,
) -> Element {
    let value = value.unwrap_or_default();
    
    rsx! {
        div { class: "form-group",
            label { r#for: "{name}", "{label}" }
            input {
                r#type: "datetime-local",
                id: "{name}",
                name: "{name}",
                required: required,
                value: "{value}"
            }
        }
    }
}
