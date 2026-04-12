use crate::schedule1::domain::Sellable;
use dioxus::prelude::*;
use kinetic_ui::{KButton, KButtonVariant};
use std::collections::HashMap;

#[derive(Clone, PartialEq, Props)]
pub struct ComponentProps {
    set_working_product: EventHandler<Sellable>,
    working_product: Sellable,
    saved_recipes: HashMap<String, Sellable>,
}

#[component]
pub fn SavedRecipes(props: ComponentProps) -> Element {
    rsx! {
        if !props.saved_recipes.is_empty() {
            div { grid_column: "1 / -1", "Saved Recipes" }
            {props.saved_recipes.iter().map(|(key, recipe)| {
                let recipe_clone = recipe.clone();
                rsx! {
                    KButton {
                        key: "{key}",
                        variant: KButtonVariant::Ghost,
                        onclick: move |_| {
                            props.set_working_product.call(recipe_clone.clone());
                        },
                        "{recipe.name}"
                    }
                }
            })}
        }
    }
}
