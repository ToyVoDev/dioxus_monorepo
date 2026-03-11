use crate::MyState;
use dioxus::prelude::*;

#[component]
pub fn Me() -> Element {
    let resume = use_context::<MyState>().resume.cloned().unwrap_or_default();
    rsx! {
        div {
            h3 { class: "section-title", "About Me" }
            p { class: "pre-wrap", "{resume.basics.summary}" }
        }
        div {
            h3 { class: "section-title", "Skills" }
            div {
                class: "chip-wrapper",
                for skill in resume.skills {
                    div {
                        class: "chip",
                        "{skill.name}"
                    }
                }
            }
        }
        div {
            h3 { class: "section-title", "Profiles" }
            ul {
                for (i, profile) in resume.basics.profiles.iter().enumerate() {
                    li {
                        class: "profile-link",
                        if i != 0 {" | "}
                        Link {
                            to: "{profile.url}",
                            new_tab: true,
                            rel: "noreferrer noopener",
                            "{profile.network}"
                        }
                    }
                }
            }
        }
    }
}
