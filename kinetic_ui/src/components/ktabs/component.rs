use dioxus::prelude::*;
use dioxus_primitives::tabs::{self, TabContentProps, TabListProps, TabTriggerProps, TabsProps};

/// Standalone component that only loads the tabs CSS stylesheet.
/// Use when consuming `dioxus_primitives::tabs` directly with k-tabs classes.
#[component]
pub fn KTabsStylesheet() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
    }
}

#[component]
pub fn KTabs(props: TabsProps) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        tabs::Tabs {
            class: "k-tabs",
            value: props.value,
            default_value: props.default_value,
            on_value_change: props.on_value_change,
            disabled: props.disabled,
            horizontal: props.horizontal,
            roving_loop: props.roving_loop,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn KTabList(props: TabListProps) -> Element {
    rsx! {
        tabs::TabList { class: "k-tabs__list", attributes: props.attributes, {props.children} }
    }
}

#[component]
pub fn KTabTrigger(#[props(default)] badge: Option<u32>, props: TabTriggerProps) -> Element {
    rsx! {
        tabs::TabTrigger {
            class: "k-tabs__trigger",
            value: props.value,
            index: props.index,
            disabled: props.disabled,
            id: props.id,
            attributes: props.attributes,
            {props.children}
            if let Some(count) = badge {
                span { class: "k-badge", "data-variant": "muted", "{count}" }
            }
        }
    }
}

#[component]
pub fn KTabContent(props: TabContentProps) -> Element {
    rsx! {
        tabs::TabContent {
            class: "k-tabs__content",
            value: props.value,
            index: props.index,
            id: props.id,
            attributes: props.attributes,
            {props.children}
        }
    }
}
