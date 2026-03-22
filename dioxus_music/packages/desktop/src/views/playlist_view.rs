use crate::Route;
use dioxus::prelude::*;
use dioxus_music_ui::TrackList;
use uuid::Uuid;

#[component]
pub fn PlaylistView(id: ReadSignal<Uuid>) -> Element {
    let detail = use_server_future(move || {
        let id = id();
        async move { dioxus_music_api::get_playlist(id).await }
    })?;
    let tracks = use_server_future(move || {
        let id = id();
        async move { dioxus_music_api::get_playlist_tracks(id).await }
    })?;

    let detail_result = detail.read().clone();
    let tracks_result = tracks.read().clone();

    rsx! {
        div { style: "padding: var(--k-space-6);",
            Link {
                style: "color: var(--k-on-surface-variant); text-decoration: none; font-size: 0.875rem; display: inline-flex; align-items: center; gap: var(--k-space-1); margin-bottom: var(--k-space-4);",
                to: Route::Playlists {},
                "← Back to Playlists"
            }

            match detail_result {
                Some(Ok(pl)) => rsx! {
                    h1 {
                        style: "font-family: var(--k-font-display); font-size: 1.75rem; font-weight: 700; color: var(--k-on-surface); margin-bottom: var(--k-space-1);",
                        "{pl.name}"
                    }
                    p {
                        style: "font-family: var(--k-font-mono); font-size: 0.6875rem; color: var(--k-on-surface-variant); text-transform: uppercase; margin-bottom: var(--k-space-4);",
                        if pl.playlist_type == "smart" { "Smart Playlist" } else { "Manual Playlist" }
                    }
                },
                Some(Err(e)) => rsx! { p { "Error: {e}" } },
                None => rsx! { p { style: "color: var(--k-on-surface-variant);", "Loading..." } },
            }

            match tracks_result {
                Some(Ok(track_list)) => rsx! {
                    TrackList { tracks: track_list }
                },
                Some(Err(e)) => rsx! { p { "Error loading tracks: {e}" } },
                None => rsx! {},
            }
        }
    }
}
