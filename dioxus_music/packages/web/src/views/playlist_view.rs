use dioxus::prelude::*;
use uuid::Uuid;

#[component]
pub fn PlaylistView(id: ReadSignal<Uuid>) -> Element {
    rsx! { "Coming soon" }
}
