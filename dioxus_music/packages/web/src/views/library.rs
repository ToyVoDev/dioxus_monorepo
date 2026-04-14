use dioxus::prelude::*;
use dioxus_music_api::types::BaseItemDto;
use dioxus_music_ui::api_client::use_api_client;

use crate::Route;

const LIBRARY_CSS: Asset = asset!("/assets/library.css");

#[component]
pub fn Library() -> Element {
    let client = use_api_client();
    let albums = use_resource(move || {
        let client = client.clone();
        async move { client.get_albums(None).await.ok() }
    });

    rsx! {
        document::Link { rel: "stylesheet", href: LIBRARY_CSS }
        div { class: "library",
            match &*albums.read() {
                Some(Some(result)) => rsx! {
                    div { class: "album-grid",
                        for album in &result.items {
                            AlbumCard { album: album.clone() }
                        }
                    }
                },
                Some(None) => rsx! { p { "Failed to load library." } },
                None => rsx! { p { "Loading\u{2026}" } },
            }
        }
    }
}

#[component]
fn AlbumCard(album: BaseItemDto) -> Element {
    let client = use_api_client();
    let image_url = album
        .image_tags
        .as_ref()
        .and_then(|t| t.get("Primary"))
        .map(|_| client.image_url(album.id, "Primary"));

    rsx! {
        Link {
            class: "album-card",
            to: Route::AlbumDetail { name: album.name.clone() },
            if let Some(url) = image_url {
                img { class: "album-card__art", src: url }
            } else {
                div { class: "album-card__art album-card__art--placeholder" }
            }
            div { class: "album-card__info",
                p { class: "album-card__title", "{album.name}" }
                if let Some(artist) = &album.album_artist {
                    p { class: "album-card__artist", "{artist}" }
                }
                if let Some(year) = album.production_year {
                    p { class: "album-card__year", "{year}" }
                }
            }
        }
    }
}
