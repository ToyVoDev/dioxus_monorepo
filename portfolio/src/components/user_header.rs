use crate::MyState;
use dioxus::prelude::*;
use kinetic_ui::ThemeToggle;

#[component]
pub fn UserHeader() -> Element {
    let resume = use_context::<MyState>().resume.cloned().unwrap_or_default();

    // Derive username from gitconnected profile if not directly available
    let username = if resume.basics.username.is_empty() {
        resume
            .basics
            .profiles
            .iter()
            .find(|p| p.network.to_lowercase() == "gitconnected")
            .map(|p| p.username.clone())
            .unwrap_or_default()
    } else {
        resume.basics.username.clone()
    };

    // Derive region from location object if not directly available
    let region = if resume.basics.region.is_empty() {
        let loc = &resume.basics.location;
        if !loc.city.is_empty() && !loc.region.is_empty() {
            format!("{}, {}", loc.city, loc.region)
        } else if !loc.region.is_empty() {
            loc.region.clone()
        } else {
            loc.city.clone()
        }
    } else {
        resume.basics.region.clone()
    };

    // Compute years of experience from earliest work start year
    let years_of_experience = if resume.basics.years_of_experience > 0 {
        resume.basics.years_of_experience as i64
    } else {
        let current_year = 2026_i64;
        resume
            .work
            .iter()
            .filter_map(|w| w.start.year)
            .min()
            .map(|earliest| current_year - earliest)
            .unwrap_or(0)
    };

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
                    if !username.is_empty() {
                        h4 {
                            Link {
                                to: "https://gitconnected.com/{username}",
                                rel: "noopener noreferrer",
                                new_tab: true,
                                "@{username}"
                            }
                        }
                    }
                    p { "{resume.basics.label}" }
                    if !region.is_empty() {
                        p { "Coding in {region}" }
                    }
                    if years_of_experience > 0 {
                        p { "{years_of_experience} years of experience as a developer" }
                    }
                    if !resume.basics.headline.is_empty() {
                        p { "{resume.basics.headline}" }
                    }
                }
            }
            div {
                class:"d-flex align-items-start",
                gap: "8px",
                ThemeToggle {}
                if !username.is_empty() {
                    Link {
                        class:"view-resume-link",
                        to: "https://gitconnected.com/{username}/resume",
                        rel: "noopener noreferrer",
                        new_tab: true,
                        "View Résumé ➜"
                    }
                }
            }
        }
    }
}
