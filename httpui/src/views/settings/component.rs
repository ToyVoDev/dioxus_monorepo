use dioxus::prelude::*;

/// The Settings page component that will be rendered when the current route is `[Route::Settings]`
#[component]
pub fn SettingsSection() -> Element {
    rsx! {
        div {
            id: "settings",
            "Settings"
        }
    }
}
