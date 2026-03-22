use crate::audio::render_audio_element;
use crate::header::Header;
use crate::player_bar::PlayerBar;
use crate::queue_panel::QueuePanel;
use dioxus::prelude::*;
use kinetic_ui::KineticTheme;

const APP_SHELL_CSS: Asset = asset!("/assets/styling/app-shell.css");

#[component]
pub fn AppShell(
    sidebar: Element,
    children: Element,
    #[props(default)] player_bar_hidden: bool,
    #[props(default)] on_player_expand: Option<EventHandler<()>>,
) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: APP_SHELL_CSS }
        KineticTheme {
            div { class: "app-shell",
                {sidebar}
                Header {}
                main { class: "app-shell__content",
                    {children}
                    QueuePanel {}
                }
            }
            PlayerBar {
                hidden: player_bar_hidden,
                on_expand: on_player_expand,
            }
            {render_audio_element()}
        }
    }
}
