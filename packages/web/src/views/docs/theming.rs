use crate::{docs::parse_doc, views::DocView};
use dioxus::prelude::*;

#[component]
pub fn ThemingView() -> Element {
    let markdown = include_str!("../../content/theming.md");
    rsx! {

        DocView {
            parsed_content: parse_doc(markdown),
        }
    }
}
