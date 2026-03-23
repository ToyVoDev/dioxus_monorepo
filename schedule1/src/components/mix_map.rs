use crate::Effect;
use crate::sellable::Sellable;
use dioxus::prelude::*;

const SCALE: f64 = 100.;
const SIZE: f64 = 1000.;

const ALL_EFFECTS: [Effect; 35] = [
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

#[derive(PartialEq, Clone, Props)]
pub struct ComponentProps {
    pub added_effect: Signal<Option<Effect>>,
    pub previous_working_product: Signal<Sellable>,
    pub working_product: Signal<Sellable>,
}

#[component]
pub fn MixMap(props: ComponentProps) -> Element {
    let added_effect = props.added_effect.read();
    let previous_working_product = props.previous_working_product.read();
    let working_product = props.working_product.read();
    let center = SIZE / 2.;
    let radius = 0.5 * SCALE;

    rsx! {
        div {
            grid_column: "1 / -1",
            position: "relative",
            width: "100%",
            padding_bottom: "100%",
            overflow: "hidden",

            // Inner container scaled to 1000x1000 coordinate space
            div {
                position: "absolute",
                top: "0",
                left: "0",
                width: "100%",
                height: "100%",

                // Outer ring + vector lines via SVG
                svg {
                    style: "position: absolute; top: 0; left: 0; width: 100%; height: 100%;",
                    view_box: "0 0 {SIZE} {SIZE}",
                    // Outer ring
                    circle {
                        cx: "{center}",
                        cy: "{center}",
                        r: "{5. * SCALE}",
                        fill: "none",
                        stroke: "#888",
                        stroke_width: "1",
                    }
                    // Vector lines for added effect
                    if let Some(ae) = *added_effect {
                        for effect in previous_working_product.effects.iter() {
                            {
                                let dir = effect.direction();
                                let mag = effect.magnitude();
                                let x1 = center + dir.0 * mag * SCALE;
                                let y1 = center + dir.1 * mag * SCALE;
                                let ae_dir = ae.direction();
                                let ae_mag = ae.magnitude();
                                let x2 = x1 + ae_dir.0 * ae_mag * SCALE;
                                let y2 = y1 + ae_dir.1 * ae_mag * SCALE;
                                let color = effect.color();
                                rsx! {
                                    line {
                                        x1: "{x1}",
                                        y1: "{y1}",
                                        x2: "{x2}",
                                        y2: "{y2}",
                                        stroke: "{color}",
                                        stroke_width: "2",
                                    }
                                }
                            }
                        }
                    }
                }

                // Effect circles as HTML elements
                for effect in ALL_EFFECTS {
                    {
                        let dir = effect.direction();
                        let mag = effect.magnitude();
                        let cx = center + dir.0 * mag * SCALE;
                        let cy = center + dir.1 * mag * SCALE;
                        let filled = previous_working_product.effects.contains(&effect)
                            || working_product.effects.contains(&effect);
                        let color = effect.color();
                        let label = format!("{effect:?}");
                        // Position as percentage of the 1000x1000 space
                        let left_pct = (cx - radius) / SIZE * 100.;
                        let top_pct = (cy - radius) / SIZE * 100.;
                        let size_pct = (radius * 2.) / SIZE * 100.;
                        rsx! {
                            div {
                                position: "absolute",
                                left: "{left_pct:.2}%",
                                top: "{top_pct:.2}%",
                                width: "{size_pct:.2}%",
                                height: "{size_pct:.2}%",
                                border_radius: "50%",
                                display: "flex",
                                align_items: "center",
                                justify_content: "center",
                                font_size: "0.7vw",
                                white_space: "nowrap",
                                border: if filled { "none" } else { "1px solid {color}" },
                                background: if filled { "{color}" } else { "none" },
                                color: if filled { "var(--primary-color)" } else { "var(--secondary-color)" },
                                "{label}"
                            }
                        }
                    }
                }
            }
        }
    }
}
