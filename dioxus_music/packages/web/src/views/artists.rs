use dioxus::prelude::*;
use dioxus_music_api::types::BaseItemDto;
use dioxus_music_ui::api_client::use_api_client;

#[component]
pub fn Artists() -> Element {
    let client = use_api_client();
    let artists = use_resource(move || {
        let client = client.clone();
        async move { client.get_artists().await.ok() }
    });

    rsx! {
        div { class: "artists",
            match &*artists.read() {
                Some(Some(result)) => rsx! {
                    div { class: "artist-list",
                        for artist in &result.items {
                            ArtistRow { artist: artist.clone() }
                        }
                    }
                },
                Some(None) => rsx! { p { "Failed to load artists." } },
                None => rsx! { p { "Loading…" } },
            }
        }
    }
}

#[component]
fn ArtistRow(artist: BaseItemDto) -> Element {
    let client = use_api_client();
    let image_url = artist.image_tags
        .as_ref()
        .and_then(|t| t.get("Primary"))
        .map(|_| client.image_url(artist.id, "Primary"));

    rsx! {
        div { class: "artist-row",
            if let Some(url) = image_url {
                img { class: "artist-row__art", src: url }
            }
            span { class: "artist-row__name", "{artist.name}" }
        }
    }
}
