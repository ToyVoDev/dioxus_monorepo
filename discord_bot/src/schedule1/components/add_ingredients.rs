use crate::schedule1::domain::Ingredient;
use dioxus::prelude::*;
use kinetic_ui::{KButton, KButtonVariant};

#[derive(PartialEq, Clone, Props)]
pub struct ComponentProps {
    pub add_ingredient: EventHandler<Ingredient>,
}

#[component]
pub fn AddIngredients(props: ComponentProps) -> Element {
    use Ingredient::*;
    rsx! {
        div { grid_column: "1 / -1", "Add Ingredient" }
        KButton { variant: KButtonVariant::Ghost, onclick: move |_| props.add_ingredient.call(Cuke), "Cuke" }
        KButton { variant: KButtonVariant::Ghost, onclick: move |_| props.add_ingredient.call(Banana), "Banana" }
        KButton { variant: KButtonVariant::Ghost, onclick: move |_| props.add_ingredient.call(Paracetamol), "Paracetamol" }
        KButton { variant: KButtonVariant::Ghost, onclick: move |_| props.add_ingredient.call(Donut), "Donut" }
        KButton { variant: KButtonVariant::Ghost, onclick: move |_| props.add_ingredient.call(Viagra), "Viagra" }
        KButton { variant: KButtonVariant::Ghost, onclick: move |_| props.add_ingredient.call(MouthWash), "Mouth Wash" }
        KButton { variant: KButtonVariant::Ghost, onclick: move |_| props.add_ingredient.call(FluMedicine), "Flu Medicine" }
        KButton { variant: KButtonVariant::Ghost, onclick: move |_| props.add_ingredient.call(Gasoline), "Gasoline" }
        KButton { variant: KButtonVariant::Ghost, onclick: move |_| props.add_ingredient.call(EnergyDrink), "Energy Drink" }
        KButton { variant: KButtonVariant::Ghost, onclick: move |_| props.add_ingredient.call(MotorOil), "Motor Oil" }
        KButton { variant: KButtonVariant::Ghost, onclick: move |_| props.add_ingredient.call(MegaBean), "Mega Bean" }
        KButton { variant: KButtonVariant::Ghost, onclick: move |_| props.add_ingredient.call(Chili), "Chili" }
        KButton { variant: KButtonVariant::Ghost, onclick: move |_| props.add_ingredient.call(Battery), "Battery" }
        KButton { variant: KButtonVariant::Ghost, onclick: move |_| props.add_ingredient.call(Iodine), "Iodine" }
        KButton { variant: KButtonVariant::Ghost, onclick: move |_| props.add_ingredient.call(Addy), "Addy" }
        KButton { variant: KButtonVariant::Ghost, onclick: move |_| props.add_ingredient.call(HorseSemen), "Horse Semen" }
    }
}
