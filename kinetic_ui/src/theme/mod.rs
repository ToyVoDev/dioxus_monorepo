use dioxus::prelude::*;

#[component]
pub fn KineticTheme(children: Element) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./kinetic-theme.css") }
        document::Link { rel: "stylesheet", href: asset!("./typography.css") }
        document::Link { rel: "stylesheet", href: asset!("./utilities.css") }
        {children}
    }
}
