use crate::Effect;
use crate::sellable::Sellable;
use dioxus::prelude::*;
use web_sys::wasm_bindgen::JsCast;

const SCALE: f64 = 100.;

fn draw_circle(
    context: &web_sys::CanvasRenderingContext2d,
    center: (f64, f64),
    radius: f64,
    color: &str,
    filled: bool,
) {
    context.save();
    context.begin_path();
    context
        .arc(center.0, center.1, radius, 0.0, std::f64::consts::PI * 2.0)
        .unwrap();
    if filled {
        context.set_fill_style_str(color);
        context.fill();
    } else {
        context.set_stroke_style_str(color);
        context.stroke();
    }
    context.restore();
}

fn draw_vector(
    context: &web_sys::CanvasRenderingContext2d,
    origin: (f64, f64),
    magnitude: f64,
    direction: (f64, f64),
    color: &str,
) {
    context.save();
    context.begin_path();
    context.move_to(origin.0, origin.1);
    context.line_to(
        origin.0 + magnitude * direction.0,
        origin.1 + magnitude * direction.1,
    );
    context.set_stroke_style_str(color);
    context.set_line_width(2.0);
    context.stroke();
    context.restore();
}

#[derive(PartialEq, Clone, Props)]
pub struct ComponentProps {
    pub added_effect: Signal<Option<Effect>>,
    pub previous_working_product: Signal<Sellable>,
    pub working_product: Signal<Sellable>,
}

#[component]
pub fn MixMap(props: ComponentProps) -> Element {
    use_effect(move || {
        let added_effect = props.added_effect.read();
        let previous_working_product = props.previous_working_product.read();
        let working_product = props.working_product.read();
        let canvas = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("mix_map")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();
        let width = canvas.width() as f64;
        let height = canvas.height() as f64;
        let center = (width / 2., height / 2.);
        context.clear_rect(0., 0., width, height);
        draw_circle(&context, center, 5. * SCALE, "#888", false);
        let all_effects = [
            Effect::AntiGravity,
            Effect::Athletic,
            Effect::Balding,
            Effect::BrightEyed,
            Effect::Calming,
            Effect::CalorieDense,
            Effect::Cyclopean,
            Effect::Disorienting,
            Effect::Electrifying,
            Effect::Energizing,
            Effect::Euphoric,
            Effect::Explosive,
            Effect::Focused,
            Effect::Foggy,
            Effect::Gingeritis,
            Effect::Glowing,
            Effect::Jennerising,
            Effect::Laxative,
            Effect::Lethal,
            Effect::LongFaced,
            Effect::Munchies,
            Effect::Paranoia,
            Effect::Refreshing,
            Effect::Schizophrenic,
            Effect::Sedating,
            Effect::SeizureInducing,
            Effect::Shrinking,
            Effect::Slippery,
            Effect::Smelly,
            Effect::Sneaky,
            Effect::Spicy,
            Effect::ThoughtProvoking,
            Effect::Toxic,
            Effect::TropicThunder,
            Effect::Zombifying,
        ];
        for effect in all_effects {
            let direction = effect.direction();
            let magnitude = effect.magnitude();
            let circle_center = (
                center.0 + direction.0 * magnitude * SCALE,
                center.1 + direction.1 * magnitude * SCALE,
            );
            let filled = previous_working_product.effects.contains(&effect)
                || working_product.effects.contains(&effect);
            draw_circle(
                &context,
                circle_center,
                0.5 * SCALE,
                &effect.color(),
                filled,
            );
        }
        if let Some(added_effect) = *added_effect {
            previous_working_product.effects.iter().for_each(|effect| {
                let direction = effect.direction();
                let magnitude = effect.magnitude();
                let circle_center = (
                    center.0 + direction.0 * magnitude * SCALE,
                    center.1 + direction.1 * magnitude * SCALE,
                );
                draw_vector(
                    &context,
                    circle_center,
                    added_effect.magnitude() * SCALE,
                    added_effect.direction(),
                    &effect.color(),
                );
            });
        }
        for effect in all_effects {
            let direction = effect.direction();
            let magnitude = effect.magnitude();
            let circle_center = (
                center.0 + direction.0 * magnitude * SCALE,
                center.1 + direction.1 * magnitude * SCALE,
            );
            let filled = previous_working_product.effects.contains(&effect)
                || working_product.effects.contains(&effect);
            if filled {
                context.set_fill_style_str("#000");
            } else {
                context.set_fill_style_str("#fff");
            }
            context.set_text_align("center");
            context.set_text_baseline("middle");
            context
                .fill_text(
                    format!("{effect:?}").as_str(),
                    circle_center.0,
                    circle_center.1,
                )
                .unwrap();
        }
    });

    rsx! {
        div {
            class: "flex flex-col justify-center col-span-full",
            canvas {
                id: "mix_map",
                width: "1000",
                height: "1000",
                class: "w-auto",
            }
        }
    }
}
