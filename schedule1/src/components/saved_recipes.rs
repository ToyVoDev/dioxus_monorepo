use crate::components::Button;
use crate::sellable::Sellable;
use dioxus::prelude::*;
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
            div { class: "col-span-full", "Saved Recipes" }
            {props.saved_recipes.iter().map(|(key, recipe)| {
                let recipe_clone = recipe.clone();
                rsx! {
                    Button {
                        key: "{key}",
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
