use crate::components::button::ButtonVariant;
use dioxus::prelude::*;
use dioxus_free_icons::{Icon, IconShape};
use dioxus_primitives::select::{
    self, SelectGroupLabelProps, SelectGroupProps, SelectListProps, SelectOptionProps, SelectProps,
    SelectValueProps,
};

#[component]
pub fn Select<T: Clone + PartialEq + 'static>(props: SelectProps<T>) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        select::Select {
            class: "select",
            value: props.value,
            default_value: props.default_value,
            on_value_change: props.on_value_change,
            disabled: props.disabled,
            name: props.name,
            placeholder: props.placeholder,
            roving_loop: props.roving_loop,
            typeahead_timeout: props.typeahead_timeout,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn ButtonSelectTrigger<T: IconShape + Clone + PartialEq + 'static>(
    #[props(default)] variant: ButtonVariant,
    icon: Option<T>,
    /// Additional attributes for the trigger button
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    /// The children to render inside the trigger
    children: Element,
) -> Element {
    let mut combined_attributes = attributes;

    // Add event handlers to prevent default behavior to prevent the select
    // from toggling on mouse events even though we only toggle on click.
    combined_attributes.push(onmousedown(move |event: Event<MouseData>| {
        event.prevent_default();
        event.stop_propagation();
    }));
    combined_attributes.push(onmouseup(move |event: Event<MouseData>| {
        event.prevent_default();
        event.stop_propagation();
    }));

    rsx! {
        select::SelectTrigger {
            class: "button",
            "data-style": variant.class(),
            attributes: combined_attributes,
            {children}
            if let Some(icon) = icon {
                Icon {
                    width: 12,
                    height: 12,
                    fill: "inherit",
                    icon: icon,
                }
            } else {
                svg {
                    class: "select-expand-icon",
                    view_box: "0 0 24 24",
                    xmlns: "http://www.w3.org/2000/svg",
                    polyline { points: "6 9 12 15 18 9" }
                }
            }
        }
    }
}

#[component]
pub fn SelectTrigger<T: IconShape + Clone + PartialEq + 'static>(
    icon: Option<T>,
    /// Additional attributes for the trigger button
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    /// The children to render inside the trigger
    children: Element,
) -> Element {
    rsx! {
        select::SelectTrigger { class: "select-trigger", attributes: attributes,
            {children}
            if let Some(icon) = icon {
                Icon {
                    width: 12,
                    height: 12,
                    fill: "inherit",
                    icon: icon,
                }
            } else {
                svg {
                    class: "select-expand-icon",
                    view_box: "0 0 24 24",
                    xmlns: "http://www.w3.org/2000/svg",
                    polyline { points: "6 9 12 15 18 9" }
                }
            }
        }
    }
}

#[component]
pub fn SelectValue(props: SelectValueProps) -> Element {
    rsx! {
        select::SelectValue { attributes: props.attributes }
    }
}

#[component]
pub fn SelectList(props: SelectListProps) -> Element {
    rsx! {
        select::SelectList {
            class: "select-list",
            id: props.id,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn SelectGroup(props: SelectGroupProps) -> Element {
    rsx! {
        select::SelectGroup {
            class: "select-group",
            disabled: props.disabled,
            id: props.id,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn SelectGroupLabel(props: SelectGroupLabelProps) -> Element {
    rsx! {
        select::SelectGroupLabel {
            class: "select-group-label",
            id: props.id,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn SelectOption<T: Clone + PartialEq + 'static>(props: SelectOptionProps<T>) -> Element {
    rsx! {
        select::SelectOption::<T> {
            class: "select-option",
            value: props.value,
            text_value: props.text_value,
            disabled: props.disabled,
            id: props.id,
            index: props.index,
            aria_label: props.aria_label,
            aria_roledescription: props.aria_roledescription,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn SelectItemIndicator() -> Element {
    rsx! {
        select::SelectItemIndicator {
            svg {
                class: "select-check-icon",
                view_box: "0 0 24 24",
                xmlns: "http://www.w3.org/2000/svg",
                path { d: "M5 13l4 4L19 7" }
            }
        }
    }
}
