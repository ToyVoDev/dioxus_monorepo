use crate::schedule1::domain::{Effect, Product};
use dioxus::prelude::*;
use kinetic_ui::{KButton, KButtonVariant};

#[derive(PartialEq, Clone, Props)]
pub struct ComponentProps {
    pub set_working_product: EventHandler<Product>,
}

#[component]
pub fn BaseProducts(props: ComponentProps) -> Element {
    use Product::*;
    rsx! {
        div { grid_column: "1 / -1", "Base Product" }
        KButton { variant: KButtonVariant::Ghost, onclick: move |_| props.set_working_product.call(Marijuana(Effect::Calming)), "OG Kush" }
        KButton { variant: KButtonVariant::Ghost, onclick: move |_| props.set_working_product.call(Marijuana(Effect::Energizing)), "Sour Diesel" }
        KButton { variant: KButtonVariant::Ghost, onclick: move |_| props.set_working_product.call(Marijuana(Effect::Refreshing)), "Green Crack" }
        KButton { variant: KButtonVariant::Ghost, onclick: move |_| props.set_working_product.call(Marijuana(Effect::Sedating)), "Granddaddy Purple" }
        KButton { variant: KButtonVariant::Ghost, onclick: move |_| props.set_working_product.call(Meth), "Meth" }
        KButton { variant: KButtonVariant::Ghost, onclick: move |_| props.set_working_product.call(Cocaine), "Cocaine" }
    }
}
