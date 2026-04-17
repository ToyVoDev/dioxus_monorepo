use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::md_file_icons::MdFolder;

#[component]
pub fn IconPicker(selected: String, on_change: EventHandler<String>) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        button {
            class: "icon-picker",
            onclick: move |_| {
                let new_icon = if selected.as_str() == "folder" {
                    "dns".to_string()
                } else {
                    "folder".to_string()
                };
                on_change.call(new_icon);
            },
            Icon { icon: MdFolder, width: 24, height: 24 }
        }
    }
}
