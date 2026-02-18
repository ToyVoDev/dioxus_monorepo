use crate::Route;
use dioxus::prelude::*;
use dioxus_music_api::models::SmartPlaylistRules;
use dioxus_music_ui::{PlaylistFormModal, PlaylistFormMode};

#[component]
pub fn PlaylistSidebarSection() -> Element {
    let mut playlists = use_server_future(dioxus_music_api::get_playlists)?;
    let mut show_modal = use_signal(|| Option::<PlaylistFormMode>::None);
    let mut genres = use_signal(Vec::<String>::new);

    let result = playlists.read().clone();

    rsx! {
        div { class: "sidebar-section",
            div { class: "sidebar-section__header",
                span { class: "sidebar-section__title", "Playlists" }
                div { class: "sidebar-section__actions",
                    button {
                        class: "sidebar-section__btn",
                        title: "New manual playlist",
                        onclick: move |_| {
                            show_modal.set(Some(PlaylistFormMode::CreateManual));
                        },
                        "+"
                    }
                    button {
                        class: "sidebar-section__btn",
                        title: "New smart playlist",
                        onclick: move |_| async move {
                            if let Ok(g) = dioxus_music_api::get_genres().await {
                                genres.set(g.clone());
                                show_modal.set(Some(PlaylistFormMode::CreateSmart { genres: g }));
                            }
                        },
                        "S+"
                    }
                }
            }

            match result {
                Some(Ok(list)) => rsx! {
                    div { class: "sidebar__nav",
                        for pl in list.iter() {
                            Link {
                                to: Route::PlaylistView { id: pl.id },
                                span { class: "sidebar-playlist-link",
                                    span { class: "sidebar-playlist-link__icon",
                                        if pl.playlist_type == "smart" { "S" } else { "P" }
                                    }
                                    "{pl.name}"
                                }
                            }
                        }
                    }
                },
                Some(Err(_)) => rsx! {},
                None => rsx! {},
            }
        }

        if let Some(mode) = show_modal() {
            PlaylistFormModal {
                mode: mode.clone(),
                on_save: move |(name, rules): (String, Option<SmartPlaylistRules>)| async move {
                    let result = match rules {
                        Some(r) => dioxus_music_api::create_smart_playlist(name, r).await,
                        None => dioxus_music_api::create_manual_playlist(name).await,
                    };
                    if result.is_ok() {
                        playlists.restart();
                    }
                    show_modal.set(None);
                },
                on_cancel: move |_| {
                    show_modal.set(None);
                },
            }
        }
    }
}
