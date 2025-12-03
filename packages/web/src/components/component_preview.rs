use dioxus::prelude::*;
#[component]
pub fn ComponentPreview(name: String) -> Element {
    rsx! {
        div { class: "not-prose my-6 rounded-lg border bg-card p-6 flex items-center justify-center min-h-[200px]",
            "TODO: Component Preview"
        }
    }
}
