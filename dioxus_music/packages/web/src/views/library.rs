use dioxus::prelude::*;
use dioxus_music_ui::TrackList;

#[component]
pub fn Library() -> Element {
    let mut tracks = use_server_future(dioxus_music_api::get_library)?;
    let mut rescanning = use_signal(|| false);

    let result = tracks.read().clone();

    rsx! {
        div { class: "library__header",
            h1 { class: "library__title", "All Songs" }
            button {
                class: "library__rescan-btn",
                disabled: rescanning(),
                onclick: move |_| async move {
                    rescanning.set(true);
                    if let Ok(()) = dioxus_music_api::rescan_library().await {
                        tracks.restart();
                    }
                    rescanning.set(false);
                },
                if rescanning() { "Scanning..." } else { "Rescan Library" }
            }
        }

        match result {
            Some(Ok(tracks)) => rsx! {
                TrackList { tracks }
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
