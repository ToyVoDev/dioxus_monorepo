use dioxus::prelude::*;
use dioxus_music_ui::{TrackList, group_tracks_into_albums};

use crate::Route;

const LIBRARY_CSS: Asset = asset!("/assets/library.css");

#[component]
pub fn Library() -> Element {
    let tracks = use_server_future(dioxus_music_api::get_library)?;
    let mut show_all_songs = use_signal(|| false);

    let result = tracks.read().clone();

    rsx! {
        document::Link { rel: "stylesheet", href: LIBRARY_CSS }

        div { class: "library",
            div { class: "library__header",
                div {
                    h1 { class: "library__title",
                        if show_all_songs() { "All Songs" } else { "Albums" }
                    }
                }
                button {
                    class: "library__toggle",
                    onclick: move |_| show_all_songs.toggle(),
                    if show_all_songs() { "Show Albums" } else { "Show All Songs" }
                }
            }

            match result {
                Some(Ok(tracks)) => rsx! {
                    if show_all_songs() {
                        TrackList { tracks: tracks.clone() }
                    } else {
                        {
                            let albums = group_tracks_into_albums(&tracks);
                            rsx! {
                                p { class: "library__subtitle", "{albums.len()} albums" }
                                div { class: "album-grid",
                                    for album in albums {
                                        Link {
                                            class: "album-card",
                                            to: Route::AlbumDetail { name: album.name.clone() },
                                            div { class: "album-card__art",
                                                "{album.name.chars().next().unwrap_or('?')}"
                                            }
                                            div { class: "album-card__name", "{album.name}" }
                                            div { class: "album-card__artist", "{album.artist}" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
                Some(Err(e)) => rsx! {
                    p { "Error loading library: {e}" }
                },
                None => rsx! {
                    p { "Loading..." }
                },
            }
        }
    }
}
