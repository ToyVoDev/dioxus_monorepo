use super::price_per_unit::ComponentProps;
use dioxus::prelude::*;

#[component]
pub fn TotalPrice(props: ComponentProps) -> Element {
    rsx! {
        div {
            "Total Price:"
        }
        div { justify_self: "end", "${props.working_product.price(props.mix_state.clone()):.2}" }
    }
}
