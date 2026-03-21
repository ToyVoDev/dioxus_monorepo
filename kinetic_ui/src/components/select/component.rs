use dioxus::prelude::*;
use dioxus_primitives::select::{
    self, SelectGroupLabelProps, SelectGroupProps, SelectListProps,
    SelectOptionProps, SelectProps, SelectTriggerProps, SelectValueProps,
};

#[component]
pub fn KSelect<T: Clone + PartialEq + 'static>(props: SelectProps<T>) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        select::Select {
            class: "k-select",
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
pub fn KSelectTrigger(props: SelectTriggerProps) -> Element {
    rsx! {
        select::SelectTrigger {
            class: "k-select__trigger",
            attributes: props.attributes,
            {props.children}
            svg {
                class: "k-select__expand-icon",
                view_box: "0 0 24 24",
                xmlns: "http://www.w3.org/2000/svg",
                polyline { points: "6 9 12 15 18 9" }
            }
        }
    }
}

#[component]
pub fn KSelectValue(props: SelectValueProps) -> Element {
    rsx! {
        select::SelectValue { attributes: props.attributes }
    }
}

#[component]
pub fn KSelectList(props: SelectListProps) -> Element {
    rsx! {
        select::SelectList {
            class: "k-select__list",
            id: props.id,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn KSelectGroup(props: SelectGroupProps) -> Element {
    rsx! {
        select::SelectGroup {
            class: "k-select__group",
            disabled: props.disabled,
            id: props.id,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn KSelectGroupLabel(props: SelectGroupLabelProps) -> Element {
    rsx! {
        select::SelectGroupLabel {
            class: "k-select__group-label",
            id: props.id,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn KSelectOption<T: Clone + PartialEq + 'static>(props: SelectOptionProps<T>) -> Element {
    rsx! {
        select::SelectOption::<T> {
            class: "k-select__option",
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
pub fn KSelectItemIndicator(children: Element) -> Element {
    rsx! {
        select::SelectItemIndicator {
            if children == VNode::empty() {
                svg {
                    class: "k-select__check-icon",
                    view_box: "0 0 24 24",
                    xmlns: "http://www.w3.org/2000/svg",
                    path { d: "M5 13l4 4L19 7" }
                }
            } else {
                {children}
            }
        }
    }
}
