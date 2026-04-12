use crate::schedule1::domain::Sellable;
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::go_icons::{GoBookmark, GoBookmarkSlash};
use kinetic_ui::{IconButton, KInput};
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
            grid_column: "1 / -1", display: "flex", gap: "8px",
            if props.saved_recipes.contains_key(&props.working_product.key()) {
                IconButton {
                    onclick: move |_| {
                        props.toggle_save.call(working_product_clone.clone());
                    },
                    Icon { icon: GoBookmarkSlash }
                }
            } else {
                IconButton {
                    disabled: props.working_product.ingredients.is_empty(),
                    onclick: move |_| {
                        props.toggle_save.call(working_product_clone.clone());
                    },
                    Icon { icon: GoBookmark }
                }
            },
            div { flex_grow: "1",
                KInput {
                    value: props.working_product.name.clone(),
                    oninput: move |event: FormEvent| props.set_working_product.call(props.working_product.with_name(event.value())),
                }
            }
        }
    }
}
