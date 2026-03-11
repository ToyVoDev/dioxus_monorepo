use crate::components::Button;
use crate::sellable::{Effect, Product};
use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct ComponentProps {
    pub set_working_product: EventHandler<Product>,
}

#[component]
pub fn BaseProducts(props: ComponentProps) -> Element {
    use Product::*;
    rsx! {
        div { class: "col-span-full", "Base Product" }
        Button { onclick: move |_| props.set_working_product.call(Marijuana(Effect::Calming)), "OG Kush" }
        Button { onclick: move |_| props.set_working_product.call(Marijuana(Effect::Energizing)), "Sour Diesel" }
        Button { onclick: move |_| props.set_working_product.call(Marijuana(Effect::Refreshing)), "Green Crack" }
        Button { onclick: move |_| props.set_working_product.call(Marijuana(Effect::Sedating)), "Granddaddy Purple" }
        Button { onclick: move |_| props.set_working_product.call(Meth), "Meth" }
        Button { onclick: move |_| props.set_working_product.call(Cocaine), "Cocaine" }
    }
}
