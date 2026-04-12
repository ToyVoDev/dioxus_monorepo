use crate::schedule1::{Effect, MixState, Product, Sellable, *};
use dioxus::prelude::*;
use std::collections::HashMap;

#[component]
pub fn Schedule1() -> Element {
    let mut working_product =
        use_signal(|| Sellable::from_product(Product::Marijuana(Effect::Calming)));
    let mut previous_working_product =
        use_signal(|| Sellable::from_product(Product::Marijuana(Effect::Calming)));
    let mut saved_recipes = use_signal(HashMap::<String, Sellable>::new);
    let mut mix_state = use_signal(MixState::default);
    let mut added_effect = use_signal(|| None);

    rsx! {
        div {
            display: "flex",
            justify_content: "flex_end",
            padding: "8px",
        }
        div {
            display: "grid",
            gap: "1rem",
            grid_template_columns: "minmax(365px, 1fr) minmax(240px, 1fr) minmax(150px, 1fr)",
            div {
                display: "grid",
                grid_template_columns: "repeat(3, 1fr)",
                gap: "1rem",
                align_content: "start",
                BaseProducts { set_working_product: move |product| {
                    working_product.set(Sellable::from_product(product));
                    added_effect.set(None);
                }}
                AddIngredients { add_ingredient: move |ingredient| {
                    previous_working_product.set(working_product());
                    working_product.set(working_product().add_ingredient(ingredient));
                    added_effect.set(Some(ingredient.effect()));
                }}
                SavedRecipes {
                    set_working_product: move |recipe| {
                        working_product.set(recipe);
                        added_effect.set(None);
                    },
                    working_product: working_product(),
                    saved_recipes: saved_recipes(),
                }
            }
            div {
                display: "grid",
                grid_template_columns: "repeat(2, 1fr)",
                gap: "1rem",
                align_content: "start",
                SaveProduct {
                    working_product: working_product(),
                    saved_recipes: saved_recipes(),
                    set_working_product: move |recipe| working_product.set(recipe),
                    toggle_save: move |recipe: Sellable| {
                        let key = recipe.key();
                        if saved_recipes.read().contains_key(&key) {
                            saved_recipes.write().remove(&key);
                        } else {
                            saved_recipes.write().insert(key, recipe);
                        }
                    },
                }
                Expenses {
                    mix_state: mix_state(),
                    working_product: working_product(),
                }
                div { style: "grid-column: 1 / -1; border-bottom: 1px solid var(--primary-color-6);" }
                match working_product.read().base {
                    Product::Meth => rsx! {
                        PseudoOptions {
                            mix_state: mix_state(),
                            set_pseudo_quality: move |quality| mix_state.write().pseudo_quality = quality,
                        }
                    },
                    _ => rsx! {
                        YieldOptions {
                            mix_state: mix_state(),
                            working_product: working_product(),
                            toggle_ingredient: move |ingredient| {
                                if mix_state.read().ingredients.contains(&ingredient) {
                                    mix_state.write().ingredients.remove(&ingredient);
                                } else {
                                    mix_state.write().ingredients.insert(ingredient);
                                }
                            },
                            set_use_pot: move |use_pot| mix_state.write().use_pot = use_pot,
                        }
                        SoilOptions {
                            mix_state: mix_state(),
                            set_soil_quality: move |quality| mix_state.write().soil_quality = quality,
                            toggle_ingredient: move |ingredient| {
                                if mix_state.read().ingredients.contains(&ingredient) {
                                    mix_state.write().ingredients.remove(&ingredient);
                                } else {
                                    mix_state.write().ingredients.insert(ingredient);
                                }
                            },
                        }
                    }
                }
                div { style: "grid-column: 1 / -1; border-bottom: 1px solid var(--primary-color-6);" }
                PricePerUnit {
                    working_product: working_product(),
                    mix_state: mix_state(),
                }
                div { style: "grid-column: 1 / -1; border-bottom: 1px solid var(--primary-color-6);" }
                TotalPrice { working_product: working_product(), mix_state: mix_state() }
            }
            div {
                display: "grid",
                grid_template_columns: "repeat(2, 1fr)",
                gap: "1rem",
                align_content: "start",
                div { style: "grid-column: 1 / -1; border: 1px solid var(--primary-color-6); padding: 0.5rem;", "Warning: Column in progress, has inaccuracies"}
                Addictiveness { working_product: working_product() }
                if !working_product.read().effects.is_empty() {
                    div { style: "grid-column: 1 / -1; border-bottom: 1px solid var(--primary-color-6);" }
                    Causes { working_product: working_product() }
                }
                MixMap {
                    added_effect: added_effect,
                    previous_working_product: previous_working_product,
                    working_product: working_product,
                }
                div { style: "grid-column: 1 / -1; border-bottom: 1px solid var(--primary-color-6);" }
                SellPrices { working_product: working_product() }
            }
        }
    }
}
