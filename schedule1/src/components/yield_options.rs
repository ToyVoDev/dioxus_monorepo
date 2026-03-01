use crate::components::Button;
use crate::sellable::{MixState, OneTimeIngredient, Sellable};
use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct ComponentProps {
    pub toggle_ingredient: EventHandler<OneTimeIngredient>,
    pub set_use_pot: EventHandler<bool>,
    pub working_product: Sellable,
    pub mix_state: MixState,
}

#[component]
pub fn YieldOptions(props: ComponentProps) -> Element {
    rsx! {
        div {
            class: "flex justify-between col-span-full",
            div {
                class: "flex gap-2",
                label {
                    class: "flex gap-2 whitespace-nowrap items-center justify-between",
                    "Use PGR",
                    input {
                        r#type: "checkbox",
                        checked: "{props.mix_state.ingredients.contains(&OneTimeIngredient::PGR)}",
                        onchange: move |_| {
                            props.toggle_ingredient.call(OneTimeIngredient::PGR);
                        }
                    }
                }
                Button {
                    onclick: move |_| {
                        props.set_use_pot.call(false);
                    },
                    active: !props.mix_state.use_pot,
                    "Tent"
                }
                Button {
                    onclick: move |_| {
                        props.set_use_pot.call(true);
                    },
                    active: props.mix_state.use_pot,
                    "Pot"
                }
            }
            div {
                "/{props.working_product.yield_amount(props.mix_state.clone())}"
            }
        }
    }
}
