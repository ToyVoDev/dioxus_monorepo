use crate::components::button::ButtonVariant;
use dioxus::prelude::*;
use dioxus_free_icons::{Icon, IconShape};
use dioxus_primitives::select;

#[component]
pub fn IconSelectTrigger<T: IconShape + Clone + PartialEq + 'static>(
    #[props(default)] variant: ButtonVariant,
    icon: T,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
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
        {children}
        select::SelectTrigger {
            class: "button",
            style: "flex-basis: 0",
            "data-style": variant.class(),
            attributes: combined_attributes,
            Icon {
                width: 12,
                height: 12,
                fill: "inherit",
                icon: icon,
            }
        }
    }
}
