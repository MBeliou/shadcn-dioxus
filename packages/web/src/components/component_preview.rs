use dioxus::prelude::*;
use crate::demos::get_demo;

#[component]
pub fn ComponentPreview(name: String) -> Element {
    match get_demo(&name) {
        Some(demo) => {
            rsx! {
                div { class: "not-prose my-4 sm:my-6 rounded-lg border bg-background p-3 sm:p-6 flex items-center justify-center min-h-[150px] sm:min-h-[200px] overflow-x-auto",
                    {demo}
                }
            }
        }
        None => {
            rsx! {
                div { class: "not-prose my-4 sm:my-6 rounded-lg border bg-background p-3 sm:p-6 text-muted-foreground text-center",
                    "Demo '{name}' not found"
                }
            }
        }
    }
}
