use crate::player_bar::PlayerBar;
use crate::queue_panel::QueuePanel;
use dioxus::prelude::*;

const APP_SHELL_CSS: Asset = asset!("/assets/styling/app_shell.css");

#[component]
pub fn AppShell(sidebar: Element, children: Element) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: APP_SHELL_CSS }

        div { class: "app-shell",
            div { class: "app-shell__sidebar",
                {sidebar}
            }
            div { class: "app-shell__main",
                {children}
                QueuePanel {}
            }
            PlayerBar {}
        }
    }
}
