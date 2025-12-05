use dioxus::prelude::*;
use crate::demos::get_demo;

#[component]
pub fn ComponentPreview(name: String) -> Element {
    match get_demo(&name) {
        Some(demo) => rsx! {
            div { class: "not-prose my-6 rounded-lg border bg-card p-6 flex items-center justify-center min-h-[200px]",
                {demo}
            }
        },
        None => rsx! {
            div { class: "not-prose my-6 rounded-lg border bg-muted p-6 text-muted-foreground text-center",
                "Demo '{name}' not found"
            }
        },
    }
}
