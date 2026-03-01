use crate::components::IconButton;
use crate::sellable::Sellable;
use dioxus::prelude::*;
use dioxus_free_icons::icons::go_icons::{GoBookmark, GoBookmarkSlash};
use std::collections::HashMap;

#[derive(PartialEq, Clone, Props)]
pub struct ComponentProps {
    pub set_working_product: EventHandler<Sellable>,
    pub toggle_save: EventHandler<Sellable>,
    pub working_product: Sellable,
    pub saved_recipes: HashMap<String, Sellable>,
}

#[component]
pub fn SaveProduct(props: ComponentProps) -> Element {
    let working_product_clone = props.working_product.clone();
    rsx! {
        div {
            class: "col-span-full flex gap-2",
            if props.saved_recipes.contains_key(&props.working_product.key()) {
                IconButton {
                    icon: GoBookmarkSlash,
                    onclick: move |_| {
                        props.toggle_save.call(working_product_clone.clone());
                    }
                }
            } else {
                IconButton {
                    icon: GoBookmark,
                    disabled: Some(props.working_product.ingredients.is_empty()),
                    onclick: move |_| {
                        props.toggle_save.call(working_product_clone.clone());
                    }
                }
            },
            input {
                class: "grow",
                value: "{props.working_product.name}",
                oninput: move |event| props.set_working_product.call(props.working_product.with_name(event.value())),
            }
        }
    }
}
