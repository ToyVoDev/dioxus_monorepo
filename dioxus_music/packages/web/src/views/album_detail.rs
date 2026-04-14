use dioxus::prelude::*;
use dioxus_music_ui::{TrackList, api_client::use_api_client, player_state::use_player_state};

#[component]
pub fn AlbumDetail(name: String) -> Element {
    let client = use_api_client();

    // Fetch all albums, find the one matching this name.
    let albums = use_resource(move || {
        let client = client.clone();
        let name = name.clone();
        async move {
            let result = client.get_albums(None).await.ok()?;
            result.items.into_iter().find(|a| a.name == name)
        }
    });

    let album = albums.read();
    let album = match &*album {
        Some(Some(a)) => a.clone(),
        Some(None) => return rsx! { p { "Album not found." } },
        None => return rsx! { p { "Loading\u{2026}" } },
    };

    let album_id = album.id;
    let tracks = use_resource(move || {
        let client = client.clone();
        async move { client.get_album_tracks(album_id).await.ok() }
    });

    let image_url = album
        .image_tags
        .as_ref()
        .and_then(|t| t.get("Primary"))
        .map(|_| client.image_url(album.id, "Primary"));

    rsx! {
        div { class: "album-detail",
            div { class: "album-detail__header",
                if let Some(url) = image_url {
                    img { class: "album-detail__art", src: url }
                }
                div { class: "album-detail__meta",
                    h1 { "{album.name}" }
                    if let Some(artist) = &album.album_artist {
                        p { "{artist}" }
                    }
                    if let Some(year) = album.production_year {
                        p { "{year}" }
                    }
                }
            }
            match &*tracks.read() {
                Some(Some(result)) => rsx! {
                    TrackList { tracks: result.items.clone() }
                },
                Some(None) => rsx! { p { "Failed to load tracks." } },
                None => rsx! { p { "Loading tracks\u{2026}" } },
            }
        }
    }
}
