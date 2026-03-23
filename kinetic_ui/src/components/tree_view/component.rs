use dioxus::prelude::*;

#[component]
pub fn TreeBranch(
    label: Element,
    #[props(default)] initially_expanded: Option<bool>,
    children: Element,
) -> Element {
    let mut expanded = use_signal(|| initially_expanded.unwrap_or(false));
    let expanded_str = expanded().to_string();

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }

        div {
            class: "k-tree__branch",
            button {
                class: "k-tree__branch-trigger",
                "data-expanded": "{expanded_str}",
                onclick: move |_| expanded.toggle(),
                svg {
                    class: "k-tree__chevron",
                    view_box: "0 0 24 24",
                    xmlns: "http://www.w3.org/2000/svg",
                    polyline { points: "9 6 15 12 9 18" }
                }
                {label}
            }
            if expanded() {
                div {
                    class: "k-tree__branch-content",
                    {children}
                }
            }
        }
    }
}

#[component]
pub fn TreeLeaf(
    #[props(default)] selected: Option<bool>,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let selected_str = selected.unwrap_or(false).to_string();

    rsx! {
        button {
            class: "k-tree__leaf",
            "data-selected": "{selected_str}",
            onclick: move |e| {
                if let Some(f) = &onclick {
                    f.call(e);
                }
            },
            {children}
        }
    }
}
