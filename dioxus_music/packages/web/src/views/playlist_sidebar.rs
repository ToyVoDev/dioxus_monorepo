use dioxus::prelude::*;
use dioxus_music_api::types::SmartPlaylistRules;
use dioxus_music_ui::api_client::ApiClient;

use crate::Route;

#[component]
pub fn PlaylistSidebarSection() -> Element {
    let client = use_context::<ApiClient>();
    let client_playlists = client.clone();
    let mut playlists = use_resource(move || {
        let client = client_playlists.clone();
        async move { client.get_playlists().await.ok() }
    });
    let mut show_create = use_signal(|| false);

    rsx! {
        div { class: "playlist-sidebar",
            div { class: "playlist-sidebar__actions",
                button { onclick: move |_| show_create.set(true), "+" }
            }
            match &*playlists.read() {
                Some(Some(result)) => rsx! {
                    for p in &result.items {
                        Link {
                            class: "sidebar__nav-item",
                            to: Route::PlaylistView { id: p.id },
                            "{p.name}"
                        }
                    }
                },
                _ => rsx! {},
            }
            if show_create() {
                CreatePlaylistModal {
                    on_save: move |_| {
                        show_create.set(false);
                        playlists.restart();
                    },
                    on_cancel: move |_| show_create.set(false),
                }
            }
        }
    }
}

#[component]
fn CreatePlaylistModal(on_save: EventHandler<()>, on_cancel: EventHandler<()>) -> Element {
    let client = use_context::<ApiClient>();
    let client_genres = client.clone();
    let mut name = use_signal(String::new);
    let mut is_smart = use_signal(|| false);
    let mut include_genres = use_signal(|| Vec::<String>::new());
    let mut exclude_genres = use_signal(|| Vec::<String>::new());

    let _genres_resource = use_resource(move || {
        let client = client_genres.clone();
        async move { client.get_genres().await.ok() }
    });

    rsx! {
        div { class: "modal",
            input {
                placeholder: "Playlist name",
                value: name(),
                oninput: move |e| name.set(e.value()),
            }
            label {
                input { r#type: "checkbox", checked: is_smart(), onchange: move |e| is_smart.set(e.checked()) }
                " Smart playlist"
            }
            if is_smart() {
                p { "Genre filters (advanced UI in future iteration)" }
            }
            div { class: "modal__actions",
                button {
                    onclick: move |_| {
                        let n = name();
                        let smart = is_smart();
                        let inc = include_genres();
                        let exc = exclude_genres();
                        let client = client.clone();
                        spawn(async move {
                            if smart {
                                let _ = client.create_smart_playlist(&n, SmartPlaylistRules {
                                    include_genres: inc,
                                    exclude_genres: exc,
                                }).await;
                            } else {
                                let _ = client.create_playlist(&n).await;
                            }
                        });
                        on_save.call(());
                    },
                    "Save"
                }
                button { onclick: move |_| on_cancel.call(()), "Cancel" }
            }
        }
    }
}
