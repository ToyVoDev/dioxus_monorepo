use crate::sellable::{MixState, OneTimeIngredient, Product, Quality, Sellable};
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct ComponentProps {
    pub working_product: Sellable,
    pub mix_state: MixState,
}

#[component]
pub fn PricePerUnit(props: ComponentProps) -> Element {
    rsx! {
        div { class: "col-span-full", "Price per unit:" }
        div {"{props.working_product.base:?}"}
        div { class: "justify-self-end", "${props.working_product.unit_price(props.mix_state.clone()):.2}" }
        match props.working_product.base {
            Product::Meth => rsx! {},
            _ => rsx! {
                match props.mix_state.soil_quality {
                    Quality::Low => rsx! {
                        div { "Soil" }
                        div { class: "justify-self-end", "${10./props.working_product.yield_amount(props.mix_state.clone()):.2}" }
                    },
                    Quality::Medium => rsx! {
                        div { "Long-Life Soil" }
                        div { class: "justify-self-end", "${30./(props.working_product.yield_amount(props.mix_state.clone())*2.):.2}" }
                    },
                    Quality::High => rsx! {
                        div { "Extra Long-Life Soil" }
                        div { class: "justify-self-end", "${60./(props.working_product.yield_amount(props.mix_state.clone())*3.):.2}" }
                    },
                }
                if props.mix_state.ingredients.contains(&OneTimeIngredient::PGR) {
                    div { "PGR" }
                    div { class: "justify-self-end", "${30./props.working_product.yield_amount(props.mix_state.clone()):.2}" }
                }
                if props.mix_state.ingredients.contains(&OneTimeIngredient::Fertilizer) {
                    div { "Fertilizer" }
                    div { class: "justify-self-end", "${30./props.working_product.yield_amount(props.mix_state.clone()):.2}" }
                }
                if props.mix_state.ingredients.contains(&OneTimeIngredient::SpeedGrow) {
                    div { "Speed Grow" }
                    div { class: "justify-self-end", "${30./props.working_product.yield_amount(props.mix_state.clone()):.2}" }
                }
            }
        }
        for ingredient in props.working_product.ingredients.iter() {
            div {"{ingredient:?}"}
            div { class: "justify-self-end", "${ingredient.price():.2}" }
        }
    }
}
