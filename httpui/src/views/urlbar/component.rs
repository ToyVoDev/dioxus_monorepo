use crate::components::select::{
    ButtonSelectTrigger, Select, SelectList, SelectOption, SelectValue,
};
use crate::components::{
    button::{Button, ButtonVariant},
    input::Input,
};
use crate::state::AppState;
use dioxus::prelude::*;
use dioxus_free_icons::icons::md_navigation_icons::MdUnfoldMore;
use strum::IntoEnumIterator;

#[derive(Debug, Clone, Copy, PartialEq, strum::EnumCount, strum::EnumIter, strum::Display)]
enum HttpMethods {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    OPTIONS,
    HEAD,
}

impl HttpMethods {
    fn from_str(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "POST" => Self::POST,
            "PUT" => Self::PUT,
            "PATCH" => Self::PATCH,
            "DELETE" => Self::DELETE,
            "OPTIONS" => Self::OPTIONS,
            "HEAD" => Self::HEAD,
            _ => Self::GET,
        }
    }
}

#[component]
pub fn Urlbar() -> Element {
    let mut app_state = use_context::<AppState>();

    let current_request = use_memo(move || {
        let selected_id = app_state.selected_request.read();
        selected_id.and_then(|id| {
            app_state
                .requests
                .read()
                .iter()
                .find(|r| r.id == id)
                .cloned()
        })
    });

    let mut method_value: Signal<Option<Option<HttpMethods>>> = use_signal(|| None);

    use_effect(move || {
        let new_val = current_request().map(|r| HttpMethods::from_str(&r.method));
        method_value.set(Some(new_val));
    });

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }

        div {
            id: "urlbar",
            if let Some(req) = current_request() {
                Select::<HttpMethods> {
                    value: method_value,
                    on_value_change: {
                        let req_id = req.id;
                        move |selection: Option<HttpMethods>| {
                            if let Some(selected) = selection {
                                let method_str = selected.to_string();
                                let mut requests = app_state.requests.write();
                                if let Some(r) = requests.iter_mut().find(|r| r.id == req_id) {
                                    r.method = method_str;
                                }
                            }
                        }
                    },
                    ButtonSelectTrigger {
                        aria_label: "HTTP Method",
                        variant: ButtonVariant::Ghost,
                        icon: MdUnfoldMore,
                        SelectValue { style: "font-size: small;" }
                    }
                    SelectList { aria_label: "HTTP Method",
                        {HttpMethods::iter().enumerate().map(|(i, option)| {
                            rsx! {
                                SelectOption::<HttpMethods> {
                                    index: i,
                                    value: option,
                                    text_value: option.to_string(),
                                    "{option}"
                                }
                            }
                        })}
                    }
                }
                Input {
                    value: "{req.url}",
                    placeholder: "Enter URL...",
                    oninput: {
                        let req_id = req.id;
                        move |e: FormEvent| {
                            let new_url = e.value();
                            let mut requests = app_state.requests.write();
                            if let Some(r) = requests.iter_mut().find(|r| r.id == req_id) {
                                r.url = new_url;
                            }
                        }
                    },
                }
                Button {
                    variant: ButtonVariant::Primary,
                    onclick: {
                        let req_method = req.method.clone();
                        let req_url = req.url.clone();
                        move |_| {
                            let method = req_method.clone();
                            let url = req_url.clone();
                            async move {
                                let client = reqwest::Client::new();
                                let result = match method.to_uppercase().as_str() {
                                    "POST" => client.post(&url).send().await,
                                    "PUT" => client.put(&url).send().await,
                                    "PATCH" => client.patch(&url).send().await,
                                    "DELETE" => client.delete(&url).send().await,
                                    "OPTIONS" => client.request(reqwest::Method::OPTIONS, &url).send().await,
                                    "HEAD" => client.head(&url).send().await,
                                    _ => client.get(&url).send().await,
                                };
                                match result {
                                    Ok(resp) => {
                                        let status = resp.status();
                                        match resp.text().await {
                                            Ok(body) => {
                                                let text = format!("HTTP {status}\n\n{body}");
                                                *app_state.response.write() = text;
                                            }
                                            Err(e) => {
                                                *app_state.response.write() = format!("Error reading response body: {e}");
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        *app_state.response.write() = format!("Request failed: {e}");
                                    }
                                }
                            }
                        }
                    },
                    "Send"
                }
            } else {
                span { class: "urlbar-placeholder", "Select a request to get started" }
            }
        }
    }
}
