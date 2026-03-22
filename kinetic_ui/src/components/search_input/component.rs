use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::md_action_icons::MdSearch;

#[component]
pub fn KSearchInput(
    #[props(default)] placeholder: Option<String>,
    #[props(default)] value: Option<String>,
    oninput: Option<EventHandler<FormEvent>>,
) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }

        div {
            class: "k-search-input",
            span {
                class: "k-search-input__icon",
                Icon { icon: MdSearch, width: 16, height: 16 }
            }
            input {
                class: "k-search-input__field",
                r#type: "text",
                placeholder: placeholder.unwrap_or_default(),
                value: value.unwrap_or_default(),
                oninput: move |e| {
                    if let Some(f) = &oninput {
                        f.call(e);
                    }
                },
            }
        }
    }
}
