use crate::schedule1::domain::{MixState, OneTimeIngredient, Quality};
use dioxus::prelude::*;
use dioxus_primitives::checkbox::CheckboxState;
use kinetic_ui::{Checkbox, KButton, KButtonVariant};

#[derive(PartialEq, Clone, Props)]
pub struct ComponentProps {
    pub set_soil_quality: EventHandler<Quality>,
    pub toggle_ingredient: EventHandler<OneTimeIngredient>,
    pub mix_state: MixState,
}

#[component]
pub fn SoilOptions(props: ComponentProps) -> Element {
    let fertilizer_checked = props
        .mix_state
        .ingredients
        .contains(&OneTimeIngredient::Fertilizer);
    let speed_grow_checked = props
        .mix_state
        .ingredients
        .contains(&OneTimeIngredient::SpeedGrow);
    rsx! {
        div { display: "flex", flex_wrap: "wrap", grid_column: "1 / -1", justify_content: "space-between", gap: "8px",
            div {
                display: "flex", flex_direction: "column", justify_content: "center",
                label {
                    display: "flex", gap: "8px", white_space: "nowrap", align_items: "center", justify_content: "space-between",
                    "Use Fertilizer",
                    Checkbox {
                        checked: if fertilizer_checked { CheckboxState::Checked } else { CheckboxState::Unchecked },
                        on_checked_change: move |_: CheckboxState| {
                            props.toggle_ingredient.call(OneTimeIngredient::Fertilizer);
                        },
                    }
                }
                label {
                    display: "flex", gap: "8px", white_space: "nowrap", align_items: "center", justify_content: "space-between",
                    "Use Speed Grow",
                    Checkbox {
                        checked: if speed_grow_checked { CheckboxState::Checked } else { CheckboxState::Unchecked },
                        on_checked_change: move |_: CheckboxState| {
                            props.toggle_ingredient.call(OneTimeIngredient::SpeedGrow);
                        },
                    }
                }
            }
            div {
                display: "flex", gap: "8px",
                KButton {
                    variant: if props.mix_state.soil_quality == Quality::Low { KButtonVariant::Primary } else { KButtonVariant::Ghost },
                    onclick: move |_| {
                        props.set_soil_quality.call(Quality::Low);
                    },
                    "Soil"
                }
                KButton {
                    variant: if props.mix_state.soil_quality == Quality::Medium { KButtonVariant::Primary } else { KButtonVariant::Ghost },
                    onclick: move |_| {
                        props.set_soil_quality.call(Quality::Medium);
                    },
                    "Long-Life Soil"
                }
                KButton {
                    variant: if props.mix_state.soil_quality == Quality::High { KButtonVariant::Primary } else { KButtonVariant::Ghost },
                    onclick: move |_| {
                        props.set_soil_quality.call(Quality::High);
                    },
                    "Extra Long-Life Soil"
                }
            }
        }
    }
}
