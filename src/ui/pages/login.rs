use dioxus::prelude::*;

use crate::ui::layouts::AuthLayout;

#[component]
pub fn LoginPage(flash_message: Option<String>, flash_type: Option<String>) -> Element {
    rsx! {
        AuthLayout {
            div { class: "auth-container",
                div { class: "auth-card",
                    h1 { "Login" }
                    form { action: "/web/login", method: "post",
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
                        button { r#type: "submit", class: "btn btn-primary", "Login" }
                    }
                    p { class: "auth-link",
                        "Don't have an account? "
                        a { href: "/web/register", "Register here" }
                    }
                }
            }
        }
    }
}
