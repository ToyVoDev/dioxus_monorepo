use crate::MyState;
use dioxus::prelude::*;

#[component]
pub fn Projects() -> Element {
    let resume = use_context::<MyState>().resume.cloned().unwrap_or_default();
    rsx! {
        div {
            h3 { class: "section-header", "Projects" }
            ul {
                for project in resume.projects {
                    li { class: "project-item",
                        h4 {
                            class: "bold",
                            "{project.display_name}"
                            if !project.website.is_empty() {
                                Link {
                                    padding_left: "4px",
                                    to: "{project.website}",
                                    new_tab: true,
                                    "{project.website}"
                                }
                            } else if !project.github_url.is_empty() {
                                Link {
                                    padding_left: "4px",
                                    to: "{project.github_url}",
                                    new_tab: true,
                                    "{project.github_url}"
                                }
                            }
                        }
                        p {
                            if project.description.is_empty() {
                                "{project.summary}"
                            } else {
                                "{project.description}"
                            }
                        }
                        div { class: "chip-wrapper justify-content-start",
                            for item in project.languages {
                                div {
                                    class: "chip",
                                    "{item}"
                                }
                            }
                            for item in project.libraries {
                                div {
                                    class: "chip",
                                    "{item}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
