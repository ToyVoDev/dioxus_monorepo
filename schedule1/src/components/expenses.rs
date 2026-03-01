use crate::components::price_per_unit::ComponentProps;
use crate::sellable::{OneTimeIngredient, Product, Quality};
use dioxus::prelude::*;

#[component]
pub fn Expenses(props: ComponentProps) -> Element {
    rsx! {
        div { "Based on:" }
        div { class: "justify-self-end", "Price:" }
        div {"{props.working_product.base:?}"}
        div { class: "justify-self-end", "${props.working_product.base.price(props.mix_state.clone()):.2}" }
        match (props.working_product.base, props.mix_state.soil_quality) {
            (Product::Meth, _) => rsx! {},
            (_, Quality::Low) => rsx! {
                div { "Soil" }
                div { class: "justify-self-end", "${10.:.2}" }
            },
            (_, Quality::Medium) => rsx! {
                div { "Long-Life Soil" }
                div { class: "justify-self-end", "${30.:.2}" }
            },
            (_, Quality::High) => rsx! {
                div { "Extra Long-Life Soil" }
                div { class: "justify-self-end", "${60.:.2}" }
            },
        }
        match (props.working_product.base, props.mix_state.ingredients.contains(&OneTimeIngredient::PGR)) {
            (Product::Meth, _) => rsx! {},
            (_, true) => rsx! {
                div { "PGR" }
                div { class: "justify-self-end", "${30.:.2}" }
            },
            _ => rsx! {},
        }
        match (props.working_product.base, props.mix_state.ingredients.contains(&OneTimeIngredient::Fertilizer)) {
            (Product::Meth, _) => rsx! {},
            (_, true) => rsx! {
                div { "Fertilizer" }
                div { class: "justify-self-end", "${30.:.2}" }
            },
            _ => rsx! {},
        }
        match (props.working_product.base, props.mix_state.ingredients.contains(&OneTimeIngredient::SpeedGrow)) {
            (Product::Meth, _) => rsx! {},
            (_, true) => rsx! {
                div { "Speed Grow" }
                div { class: "justify-self-end", "${30.:.2}" }
            },
            _ => rsx! {},
        }
    }
}
