use dioxus::prelude::*;

#[component]
pub fn KTable(children: Element) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }

        table {
            class: "k-table",
            {children}
        }
    }
}

#[component]
pub fn KTableHeader(columns: Vec<String>) -> Element {
    rsx! {
        thead {
            tr {
                for col in columns {
                    th { class: "k-table__header", "{col}" }
                }
            }
        }
    }
}

#[component]
pub fn KTableRow(children: Element) -> Element {
    rsx! {
        tr { class: "k-table__row", {children} }
    }
}

#[component]
pub fn KTableCell(children: Element) -> Element {
    rsx! {
        td { class: "k-table__cell", {children} }
    }
}

#[component]
pub fn KTableInput(
    #[props(default)] value: String,
    oninput: EventHandler<FormEvent>,
    #[props(default)] placeholder: Option<String>,
) -> Element {
    rsx! {
        input {
            class: "k-table__input",
            value: "{value}",
            placeholder: placeholder.unwrap_or_default(),
            oninput: move |e| oninput.call(e),
        }
    }
}

#[component]
pub fn KTableAddRow(
    onclick: EventHandler<MouseEvent>,
    #[props(default)] label: Option<String>,
) -> Element {
    let text = label.unwrap_or_else(|| "+ Add".to_string());

    rsx! {
        button {
            class: "k-table__add-row",
            onclick: move |e| onclick.call(e),
            "{text}"
        }
    }
}
