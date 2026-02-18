use dioxus::prelude::*;

#[component]
pub fn LayoutGrid(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        div {
            id: "layout-grid",
            ..attributes,
            {children}
        }
    }
}
