use crate::state::{AppState, EditorTab, HttpResponse, KeyValue};
use dioxus::prelude::*;
use dioxus_primitives::tabs;
use kinetic_ui::{
    Badge, BadgeVariant, Button, ButtonVariant, IconButton, Input, KSelect, KSelectList,
    KSelectOption, KSelectTrigger, KSelectValue, KTable, KTableAddRow, KTableCell, KTableHeader,
    KTableInput, KTableRow,
};
use strum::IntoEnumIterator;

#[allow(clippy::upper_case_acronyms)]
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

#[allow(clippy::cast_precision_loss)]
fn format_bytes(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{bytes} B")
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    }
}

const fn status_badge_variant(status: u16) -> BadgeVariant {
    match status {
        200..=299 => BadgeVariant::Secondary,
        300..=399 => BadgeVariant::Tertiary,
        400..=499 => BadgeVariant::Primary,
        _ => BadgeVariant::Error,
    }
}

#[component]
pub fn Canvas() -> Element {
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

    let mut response_expanded = use_signal(|| true);

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        // Load tabs CSS — canvas uses dioxus_primitives::tabs directly with k-tabs classes
        kinetic_ui::TabsStylesheet {}

        div { class: "canvas",
            if let Some(req) = current_request() {
                // URL Bar
                div { class: "canvas__urlbar",
                    KSelect::<HttpMethods> {
                        value: method_value,
                        on_value_change: {
                            let req_id = req.id;
                            move |selection: Option<HttpMethods>| {
                                if let Some(selected) = selection {
                                    let method_str = selected.to_string();
                                    app_state.requests.with_mut(|reqs| {
                                        if let Some(r) = reqs.iter_mut().find(|r| r.id == req_id) {
                                            r.method = method_str;
                                        }
                                    });
                                }
                            }
                        },
                        KSelectTrigger {
                            aria_label: "HTTP Method",
                            KSelectValue { style: "font-size: small;" }
                        }
                        KSelectList { aria_label: "HTTP Method",
                            {HttpMethods::iter().enumerate().map(|(i, option)| {
                                rsx! {
                                    KSelectOption::<HttpMethods> {
                                        index: i,
                                        value: option,
                                        text_value: option.to_string(),
                                        "{option}"
                                    }
                                }
                            })}
                        }
                    }
                    div { class: "canvas__url-input",
                        Input {
                            monospace: true,
                            value: req.url.clone(),
                            placeholder: "Enter URL...".to_string(),
                            oninput: {
                                let req_id = req.id;
                                move |e: FormEvent| {
                                    let new_url = e.value();
                                    app_state.requests.with_mut(|reqs| {
                                        if let Some(r) = reqs.iter_mut().find(|r| r.id == req_id) {
                                            r.url = new_url;
                                        }
                                    });
                                }
                            },
                        }
                    }
                    Button {
                        variant: ButtonVariant::Primary,
                        onclick: {
                            let req_method = req.method.clone();
                            let req_url = req.url.clone();
                            let req_params = req.params.clone();
                            let req_headers = req.headers.clone();
                            let req_body = req.body.clone();
                            move |_| {
                                let method = req_method.clone();
                                let url = req_url.clone();
                                let params = req_params.clone();
                                let headers = req_headers.clone();
                                let body = req_body.clone();
                                spawn(async move {
                                    send_request(
                                        app_state, method, url, params, headers, body,
                                    )
                                    .await;
                                });
                            }
                        },
                        "Send \u{2192}"
                    }
                }

                // Editor Tabs
                div { class: "canvas__editor",
                    {render_editor_tabs(app_state, req.id, req.params.clone(), req.headers)}
                }

                // Response Section
                div { class: "canvas__response",
                    div {
                        class: "canvas__response-header",
                        onclick: move |_| {
                            let current = *response_expanded.read();
                            response_expanded.set(!current);
                        },
                        svg {
                            class: "canvas__chevron",
                            "data-expanded": "{response_expanded}",
                            width: "16",
                            height: "16",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            polyline { points: "9 6 15 12 9 18" }
                        }
                        span { class: "canvas__response-label", "Response" }
                        if let Some(resp) = app_state.http_response.read().as_ref() {
                            div { class: "canvas__response-meta",
                                Badge {
                                    variant: status_badge_variant(resp.status),
                                    "{resp.status} {resp.status_text}"
                                }
                                span { "{resp.time_ms}ms" }
                                span { "{format_bytes(resp.size_bytes)}" }
                            }
                        }
                    }
                    if *response_expanded.read() {
                        if let Some(resp) = app_state.http_response.read().as_ref() {
                            pre { class: "canvas__response-body",
                                {resp.body.lines().enumerate().map(|(i, line)| {
                                    let line_num = i + 1;
                                    let line_owned = line.to_string();
                                    rsx! {
                                        span { class: "canvas__response-line-num", "{line_num}" }
                                        "{line_owned}\n"
                                    }
                                })}
                            }
                        } else {
                            div { class: "canvas__tab-placeholder",
                                "No response yet \u{2014} send a request"
                            }
                        }
                    }
                }
            } else {
                div { class: "canvas__placeholder",
                    "Select or create a request to get started"
                }
            }
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
fn render_editor_tabs(
    mut app_state: AppState,
    request_id: i32,
    params: Vec<KeyValue>,
    headers: Vec<KeyValue>,
) -> Element {
    let active_tab = app_state.active_editor_tab.read().to_string();
    let headers_count = headers.len();

    rsx! {
        tabs::Tabs {
            class: "k-tabs",
            default_value: active_tab,
            on_value_change: move |val: String| {
                let tab = match val.as_str() {
                    "Authorization" => EditorTab::Authorization,
                    "Headers" => EditorTab::Headers,
                    "Body" => EditorTab::Body,
                    "Settings" => EditorTab::Settings,
                    _ => EditorTab::Params,
                };
                app_state.active_editor_tab.set(tab);
            },
            tabs::TabList {
                class: "k-tabs__list",
                for (i, tab) in EditorTab::iter().enumerate() {
                    tabs::TabTrigger {
                        class: "k-tabs__trigger",
                        value: tab.to_string(),
                        index: i,
                        "{tab}"
                        if tab == EditorTab::Headers && headers_count > 0 {
                            span { class: "k-badge", "data-variant": "muted", "{headers_count}" }
                        }
                    }
                }
            }

            tabs::TabContent {
                class: "k-tabs__content",
                value: "Params".to_string(),
                index: 0usize,
                div { class: "canvas__tab-content",
                    {render_kv_table(app_state, request_id, &params, KvKind::Params)}
                }
            }

            tabs::TabContent {
                class: "k-tabs__content",
                value: "Authorization".to_string(),
                index: 1usize,
                div { class: "canvas__tab-placeholder",
                    "Authorization configuration coming soon"
                }
            }

            tabs::TabContent {
                class: "k-tabs__content",
                value: "Headers".to_string(),
                index: 2usize,
                div { class: "canvas__tab-content",
                    {render_kv_table(app_state, request_id, &headers, KvKind::Headers)}
                }
            }

            tabs::TabContent {
                class: "k-tabs__content",
                value: "Body".to_string(),
                index: 3usize,
                div { class: "canvas__tab-placeholder",
                    "Request body editor coming soon"
                }
            }

            tabs::TabContent {
                class: "k-tabs__content",
                value: "Settings".to_string(),
                index: 4usize,
                div { class: "canvas__tab-placeholder",
                    "Request settings coming soon"
                }
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum KvKind {
    Params,
    Headers,
}

fn render_kv_table(
    mut app_state: AppState,
    request_id: i32,
    items: &[KeyValue],
    kind: KvKind,
) -> Element {
    rsx! {
        KTable {
            KTableHeader { columns: vec!["KEY".to_string(), "VALUE".to_string(), "DESCRIPTION".to_string(), String::new()] }
            tbody {
                for item in items.iter() {
                    {render_kv_row(app_state, request_id, item, kind)}
                }
            }
        }
        KTableAddRow {
            onclick: move |_| {
                app_state.requests.with_mut(|reqs| {
                    if let Some(req) = reqs.iter_mut().find(|r| r.id == request_id) {
                        let next_id = match kind {
                            KvKind::Params => req.params.iter().map(|p| p.id).max().unwrap_or(0) + 1,
                            KvKind::Headers => req.headers.iter().map(|h| h.id).max().unwrap_or(0) + 1,
                        };
                        let new_kv = KeyValue {
                            id: next_id,
                            key: String::new(),
                            value: String::new(),
                            description: String::new(),
                            enabled: true,
                        };
                        match kind {
                            KvKind::Params => req.params.push(new_kv),
                            KvKind::Headers => req.headers.push(new_kv),
                        }
                    }
                });
            },
        }
    }
}

fn render_kv_row(
    mut app_state: AppState,
    request_id: i32,
    item: &KeyValue,
    kind: KvKind,
) -> Element {
    let item_id = item.id;

    rsx! {
        KTableRow {
            KTableCell {
                KTableInput {
                    value: item.key.clone(),
                    placeholder: Some("Key".to_string()),
                    oninput: move |e: FormEvent| {
                        let new_key = e.value();
                        app_state.requests.with_mut(|reqs| {
                            if let Some(req) = reqs.iter_mut().find(|r| r.id == request_id) {
                                let items = match kind {
                                    KvKind::Params => &mut req.params,
                                    KvKind::Headers => &mut req.headers,
                                };
                                if let Some(kv) = items.iter_mut().find(|kv| kv.id == item_id) {
                                    kv.key = new_key;
                                }
                            }
                        });
                    },
                }
            }
            KTableCell {
                KTableInput {
                    value: item.value.clone(),
                    placeholder: Some("Value".to_string()),
                    oninput: move |e: FormEvent| {
                        let new_val = e.value();
                        app_state.requests.with_mut(|reqs| {
                            if let Some(req) = reqs.iter_mut().find(|r| r.id == request_id) {
                                let items = match kind {
                                    KvKind::Params => &mut req.params,
                                    KvKind::Headers => &mut req.headers,
                                };
                                if let Some(kv) = items.iter_mut().find(|kv| kv.id == item_id) {
                                    kv.value = new_val;
                                }
                            }
                        });
                    },
                }
            }
            KTableCell {
                KTableInput {
                    value: item.description.clone(),
                    placeholder: Some("Description".to_string()),
                    oninput: move |e: FormEvent| {
                        let new_desc = e.value();
                        app_state.requests.with_mut(|reqs| {
                            if let Some(req) = reqs.iter_mut().find(|r| r.id == request_id) {
                                let items = match kind {
                                    KvKind::Params => &mut req.params,
                                    KvKind::Headers => &mut req.headers,
                                };
                                if let Some(kv) = items.iter_mut().find(|kv| kv.id == item_id) {
                                    kv.description = new_desc;
                                }
                            }
                        });
                    },
                }
            }
            KTableCell {
                IconButton {
                    onclick: move |_| {
                        app_state.requests.with_mut(|reqs| {
                            if let Some(req) = reqs.iter_mut().find(|r| r.id == request_id) {
                                let items = match kind {
                                    KvKind::Params => &mut req.params,
                                    KvKind::Headers => &mut req.headers,
                                };
                                items.retain(|kv| kv.id != item_id);
                            }
                        });
                    },
                    svg {
                        width: "14",
                        height: "14",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        line { x1: "18", y1: "6", x2: "6", y2: "18" }
                        line { x1: "6", y1: "6", x2: "18", y2: "18" }
                    }
                }
            }
        }
    }
}

#[allow(clippy::future_not_send)]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::large_types_passed_by_value)]
async fn send_request(
    mut app_state: AppState,
    method: String,
    url: String,
    params: Vec<KeyValue>,
    headers: Vec<KeyValue>,
    body: Option<String>,
) {
    let start = std::time::Instant::now();
    let client = reqwest::Client::new();

    // Build URL with query params
    let request_url = reqwest::Url::parse(&url).map_or_else(
        |_| url.clone(),
        |mut parsed| {
            let enabled_params: Vec<_> = params
                .iter()
                .filter(|p| p.enabled && !p.key.is_empty())
                .collect();
            if !enabled_params.is_empty() {
                let mut query_pairs = parsed.query_pairs_mut();
                for p in &enabled_params {
                    query_pairs.append_pair(&p.key, &p.value);
                }
                drop(query_pairs);
            }
            parsed.to_string()
        },
    );

    let mut req_builder = match method.to_uppercase().as_str() {
        "POST" => client.post(&request_url),
        "PUT" => client.put(&request_url),
        "PATCH" => client.patch(&request_url),
        "DELETE" => client.delete(&request_url),
        "OPTIONS" => client.request(reqwest::Method::OPTIONS, &request_url),
        "HEAD" => client.head(&request_url),
        _ => client.get(&request_url),
    };

    // Add custom headers
    for h in headers.iter().filter(|h| h.enabled && !h.key.is_empty()) {
        req_builder = req_builder.header(&h.key, &h.value);
    }

    // Add body if present
    if let Some(body_str) = &body
        && !body_str.is_empty()
    {
        req_builder = req_builder.body(body_str.clone());
    }

    let result = req_builder.send().await;
    let elapsed = start.elapsed().as_millis() as u64;

    match result {
        Ok(resp) => {
            let status = resp.status().as_u16();
            let status_text = resp.status().canonical_reason().unwrap_or("").to_string();
            let resp_headers: Vec<(String, String)> = resp
                .headers()
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
                .collect();

            match resp.text().await {
                Ok(body_text) => {
                    let size_bytes = body_text.len() as u64;
                    let http_resp = HttpResponse {
                        status,
                        status_text,
                        body: body_text,
                        headers: resp_headers,
                        time_ms: elapsed,
                        size_bytes,
                    };
                    app_state.http_response.set(Some(http_resp));
                }
                Err(e) => {
                    eprintln!("Error reading response body: {e}");
                    app_state.http_response.set(None);
                }
            }
        }
        Err(e) => {
            eprintln!("Request failed: {e}");
            app_state.http_response.set(None);
        }
    }
}
