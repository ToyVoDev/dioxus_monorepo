use crate::components::sell_prices::ComponentProps;
use dioxus::prelude::*;

#[component]
pub fn Addictiveness(props: ComponentProps) -> Element {
    rsx! {
        div {
            "Addictiveness"
        }
        div { class: "justify-self-end", "{props.working_product.addictiveness():.0}%" }
    }
}
