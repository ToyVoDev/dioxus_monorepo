mod center_pdf;

use dioxus::prelude::*;

fn main() {
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        h1 { "PDF Margins" }
        p { "Coming soon..." }
    }
}
