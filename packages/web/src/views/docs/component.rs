use crate::docs::{component_exists, loader::parse_doc, registry::get_component_doc};
use crate::views::DocView;
use dioxus::prelude::*;

#[component]
pub fn ComponentDoc(name: String) -> Element {
    let exists = component_exists(&name);

    match exists {
        true => {
            // Component exists, we can try and get its contents to render
            let doc = get_component_doc(&name).and_then(parse_doc);

            // NOTE: We used to have dedicated fallback content if a component existed but did not have a doc file yet. I've pulled the plug on that one for now, we'll revisit in the future.
            rsx! {
                DocView {
                    parsed_content: doc
                }
            }
        }
        false => {
            rsx! {
                    div { class: "text-center py-12",
                        h1 { class: "text-2xl font-bold", "Component not found" }
                        p { class: "text-muted-foreground", "The component \"{name}\" doesn't exist." }
                    }
            }
        }
    }
}
