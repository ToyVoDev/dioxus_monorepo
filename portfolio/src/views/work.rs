use crate::MyState;
use dioxus::prelude::*;

#[component]
pub fn Work() -> Element {
    let resume = use_context::<MyState>().resume.cloned().unwrap_or_default();
    rsx! {
        div {
            h3 { class: "section-title", "Work" }
            ul {
                for work in resume.work {
                    li { class: "work-item",
                        div {
                            display: "flex",
                            flex_direction: "row",
                            align_items: "center",
                            gap: "4px",
                            // Should be a non breaking space
                            p { class: "d-inline-block bold", "{work.name}" }
                            span {
                                match (work.start.year, work.end.year) {
                                    (Some(start), Some(end)) => {
                                        format!("{} ⋅ {start} to {end}", work.location)
                                    },
                                    (Some(start), _) => {
                                        format!("{} ⋅ Since {start}", work.location)
                                    },
                                    (_,_) => {
                                        work.location.to_string()
                                    }
                                }
                            }
                            if !work.website.is_empty() {
                                Link {
                                    to: "{work.website}",
                                    new_tab: true,
                                    "{work.website}"
                                }
                            }
                        }
                        h4 { class: "bold", "{work.position}" }
                        p { class: "pre-wrap", "{work.summary}" }
                    }
                }
            }
        }
    }
}
