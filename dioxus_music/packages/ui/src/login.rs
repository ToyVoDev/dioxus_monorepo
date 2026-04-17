use dioxus::prelude::*;

use crate::api_client::ApiClient;

const LOGIN_CSS: Asset = asset!("/assets/styling/login.css");

#[component]
pub fn LoginView(app_name: String, #[props(default = String::new())] subtitle: String) -> Element {
    let mut client_signal = use_context::<Signal<ApiClient>>();
    let mut username = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut error_msg = use_signal(|| None::<String>);
    let mut loading = use_signal(|| false);
    let mut show_password = use_signal(|| false);

    rsx! {
        document::Link {
            rel: "preconnect",
            href: "https://fonts.googleapis.com",
        }
        document::Link {
            rel: "stylesheet",
            href: "https://fonts.googleapis.com/css2?family=Space+Grotesk:wght@300;400;500;600;700&family=Inter:wght@300;400;500;600&family=JetBrains+Mono&display=swap",
        }
        document::Link {
            rel: "stylesheet",
            href: "https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@20..48,100..700,0..1,-50..200",
        }
        document::Link { rel: "stylesheet", href: LOGIN_CSS }
        div { class: "login-page",
            div { class: "login-blob" }
            main { class: "login-shell",
                div { class: "login-brand",
                    div { class: "login-brand__wordmark",
                        div { class: "login-brand__line" }
                        span { class: "login-brand__name", "{app_name}" }
                        div { class: "login-brand__line" }
                    }
                    if !subtitle.is_empty() {
                        p { class: "login-brand__subtitle", "{subtitle}" }
                    }
                }
                div { class: "login-card",
                    form {
                        class: "login-form",
                        onsubmit: move |e| {
                            e.prevent_default();
                            let u = username();
                            let p = password();
                            if u.is_empty() {
                                return;
                            }
                            loading.set(true);
                            error_msg.set(None);
                            spawn(async move {
                                let mut client = client_signal.read().clone();
                                match client.authenticate(&u, &p).await {
                                    Ok(_) => {
                                        *client_signal.write() = client;
                                    }
                                    Err(e) => {
                                        error_msg.set(Some(format!("Login failed: {e}")));
                                        loading.set(false);
                                    }
                                }
                            });
                        },
                        if let Some(err) = error_msg() {
                            p { class: "login-error", "{err}" }
                        }
                        div { class: "login-field",
                            label {
                                class: "login-field__label",
                                r#for: "username",
                                "System Identity / Username"
                            }
                            div { class: "login-field__input-wrap",
                                span { class: "material-symbols-outlined login-field__icon", "terminal" }
                                input {
                                    id: "username",
                                    class: "login-field__input",
                                    r#type: "text",
                                    placeholder: "username",
                                    value: username(),
                                    disabled: loading(),
                                    autocomplete: "username",
                                    oninput: move |e| username.set(e.value()),
                                }
                            }
                        }
                        div { class: "login-field",
                            label {
                                class: "login-field__label",
                                r#for: "password",
                                "Access Key / Password"
                            }
                            div { class: "login-field__input-wrap",
                                span { class: "material-symbols-outlined login-field__icon", "key" }
                                input {
                                    id: "password",
                                    class: "login-field__input",
                                    r#type: if show_password() { "text" } else { "password" },
                                    placeholder: "\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}",
                                    value: password(),
                                    disabled: loading(),
                                    autocomplete: "current-password",
                                    oninput: move |e| password.set(e.value()),
                                }
                                button {
                                    class: "login-field__toggle",
                                    r#type: "button",
                                    onclick: move |_| show_password.set(!show_password()),
                                    span { class: "material-symbols-outlined",
                                        if show_password() { "visibility_off" } else { "visibility" }
                                    }
                                }
                            }
                        }
                        div { class: "login-actions",
                            button {
                                class: "login-btn-primary",
                                r#type: "submit",
                                disabled: loading(),
                                if loading() {
                                    "Signing in\u{2026}"
                                } else {
                                    "SIGN IN"
                                    span {
                                        class: "material-symbols-outlined",
                                        style: "font-size: 1.25rem;",
                                        "arrow_forward"
                                    }
                                }
                            }
                        }
                    }
                }
            }
            div { class: "login-status",
                div { class: "login-status__dot" }
                span { class: "login-status__text", "System Status: Online" }
            }
        }
    }
}
