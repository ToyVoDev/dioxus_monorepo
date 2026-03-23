use crate::state::{AppState, SideNavItem};
use dioxus::prelude::*;
use kinetic_ui::{KBadge, KBadgeVariant, TreeBranch, TreeLeaf};

const fn method_badge_variant(method: &str) -> KBadgeVariant {
    match method.as_bytes() {
        b"GET" => KBadgeVariant::Secondary,
        b"POST" => KBadgeVariant::Primary,
        b"PUT" | b"PATCH" | b"DELETE" => KBadgeVariant::Tertiary,
        _ => KBadgeVariant::Muted,
    }
}

const fn header_title(nav: SideNavItem) -> &'static str {
    match nav {
        SideNavItem::Collections => "COLLECTIONS",
        SideNavItem::History => "HISTORY",
        SideNavItem::Apis => "APIS",
        SideNavItem::MockServers => "MOCK SERVERS",
    }
}

#[component]
pub fn Explorer() -> Element {
    let state = use_context::<AppState>();
    let active_nav = state.active_sidebar_nav;
    let collections = state.collections;
    let requests = state.requests;
    let mut selected_request = state.selected_request;

    let title = header_title(active_nav());

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }

        div { class: "explorer",
            div { class: "explorer__header",
                span { class: "explorer__title", "{title}" }
            }

            if active_nav() == SideNavItem::Collections {
                div { class: "explorer__content",
                    // Collections with their requests
                    for collection in collections() {
                        TreeBranch {
                            initially_expanded: true,
                            label: rsx! { "{collection.name}" },
                            {
                                rsx! {
                                    for request in requests()
                                        .into_iter()
                                        .filter(|r| r.collection_id == Some(collection.id))
                                    {
                                        {
                                            let req_id = request.id;
                                            let variant = method_badge_variant(&request.method);
                                            let method = request.method.clone();
                                            let name = request.name.clone();
                                            let url = request.url;
                                            rsx! {
                                                TreeLeaf {
                                                    selected: selected_request() == Some(req_id),
                                                    onclick: move |_| selected_request.set(Some(req_id)),
                                                    div { class: "explorer__request-item",
                                                        div { style: "display: flex; align-items: center; gap: 6px;",
                                                            KBadge { variant: variant, "{method}" }
                                                            span { class: "explorer__request-name", "{name}" }
                                                        }
                                                        span { class: "explorer__request-url", "{url}" }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Draft/unassigned requests (no collection)
                    {
                        let draft_requests: Vec<_> = requests()
                            .into_iter()
                            .filter(|r| r.collection_id.is_none())
                            .collect();
                        if draft_requests.is_empty() {
                            rsx! {}
                        } else {
                            rsx! {
                                TreeBranch {
                                    initially_expanded: true,
                                    label: rsx! { "Drafts" },
                                    for request in draft_requests {
                                        {
                                            let req_id = request.id;
                                            let variant = method_badge_variant(&request.method);
                                            let method = request.method.clone();
                                            let name = request.name.clone();
                                            let url = request.url;
                                            rsx! {
                                                TreeLeaf {
                                                    selected: selected_request() == Some(req_id),
                                                    onclick: move |_| selected_request.set(Some(req_id)),
                                                    div { class: "explorer__request-item",
                                                        div { style: "display: flex; align-items: center; gap: 6px;",
                                                            KBadge { variant: variant, "{method}" }
                                                            span { class: "explorer__request-name", "{name}" }
                                                        }
                                                        span { class: "explorer__request-url", "{url}" }
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
            } else {
                div { class: "explorer__placeholder",
                    "Coming soon"
                }
            }
        }
    }
}
