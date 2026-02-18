use crate::Route;
use dioxus::prelude::*;
use dioxus_music_api::models::SmartPlaylistRules;
use dioxus_music_ui::{PlaylistFormModal, PlaylistFormMode, TrackList};
use uuid::Uuid;

const PLAYLIST_VIEW_CSS: Asset = asset!("/assets/playlist_view.css");

#[component]
pub fn PlaylistView(id: ReadSignal<Uuid>) -> Element {
    let mut detail = use_server_future(move || {
        let id = id();
        async move { dioxus_music_api::get_playlist(id).await }
    })?;
    let mut tracks = use_server_future(move || {
        let id = id();
        async move { dioxus_music_api::get_playlist_tracks(id).await }
    })?;

    let mut confirming_delete = use_signal(|| false);
    let mut show_track_picker = use_signal(|| false);
    let mut show_edit_modal = use_signal(|| Option::<PlaylistFormMode>::None);
    let mut genres = use_signal(Vec::<String>::new);

    // Reset UI state when switching playlists
    use_effect(move || {
        let _ = id();
        confirming_delete.set(false);
        show_track_picker.set(false);
        show_edit_modal.set(None);
    });

    let nav = navigator();

    let detail_result = detail.read().clone();
    let tracks_result = tracks.read().clone();

    rsx! {
        document::Link { rel: "stylesheet", href: PLAYLIST_VIEW_CSS }

        match detail_result {
            Some(Ok(pl)) => {
                let pl_name = pl.name.clone();
                let pl_type = pl.playlist_type.clone();
                let is_smart = pl_type == "smart";
                let pl_rules = pl.rules.clone();

                let edit_name = pl_name.clone();
                let edit_rules = pl_rules.clone();

                rsx! {
                    div { class: "playlist-view__header",
                        h1 { class: "playlist-view__title", "{pl_name}" }
                        span {
                            class: if is_smart { "playlist-view__badge playlist-view__badge--smart" } else { "playlist-view__badge playlist-view__badge--manual" },
                            "{pl_type}"
                        }

                        div { class: "playlist-view__actions",
                            if !is_smart {
                                button {
                                    class: "playlist-view__btn playlist-view__btn--add",
                                    onclick: move |_| {
                                        show_track_picker.set(true);
                                    },
                                    "Add Tracks"
                                }
                            }

                            button {
                                class: "playlist-view__btn playlist-view__btn--edit",
                                onclick: move |_| {
                                    let name = edit_name.clone();
                                    let rules = edit_rules.clone();
                                    async move {
                                        if is_smart {
                                            if let Ok(g) = dioxus_music_api::get_genres().await {
                                                genres.set(g.clone());
                                                show_edit_modal.set(Some(PlaylistFormMode::EditSmart {
                                                    name,
                                                    rules: rules.unwrap_or_default(),
                                                    genres: g,
                                                }));
                                            }
                                        } else {
                                            show_edit_modal.set(Some(PlaylistFormMode::EditManual {
                                                name,
                                            }));
                                        }
                                    }
                                },
                                "Edit"
                            }

                            if confirming_delete() {
                                div { class: "playlist-view__delete-confirm",
                                    "Delete?"
                                    button {
                                        class: "playlist-view__btn playlist-view__btn--confirm-yes",
                                        onclick: move |_| async move {
                                            if dioxus_music_api::delete_playlist(id()).await.is_ok() {
                                                nav.push(Route::Library {});
                                            }
                                        },
                                        "Yes"
                                    }
                                    button {
                                        class: "playlist-view__btn playlist-view__btn--confirm-no",
                                        onclick: move |_| confirming_delete.set(false),
                                        "No"
                                    }
                                }
                            } else {
                                button {
                                    class: "playlist-view__btn playlist-view__btn--delete",
                                    onclick: move |_| confirming_delete.set(true),
                                    "Delete"
                                }
                            }
                        }
                    }
                }
            },
            Some(Err(e)) => rsx! {
                p { "Error: {e}" }
            },
            None => rsx! {
                p { "Loading..." }
            },
        }

        match tracks_result {
            Some(Ok(track_list)) => {
                if track_list.is_empty() {
                    rsx! { p { class: "playlist-view__empty", "No tracks in this playlist yet." } }
                } else {
                    rsx! { TrackList { tracks: track_list } }
                }
            },
            Some(Err(e)) => rsx! {
                p { "Error loading tracks: {e}" }
            },
            None => rsx! {
                p { "Loading tracks..." }
            },
        }

        if show_track_picker() {
            TrackPicker {
                playlist_id: id(),
                on_added: move || {
                    tracks.restart();
                },
                on_close: move || {
                    show_track_picker.set(false);
                },
            }
        }

        if let Some(mode) = show_edit_modal() {
            PlaylistFormModal {
                mode: mode.clone(),
                on_save: move |(name, rules): (String, Option<SmartPlaylistRules>)| async move {
                    let current_id = id();
                    if let Some(r) = rules {
                        let _ = dioxus_music_api::update_smart_playlist_rules(current_id, r).await;
                    }
                    let _ = dioxus_music_api::rename_playlist(current_id, name).await;
                    detail.restart();
                    tracks.restart();
                    show_edit_modal.set(None);
                },
                on_cancel: move |_| {
                    show_edit_modal.set(None);
                },
            }
        }
    }
}

#[component]
fn TrackPicker(
    playlist_id: Uuid,
    on_added: EventHandler<()>,
    on_close: EventHandler<()>,
) -> Element {
    let all_tracks = use_server_future(dioxus_music_api::get_library)?;
    let result = all_tracks.read().clone();

    rsx! {
        div {
            class: "track-picker-overlay",
            onclick: move |_| on_close.call(()),

            div {
                class: "track-picker",
                onclick: move |e| e.stop_propagation(),

                h3 { class: "track-picker__title", "Add Tracks" }

                match result {
                    Some(Ok(tracks)) => rsx! {
                        for track in tracks.iter() {
                            {
                                let tid = track.id;
                                let pid = playlist_id;
                                rsx! {
                                    div { class: "track-picker__row",
                                        div { class: "track-picker__info",
                                            span { class: "track-picker__track-title", "{track.title}" }
                                            span { class: "track-picker__track-artist", "{track.artist}" }
                                        }
                                        button {
                                            class: "track-picker__add-btn",
                                            onclick: move |_| async move {
                                                if dioxus_music_api::add_track_to_playlist(pid, tid).await.is_ok() {
                                                    on_added.call(());
                                                }
                                            },
                                            "+"
                                        }
                                    }
                                }
                            }
                        }
                    },
                    Some(Err(e)) => rsx! { p { "Error: {e}" } },
                    None => rsx! { p { "Loading..." } },
                }

                div { class: "track-picker__close",
                    button {
                        class: "track-picker__close-btn",
                        onclick: move |_| on_close.call(()),
                        "Done"
                    }
                }
            }
        }
    }
}
