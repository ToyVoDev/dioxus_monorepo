use crate::sellable::Sellable;
use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct ComponentProps {
    pub working_product: Sellable,
}

#[component]
pub fn SellPrices(props: ComponentProps) -> Element {
    rsx! {
        div {
            class: "col-span-full",
            "Sell Price:"
        }
        div {
            "Baggie"
        }
        div { class: "justify-self-end", "${props.working_product.sell_price():.0}" }
        div {
            "Jar"
        }
        div { class: "justify-self-end", "${props.working_product.sell_price()*5.:.0}" }
        div {
            "Brick"
        }
        div { class: "justify-self-end", "${props.working_product.sell_price()*20.:.0}" }
    }
}
