use crate::components::{ComponentPreview, PmBlock};
use dioxus::prelude::*;
use dioxus_markdown::CustomComponents;

/**
 * Register all custom components for use in Markdown rendering here
 */
pub fn create_doc_components() -> CustomComponents {
    let mut components = CustomComponents::new();
    components.register("ComponentPreview", |props| {
        let name = props
            .get("name")
            .map(|v| v.as_str().to_string())
            .unwrap_or_default();
        Ok(rsx! {
            ComponentPreview { name: name.to_string() }
        })
    });

    components.register("PmBlock", |props| {
        let command = props
            .get("command")
            .map(|v| v.as_str().to_string())
            .unwrap_or_default();

        Ok(rsx! {
            PmBlock {  command:command.to_string()}
        })
    });

    components
}
