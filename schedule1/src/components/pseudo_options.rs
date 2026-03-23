use crate::sellable::{MixState, Quality};
use dioxus::prelude::*;
use kinetic_ui::{KButton, KButtonVariant};

#[derive(PartialEq, Clone, Props)]
pub struct ComponentProps {
    pub set_pseudo_quality: EventHandler<Quality>,
    pub mix_state: MixState,
}

#[component]
pub fn PseudoOptions(props: ComponentProps) -> Element {
    rsx! {
        div {
            display: "flex", justify_content: "space-between", grid_column: "1 / -1",
            div {
                display: "flex", gap: "8px",
                KButton {
                    variant: if props.mix_state.pseudo_quality == Quality::Low { KButtonVariant::Primary } else { KButtonVariant::Ghost },
                    onclick: move |_| {
                        props.set_pseudo_quality.call(Quality::Low);
                    },
                    "Low-Quality Pseudo"
                }
                KButton {
                    variant: if props.mix_state.pseudo_quality == Quality::Medium { KButtonVariant::Primary } else { KButtonVariant::Ghost },
                    onclick: move |_| {
                        props.set_pseudo_quality.call(Quality::Medium);
                    },
                    "Pseudo"
                }
                KButton {
                    variant: if props.mix_state.pseudo_quality == Quality::High { KButtonVariant::Primary } else { KButtonVariant::Ghost },
                    onclick: move |_| {
                        props.set_pseudo_quality.call(Quality::High);
                    },
                    "High-Quality Pseudo"
                }
            }
            div {
                "/10"
            }
        }
    }
}
