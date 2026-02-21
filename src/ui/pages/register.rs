use dioxus::prelude::*;

use crate::ui::layouts::AuthLayout;

#[component]
pub fn RegisterPage(flash_message: Option<String>, flash_type: Option<String>) -> Element {
    rsx! {
        AuthLayout {
            div { class: "auth-container",
                div { class: "auth-card",
                    h1 { "Register" }
                    form { action: "/web/register", method: "post",
                        div { class: "form-group",
                            label { r#for: "name", "Name" }
                            input {
                                r#type: "text",
                                id: "name",
                                name: "name",
                                required: true,
                                placeholder: "Enter your name"
                            }
                        }
                        div { class: "form-group",
                            label { r#for: "email", "Email" }
                            input {
                                r#type: "email",
                                id: "email",
                                name: "email",
                                required: true,
                                placeholder: "Enter your email"
                            }
                        }
                        div { class: "form-group",
                            label { r#for: "password", "Password" }
                            input {
                                r#type: "password",
                                id: "password",
                                name: "password",
                                required: true,
                                placeholder: "Enter your password"
                            }
                        }
                        div { class: "form-group",
                            label { r#for: "confirm_password", "Confirm Password" }
                            input {
                                r#type: "password",
                                id: "confirm_password",
                                name: "confirm_password",
                                required: true,
                                placeholder: "Confirm your password"
                            }
                        }
                        button { r#type: "submit", class: "btn btn-primary", "Register" }
                    }
                    p { class: "auth-link",
                        "Already have an account? "
                        a { href: "/web/login", "Login here" }
                    }
                }
            }
        }
    }
}
