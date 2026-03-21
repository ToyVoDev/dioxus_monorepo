use crate::Route;
use dioxus::prelude::*;
use dioxus_music_ui::{group_tracks_into_albums, TrackList};

#[component]
pub fn Library() -> Element {
    let tracks = use_server_future(dioxus_music_api::get_library)?;
    let mut show_all_songs = use_signal(|| false);

    let result = tracks.read().clone();

    rsx! {
        div {
            style: "padding: var(--k-space-6);",
            div {
                style: "display: flex; align-items: baseline; justify-content: space-between; margin-bottom: var(--k-space-6);",
                div {
                    h1 {
                        style: "font-family: var(--k-font-display); font-size: 1.75rem; font-weight: 700; color: var(--k-on-surface);",
                        "Library"
                    }
                    p {
                        style: "color: var(--k-on-surface-variant); font-size: 0.875rem; margin-top: var(--k-space-1);",
                        "Your high-fidelity audio repository"
                    }
                }
                button {
                    style: "background: transparent; border: none; color: var(--k-primary); font-size: 0.875rem; cursor: pointer; padding: var(--k-space-1) var(--k-space-2); border-radius: var(--k-radius-default);",
                    onclick: move |_| show_all_songs.toggle(),
                    if show_all_songs() { "Album Grid" } else { "All Songs" }
                }
            }

            match result {
                Some(Ok(track_list)) => {
                    if show_all_songs() {
                        rsx! { TrackList { tracks: track_list } }
                    } else {
                        let albums = group_tracks_into_albums(&track_list);
                        rsx! {
                            div {
                                style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(160px, 1fr)); gap: var(--k-space-4);",
                                for album in albums {
                                    {
                                        let initial = album.name.chars().next().unwrap_or('?').to_uppercase().to_string();
                                        let album_name = album.name.clone();
                                        rsx! {
                                            Link {
                                                style: "cursor: pointer; text-decoration: none; color: inherit; display: block; transition: transform 150ms ease;",
                                                to: Route::AlbumDetail { name: album_name },
                                                div {
                                                    style: "aspect-ratio: 1; background: var(--k-surface-highest); border-radius: var(--k-radius-lg); display: flex; align-items: center; justify-content: center; font-family: var(--k-font-display); font-size: 2rem; color: var(--k-on-surface-variant); margin-bottom: var(--k-space-2);",
                                                    "{initial}"
                                                }
                                                div {
                                                    style: "font-size: 0.875rem; color: var(--k-on-surface); white-space: nowrap; overflow: hidden; text-overflow: ellipsis;",
                                                    "{album.name}"
                                                }
                                                div {
                                                    style: "font-size: 0.75rem; color: var(--k-on-surface-variant); white-space: nowrap; overflow: hidden; text-overflow: ellipsis;",
                                                    "{album.artist}"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
                Some(Err(e)) => rsx! { p { style: "color: var(--k-error);", "Error loading library: {e}" } },
                None => rsx! { p { style: "color: var(--k-on-surface-variant);", "Loading..." } },
            }
        }
    }
}
