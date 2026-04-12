use crate::schedule1::domain::{MixState, OneTimeIngredient, Sellable};
use dioxus::prelude::*;
use dioxus_primitives::checkbox::CheckboxState;
use kinetic_ui::{Checkbox, KButton, KButtonVariant};

#[derive(PartialEq, Clone, Props)]
pub struct ComponentProps {
    pub toggle_ingredient: EventHandler<OneTimeIngredient>,
    pub set_use_pot: EventHandler<bool>,
    pub working_product: Sellable,
    pub mix_state: MixState,
}

#[component]
pub fn YieldOptions(props: ComponentProps) -> Element {
    let pgr_checked = props
        .mix_state
        .ingredients
        .contains(&OneTimeIngredient::PGR);
    rsx! {
        div {
            display: "flex", justify_content: "space-between", grid_column: "1 / -1",
            div {
                display: "flex", gap: "8px",
                label {
                    display: "flex", gap: "8px", white_space: "nowrap", align_items: "center", justify_content: "space-between",
                    "Use PGR",
                    Checkbox {
                        checked: if pgr_checked { CheckboxState::Checked } else { CheckboxState::Unchecked },
                        on_checked_change: move |_: CheckboxState| {
                            props.toggle_ingredient.call(OneTimeIngredient::PGR);
                        },
                    }
                }
                KButton {
                    variant: if !props.mix_state.use_pot { KButtonVariant::Primary } else { KButtonVariant::Ghost },
                    onclick: move |_| {
                        props.set_use_pot.call(false);
                    },
                    "Tent"
                }
                KButton {
                    variant: if props.mix_state.use_pot { KButtonVariant::Primary } else { KButtonVariant::Ghost },
                    onclick: move |_| {
                        props.set_use_pot.call(true);
                    },
                    "Pot"
                }
            }
            div {
                "/{props.working_product.yield_amount(props.mix_state.clone())}"
            }
        }
    }
}
