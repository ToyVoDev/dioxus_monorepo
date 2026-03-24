use crate::app::Route;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            id: "hero",
            div { id: "links",
                Link { to: Route::Modpack {}, "Minecraft Modpack" }
            }
        }
    }
}
