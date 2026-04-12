use crate::schedule1::domain::Sellable;
use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct ComponentProps {
    pub working_product: Sellable,
}

#[component]
pub fn SellPrices(props: ComponentProps) -> Element {
    rsx! {
        div {
            grid_column: "1 / -1",
            "Sell Price:"
        }
        div {
            "Baggie"
        }
        div { justify_self: "end", "${props.working_product.sell_price():.0}" }
        div {
            "Jar"
        }
        div { justify_self: "end", "${props.working_product.sell_price()*5.:.0}" }
        div {
            "Brick"
        }
        div { justify_self: "end", "${props.working_product.sell_price()*20.:.0}" }
    }
}
