use crate::components::Button;
use crate::sellable::{MixState, Quality};
use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct ComponentProps {
    pub set_pseudo_quality: EventHandler<Quality>,
    pub mix_state: MixState,
}

#[component]
pub fn PseudoOptions(props: ComponentProps) -> Element {
    rsx! {
        div {
            class: "flex justify-between col-span-full",
            div {
                class: "flex gap-2",
                Button {
                    onclick: move |_| {
                        props.set_pseudo_quality.call(Quality::Low);
                    },
                    active: props.mix_state.pseudo_quality == Quality::Low,
                    "Low-Quality Pseudo"
                }
                Button {
                    onclick: move |_| {
                        props.set_pseudo_quality.call(Quality::Medium);
                    },
                    active: props.mix_state.pseudo_quality == Quality::Medium,
                    "Pseudo"
                }
                Button {
                    onclick: move |_| {
                        props.set_pseudo_quality.call(Quality::High);
                    },
                    active: props.mix_state.pseudo_quality == Quality::High,
                    "High-Quality Pseudo"
                }
            }
            div {
                "/10"
            }
        }
    }
}
