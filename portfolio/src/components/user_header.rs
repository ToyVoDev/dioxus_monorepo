use crate::MyState;
use dioxus::prelude::*;

#[component]
pub fn UserHeader() -> Element {
    let resume = use_context::<MyState>().resume.cloned().unwrap_or_default();

    rsx! {
        div {
            class:"d-flex justify-content-space-between",
            div {
                class:"d-flex",
                img {
                    class: "resume-img",
                    alt: "resume",
                    src: "{resume.basics.image}"
                }
                div {
                    h2 { "{resume.basics.name}" }
                    h4 {
                        Link {
                            to: "https://gitconnected.com/{resume.basics.username}",
                            rel: "noopener noreferrer",
                            new_tab: true,
                            "@{resume.basics.username}"
                        }
                    }
                    p { "{resume.basics.label}" }
                    p { "Coding in {resume.basics.region}" }
                    p { "{resume.basics.years_of_experience} years of experience as a developer" }
                    p { "{resume.basics.headline}" }
                }
            }
            div {
                class:"d-flex align-items-start",
                Link {
                    class:"view-resume-link",
                    to: "https://gitconnected.com/{resume.basics.username}/resume",
                    rel: "noopener noreferrer",
                    new_tab: true,
                    "View Résumé ➜"
                }
            }
        }
    }
}
