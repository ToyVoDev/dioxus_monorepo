use crate::components::Button;
use crate::sellable::{MixState, OneTimeIngredient, Quality};
use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct ComponentProps {
    pub set_soil_quality: EventHandler<Quality>,
    pub toggle_ingredient: EventHandler<OneTimeIngredient>,
    pub mix_state: MixState,
}

#[component]
pub fn SoilOptions(props: ComponentProps) -> Element {
    rsx! {
        div { class: "flex flex-wrap col-span-full justify-between gap-2",
            div {
                class: "flex flex-col justify-center",
                label {
                    class: "flex gap-2 whitespace-nowrap items-center justify-between",
                    "Use Fertilizer",
                    input {
                        r#type: "checkbox",
                        checked: "{props.mix_state.ingredients.contains(&OneTimeIngredient::Fertilizer)}",
                        onchange: move |_| {
                            props.toggle_ingredient.call(OneTimeIngredient::Fertilizer);
                        }
                    }
                }
                label {
                    class: "flex gap-2 whitespace-nowrap items-center justify-between",
                    "Use Speed Grow",
                    input {
                        r#type: "checkbox",
                        checked: "{props.mix_state.ingredients.contains(&OneTimeIngredient::SpeedGrow)}",
                        onchange: move |_| {
                            props.toggle_ingredient.call(OneTimeIngredient::SpeedGrow);
                        }
                    }
                }
            }
            div {
                class: "flex gap-2",
                Button {
                    onclick: move |_| {
                        props.set_soil_quality.call(Quality::Low);
                    },
                    active: props.mix_state.soil_quality == Quality::Low,
                    "Soil"
                }
                Button {
                    onclick: move |_| {
                        props.set_soil_quality.call(Quality::Medium);
                    },
                    active: props.mix_state.soil_quality == Quality::Medium,
                    "Long-Life Soil"
                }
                Button {
                    onclick: move |_| {
                        props.set_soil_quality.call(Quality::High);
                    },
                    active: props.mix_state.soil_quality == Quality::High,
                    "Extra Long-Life Soil"
                }
            }
        }
    }
}
