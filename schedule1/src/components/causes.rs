use crate::components::sell_prices::ComponentProps;
use dioxus::prelude::*;

#[component]
pub fn Causes(props: ComponentProps) -> Element {
    rsx! {
        div { "Causes:" }
        div { justify_self: "end", "Multiplier:" }
        for effect in props.working_product.effects.iter() {
            div { "{effect:?}" }
            div { justify_self: "end", "x{effect.multiplier():.2}" }
        }
    }
}
