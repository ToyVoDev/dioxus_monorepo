use crate::components::Button;
use crate::sellable::Ingredient;
use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct ComponentProps {
    pub add_ingredient: EventHandler<Ingredient>,
}

#[component]
pub fn AddIngredients(props: ComponentProps) -> Element {
    use Ingredient::*;
    rsx! {
        div { class: "col-span-full", "Add Ingredient" }
        Button { onclick: move |_| props.add_ingredient.call(Cuke), "Cuke" }
        Button { onclick: move |_| props.add_ingredient.call(Banana), "Banana" }
        Button { onclick: move |_| props.add_ingredient.call(Paracetamol), "Paracetamol" }
        Button { onclick: move |_| props.add_ingredient.call(Donut), "Donut" }
        Button { onclick: move |_| props.add_ingredient.call(Viagra), "Viagra" }
        Button { onclick: move |_| props.add_ingredient.call(MouthWash), "Mouth Wash" }
        Button { onclick: move |_| props.add_ingredient.call(FluMedicine), "Flu Medicine" }
        Button { onclick: move |_| props.add_ingredient.call(Gasoline), "Gasoline" }
        Button { onclick: move |_| props.add_ingredient.call(EnergyDrink), "Energy Drink" }
        Button { onclick: move |_| props.add_ingredient.call(MotorOil), "Motor Oil" }
        Button { onclick: move |_| props.add_ingredient.call(MegaBean), "Mega Bean" }
        Button { onclick: move |_| props.add_ingredient.call(Chili), "Chili" }
        Button { onclick: move |_| props.add_ingredient.call(Battery), "Battery" }
        Button { onclick: move |_| props.add_ingredient.call(Iodine), "Iodine" }
        Button { onclick: move |_| props.add_ingredient.call(Addy), "Addy" }
        Button { onclick: move |_| props.add_ingredient.call(HorseSemen), "Horse Semen" }
    }
}
