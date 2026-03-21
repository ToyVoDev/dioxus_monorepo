use crate::Route;
use dioxus::prelude::*;
use dioxus_music_api::models::SmartPlaylistRules;
use dioxus_music_ui::{PlaylistFormModal, PlaylistFormMode};

#[component]
pub fn Playlists() -> Element {
    let mut playlists = use_server_future(dioxus_music_api::get_playlists)?;
    let mut show_modal = use_signal(|| Option::<PlaylistFormMode>::None);
    let mut genres = use_signal(Vec::<String>::new);

    let result = playlists.read().clone();

    rsx! {
        div { style: "padding: var(--k-space-6);",
            div {
                style: "display: flex; align-items: center; justify-content: space-between; margin-bottom: var(--k-space-4);",
                h2 {
                    style: "font-family: var(--k-font-display); font-size: 1.75rem; color: var(--k-on-surface);",
                    "Playlists"
                }
                div {
                    style: "display: flex; gap: var(--k-space-2);",
                    button {
                        style: "background: var(--k-surface-highest); border: none; color: var(--k-on-surface); padding: var(--k-space-2) var(--k-space-3); border-radius: var(--k-radius-default); cursor: pointer; font-size: 0.875rem;",
                        title: "New manual playlist",
                        onclick: move |_| {
                            show_modal.set(Some(PlaylistFormMode::CreateManual));
                        },
                        "+ Manual"
                    }
                    button {
                        style: "background: var(--k-surface-highest); border: none; color: var(--k-on-surface); padding: var(--k-space-2) var(--k-space-3); border-radius: var(--k-radius-default); cursor: pointer; font-size: 0.875rem;",
                        title: "New smart playlist",
                        onclick: move |_| async move {
                            if let Ok(g) = dioxus_music_api::get_genres().await {
                                genres.set(g.clone());
                                show_modal.set(Some(PlaylistFormMode::CreateSmart { genres: g }));
                            }
                        },
                        "+ Smart"
                    }
                }
            }

            match result {
                Some(Ok(list)) => rsx! {
                    div {
                        style: "display: flex; flex-direction: column; gap: var(--k-space-1);",
                        for pl in list.iter() {
                            Link {
                                class: "sidebar__nav-item",
                                to: Route::PlaylistView { id: pl.id },
                                span {
                                    style: "display: flex; align-items: center; gap: var(--k-space-2);",
                                    span {
                                        style: "font-family: var(--k-font-mono); font-size: 0.625rem; color: var(--k-on-surface-variant); background: var(--k-surface-highest); padding: 1px 4px; border-radius: var(--k-radius-sm);",
                                        if pl.playlist_type == "smart" { "S" } else { "M" }
                                    }
                                    "{pl.name}"
                                }
                            }
                        }
                    }
                },
                Some(Err(_)) => rsx! { p { style: "color: var(--k-on-surface-variant);", "Error loading playlists" } },
                None => rsx! { p { style: "color: var(--k-on-surface-variant);", "Loading..." } },
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
