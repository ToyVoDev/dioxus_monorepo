use crate::Route;
use crate::components::accordion::{Accordion, AccordionContent, AccordionItem, AccordionTrigger};
use crate::components::button::{ButtonVariant, LinkButton};
use crate::components::icon_select_trigger::IconSelectTrigger;
use crate::components::select::{
    Select, SelectGroup, SelectGroupLabel, SelectItemIndicator, SelectList, SelectOption,
    SelectValue,
};
use crate::state::AppState;
use dioxus::prelude::*;
use dioxus_free_icons::icons::md_navigation_icons::MdUnfoldMore;
use dioxus_free_icons::{Icon, icons::vsc_icons::VscAdd};
use strum::IntoEnumIterator;

#[derive(Debug, Clone, Copy, PartialEq, strum::EnumCount, strum::EnumIter)]
enum CreateNewOptions {
    DraftRequest,
    SavedRequest,
    Collection,
    Space,
    Import,
}

impl std::fmt::Display for CreateNewOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateNewOptions::DraftRequest => write!(f, "Draft Request"),
            CreateNewOptions::SavedRequest => write!(f, "Saved Request..."),
            CreateNewOptions::Collection => write!(f, "Collection..."),
            CreateNewOptions::Space => write!(f, "Space..."),
            CreateNewOptions::Import => write!(f, "Import..."),
        }
    }
}

#[component]
pub fn Library() -> Element {
    // Get the app state from context
    let app_state = use_context::<AppState>();

    // Extract signals to avoid borrow checker issues
    let mut spaces = app_state.spaces;
    let mut next_space_id = app_state.next_space_id;
    let mut collections = app_state.collections;
    let mut next_collection_id = app_state.next_collection_id;
    let mut requests = app_state.requests;
    let mut next_request_id = app_state.next_request_id;
    let mut open_requests = app_state.open_requests;
    let mut selected_request = app_state.selected_request;

    let mut selected_space_id: Signal<Option<Option<i32>>> = use_signal(|| None);

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        div {
            id: "library",
            Select::<i32> {
                placeholder: "Select a space...",
                value: selected_space_id,
                on_value_change: move |new_space_id: Option<i32>| {
                    if let Some(space_id) = new_space_id {
                        if space_id == -1 {
                            // Create new space (sentinel value)
                            let id = *next_space_id.read();
                            let space = crate::state::Space {
                                id,
                                name: "New Space".to_string(),
                                icon: None,
                                color: None,
                                environments: Vec::new(),
                                variables: Vec::new(),
                            };
                            spaces.with_mut(|spaces| {
                                spaces.push(space);
                            });
                            next_space_id.with_mut(|id| {
                                *id += 1;
                            });
                            selected_space_id.set(Some(Some(id)));
                        } else {
                            selected_space_id.set(Some(Some(space_id)));
                        }
                    }
                },
                IconSelectTrigger {
                    id: "space-selector-trigger",
                    aria_label: "Select Space",
                    variant: ButtonVariant::Ghost,
                    icon: MdUnfoldMore,
                    LinkButton {
                        variant: ButtonVariant::Ghost,
                        to: Route::SpaceSection {
                            id: selected_space_id().and_then(|x| x).unwrap_or(0)
                        },
                        draggable: false,
                        SelectValue { style: "font-size: small;" }
                    }
                }
                SelectList { aria_label: "Select Space",
                    SelectGroup {
                        SelectGroupLabel { "Switch space" }
                        for (i, space) in spaces.read().iter().enumerate() {
                            SelectOption::<i32> {
                                index: i,
                                value: space.id,
                                text_value: space.name.clone(),
                                SelectItemIndicator {}
                                "{space.name}"
                            }
                        }
                        SelectOption::<i32> {
                            index: spaces.read().len(),
                            value: -1, // Sentinel value for create option
                            text_value: "Create space...",
                            Icon {
                                width: 16,
                                height: 16,
                                fill: "inherit",
                               icon: VscAdd,
                            }
                            "Create..."
                        }
                    }
                }
            }
            Select::<CreateNewOptions> {
                on_value_change: move |selection: Option<CreateNewOptions>| {
                    if let Some(selected) = selection {
                        match selected {
                            CreateNewOptions::Space => {
                                // Create space by directly accessing signals
                                let id = *next_space_id.read();
                                let space = crate::state::Space {
                                    id,
                                    name: "New Space".to_string(),
                                    icon: None,
                                    color: None,
                                    environments: Vec::new(),
                                    variables: Vec::new(),
                                };
                                spaces.with_mut(|spaces| {
                                    spaces.push(space);
                                });
                                next_space_id.with_mut(|id| {
                                    *id += 1;
                                });
                                selected_space_id.set(Some(Some(id)));
                            }
                            CreateNewOptions::Collection => {
                                if let Some(Some(space_id)) = selected_space_id() {
                                    // Create collection by directly accessing signals
                                    let id = *next_collection_id.read();
                                    let collection = crate::state::Collection {
                                        id,
                                        space_id,
                                        name: "New Collection".to_string(),
                                        icon: None,
                                        color: None,
                                    };
                                    collections.with_mut(|collections| {
                                        collections.push(collection);
                                    });
                                    next_collection_id.with_mut(|id| {
                                        *id += 1;
                                    });
                                }
                            }
                            CreateNewOptions::SavedRequest => {
                                if let Some(Some(_space_id)) = selected_space_id() {
                                    // Create a request without a collection (draft)
                                    let id = *next_request_id.read();
                                    let request = crate::state::Request {
                                        id,
                                        collection_id: None,
                                        name: "New Request".to_string(),
                                        method: "GET".to_string(),
                                        url: "https://example.com".to_string(),
                                        inherit_cookies_header: false,
                                        inherit_authorization_header: false,
                                    };
                                    requests.with_mut(|requests| {
                                        requests.push(request);
                                    });
                                    next_request_id.with_mut(|id| {
                                        *id += 1;
                                    });
                                    open_requests.with_mut(|open| open.push(id));
                                    selected_request.set(Some(id));
                                }
                            }
                            CreateNewOptions::DraftRequest => {
                                // Create a draft request (no collection)
                                let id = *next_request_id.read();
                                let request = crate::state::Request {
                                    id,
                                    collection_id: None,
                                    name: "Draft Request".to_string(),
                                    method: "GET".to_string(),
                                    url: "https://example.com".to_string(),
                                    inherit_cookies_header: false,
                                    inherit_authorization_header: false,
                                };
                                requests.with_mut(|requests| {
                                    requests.push(request);
                                });
                                next_request_id.with_mut(|id| {
                                    *id += 1;
                                });
                                open_requests.with_mut(|open| open.push(id));
                                selected_request.set(Some(id));
                            }
                            CreateNewOptions::Import => {
                                // TODO: Implement import functionality
                                println!("Import not yet implemented");
                            }
                        }
                    }
                },
                IconSelectTrigger {
                    aria_label: "Create New",
                    variant: ButtonVariant::Ghost,
                    icon: VscAdd,
                }
                SelectList { aria_label: "Create New",
                    SelectGroup {
                        SelectGroupLabel { "Create New" }
                        {CreateNewOptions::iter().enumerate().map(|(i, option)| {
                            rsx! {
                                SelectOption::<CreateNewOptions> {
                                    index: i,
                                    value: option,
                                    text_value: option.to_string(),
                                    "{option}"
                                }
                            }
                        })}
                    }
                }
            }
            Accordion { allow_multiple_open: true, horizontal: false,
                style: "grid-area: collection;",
                if let Some(Some(space_id)) = selected_space_id() {
                    for (i, collection) in collections.read().iter().filter(|c| c.space_id == space_id).enumerate() {
                        AccordionItem { index: i,
                            AccordionTrigger { "{collection.name}" }
                            AccordionContent {
                                div { padding_bottom: "1rem",
                                    {
                                        let collection_requests: Vec<_> = requests.read().iter().filter(|r| r.collection_id == Some(collection.id)).cloned().collect();
                                        rsx! {
                                            for request in collection_requests {
                                                {
                                                    let request_id = request.id;
                                                    rsx! {
                                                        div {
                                                            class: "library-request-item",
                                                            class: if Some(request_id) == selected_request() { "selected" },
                                                            onclick: move |_| {
                                                                open_requests.with_mut(|open| {
                                                                    if !open.contains(&request_id) {
                                                                        open.push(request_id);
                                                                    }
                                                                });
                                                                selected_request.set(Some(request_id));
                                                            },
                                                            "{request.method} {request.name}"
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
