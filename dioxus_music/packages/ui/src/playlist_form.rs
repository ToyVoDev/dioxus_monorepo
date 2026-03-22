use dioxus::prelude::*;
use dioxus_music_api::models::SmartPlaylistRules;
use kinetic_ui::{Button, ButtonVariant, Input};

const PLAYLIST_FORM_CSS: Asset = asset!("/assets/styling/playlist-form.css");

#[derive(Debug, Clone, PartialEq)]
pub enum PlaylistFormMode {
    CreateManual,
    CreateSmart {
        genres: Vec<String>,
    },
    EditManual {
        name: String,
    },
    EditSmart {
        name: String,
        rules: SmartPlaylistRules,
        genres: Vec<String>,
    },
}

#[component]
pub fn PlaylistFormModal(
    mode: PlaylistFormMode,
    on_save: EventHandler<(String, Option<SmartPlaylistRules>)>,
    on_cancel: EventHandler<()>,
) -> Element {
    let is_smart = matches!(
        mode,
        PlaylistFormMode::CreateSmart { .. } | PlaylistFormMode::EditSmart { .. }
    );

    let initial_name = match &mode {
        PlaylistFormMode::EditManual { name } | PlaylistFormMode::EditSmart { name, .. } => {
            name.clone()
        }
        _ => String::new(),
    };

    let genres = match &mode {
        PlaylistFormMode::CreateSmart { genres } | PlaylistFormMode::EditSmart { genres, .. } => {
            genres.clone()
        }
        _ => Vec::new(),
    };

    let initial_include: Vec<String> = match &mode {
        PlaylistFormMode::EditSmart { rules, .. } => rules.include_genres.clone(),
        _ => Vec::new(),
    };

    let initial_exclude: Vec<String> = match &mode {
        PlaylistFormMode::EditSmart { rules, .. } => rules.exclude_genres.clone(),
        _ => Vec::new(),
    };

    let mut name = use_signal(move || initial_name);
    let mut include_genres = use_signal(move || initial_include);
    let mut exclude_genres = use_signal(move || initial_exclude);

    let title = match &mode {
        PlaylistFormMode::CreateManual => "New Manual Playlist",
        PlaylistFormMode::CreateSmart { .. } => "New Smart Playlist",
        PlaylistFormMode::EditManual { .. } => "Edit Playlist",
        PlaylistFormMode::EditSmart { .. } => "Edit Smart Playlist",
    };

    rsx! {
        document::Link { rel: "stylesheet", href: PLAYLIST_FORM_CSS }

        div {
            class: "playlist-form-overlay",
            onclick: move |_| on_cancel.call(()),

            div {
                class: "playlist-form-card",
                onclick: move |e| e.stop_propagation(),

                h2 { class: "playlist-form-card__title", "{title}" }

                div { class: "playlist-form-card__field",
                    label { "Name" }
                    Input {
                        r#type: "text",
                        value: "{name}",
                        placeholder: "Playlist name",
                        oninput: move |e: FormEvent| name.set(e.value()),
                    }
                }

                if is_smart {
                    div { class: "playlist-form-card__field",
                        label { "Include Genres (empty = all)" }
                        div { class: "playlist-form-card__chips",
                            for genre in genres.iter() {
                                {
                                    let g = genre.clone();
                                    let g2 = genre.clone();
                                    let included = include_genres.read().contains(&g);
                                    rsx! {
                                        button {
                                            class: if included { "genre-chip genre-chip--active" } else { "genre-chip" },
                                            onclick: move |_| {
                                                let g = g2.clone();
                                                include_genres.with_mut(|list| {
                                                    if list.contains(&g) {
                                                        list.retain(|x| x != &g);
                                                    } else {
                                                        list.push(g);
                                                    }
                                                });
                                            },
                                            "{genre}"
                                        }
                                    }
                                }
                            }
                        }
                    }

                    div { class: "playlist-form-card__field",
                        label { "Exclude Genres" }
                        div { class: "playlist-form-card__chips",
                            for genre in genres.iter() {
                                {
                                    let g = genre.clone();
                                    let g2 = genre.clone();
                                    let excluded = exclude_genres.read().contains(&g);
                                    rsx! {
                                        button {
                                            class: if excluded { "genre-chip genre-chip--exclude" } else { "genre-chip" },
                                            onclick: move |_| {
                                                let g = g2.clone();
                                                exclude_genres.with_mut(|list| {
                                                    if list.contains(&g) {
                                                        list.retain(|x| x != &g);
                                                    } else {
                                                        list.push(g);
                                                    }
                                                });
                                            },
                                            "{genre}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                div { class: "playlist-form-card__actions",
                    Button {
                        variant: ButtonVariant::Ghost,
                        onclick: move |_| on_cancel.call(()),
                        "Cancel"
                    }
                    Button {
                        variant: ButtonVariant::Primary,
                        onclick: move |_| {
                            let rules = if is_smart {
                                Some(SmartPlaylistRules {
                                    include_genres: include_genres.read().clone(),
                                    exclude_genres: exclude_genres.read().clone(),
                                })
                            } else {
                                None
                            };
                            on_save.call((name.read().clone(), rules));
                        },
                        "Save"
                    }
                }
            }
        }
    }
}
