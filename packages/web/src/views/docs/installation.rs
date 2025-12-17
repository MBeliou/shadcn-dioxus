use crate::{docs::parse_doc, views::DocView};
use dioxus::prelude::*;

#[component]
pub fn InstallationView() -> Element {
    let markdown = include_str!("../../content/get-started.md");
    rsx! {

        DocView {
            parsed_content: parse_doc(markdown),
        }
    }
}
