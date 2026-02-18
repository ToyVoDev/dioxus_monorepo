use crate::components::button::{Button, ButtonVariant};
use crate::components::select::{
    ButtonSelectTrigger, Select, SelectList, SelectOption, SelectValue,
};
use crate::state::AppState;
use dioxus::prelude::*;
use dioxus_free_icons::{
    Icon, icons::md_navigation_icons::MdUnfoldMore, icons::vsc_icons::VscAdd,
    icons::vsc_icons::VscClose,
};

#[component]
pub fn Tabbar() -> Element {
    let mut app_state = use_context::<AppState>();

    let requests = app_state.requests;
    let mut open_requests = app_state.open_requests;
    let mut selected_request = app_state.selected_request;
    let mut next_request_id = app_state.next_request_id;

    let open_tabs = use_memo(move || {
        let open_ids = open_requests();
        let all_requests = requests();
        all_requests
            .into_iter()
            .filter(|r| open_ids.contains(&r.id))
            .collect::<Vec<_>>()
    });

    let environments: Signal<Vec<String>> = use_signal(|| {
        vec![
            String::from("Development"),
            String::from("Staging"),
            String::from("Production"),
        ]
    });

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }

        div {
            id: "tabbar",
            div {
                class: "tabbar-tabs",
                for tab in open_tabs() {
                    {
                        let tab_id = tab.id;
                        let is_selected = selected_request() == Some(tab_id);
                        let label = if !tab.name.is_empty() {
                            tab.name.clone()
                        } else if !tab.url.is_empty() {
                            tab.url.clone()
                        } else {
                            "Untitled".to_string()
                        };

                        rsx! {
                            div {
                                class: if is_selected { "tab selected" } else { "tab" },
                                Button {
                                    variant: ButtonVariant::Ghost,
                                    onclick: move |_| {
                                        selected_request.set(Some(tab_id));
                                    },
                                    "{label}"
                                }
                                Button {
                                    variant: ButtonVariant::Ghost,
                                    onclick: move |_| {
                                        let mut open = open_requests.write();
                                        if let Some(pos) = open.iter().position(|&id| id == tab_id) {
                                            open.remove(pos);

                                            // If we're closing the selected tab, select a neighbor
                                            if selected_request() == Some(tab_id) {
                                                let next = if pos < open.len() {
                                                    Some(open[pos])
                                                } else if !open.is_empty() {
                                                    Some(open[open.len() - 1])
                                                } else {
                                                    None
                                                };
                                                selected_request.set(next);
                                            }
                                        }
                                    },
                                    Icon {
                                        width: 12,
                                        height: 12,
                                        fill: "inherit",
                                        icon: VscClose,
                                    }
                                }
                            }
                        }
                    }
                }
            }
            div {
                class: "tabbar-add",
                Button {
                    variant: ButtonVariant::Ghost,
                    onclick: move |_| {
                        let id = *next_request_id.read();
                        let request = crate::state::Request {
                            id,
                            collection_id: None,
                            name: String::new(),
                            method: "GET".to_string(),
                            url: String::new(),
                            inherit_cookies_header: false,
                            inherit_authorization_header: false,
                        };
                        app_state.requests.write().push(request);
                        next_request_id.with_mut(|nid| *nid += 1);
                        app_state.open_requests.write().push(id);
                        selected_request.set(Some(id));
                    },
                    Icon {
                        width: 12,
                        height: 12,
                        fill: "inherit",
                        icon: VscAdd,
                    }
                }
            }
            Select::<Option<String>> {
                on_value_change: move |selection: Option<Option<String>>| {
                    if let Some(Some(selected)) = selection {
                        println!("Selected: {}", selected);
                    }
                },
                ButtonSelectTrigger {
                    aria_label: "Select Environment",
                    variant: ButtonVariant::Ghost,
                    icon: MdUnfoldMore,
                    SelectValue { style: "font-size: small;" }
                }
                SelectList { aria_label: "Select Environment",
                    SelectOption::<Option<String>> {
                        index: environments().len() - environments().len(),
                        value: None,
                        text_value: String::from("Environments"),
                        "No Environment"
                    }
                    for (i, option) in environments().iter().enumerate() {
                        SelectOption::<Option<String>> {
                            index: i + 1,
                            value: Some(option.clone()),
                            text_value: option.clone(),
                            "{option}"
                        }
                    }
                    SelectOption::<Option<String>> {
                        index: environments().len() + 1,
                        value: None,
                        text_value: String::from("Create Environment..."),
                        "Create Environment..."
                    }
                }
            }
        }
    }
}
