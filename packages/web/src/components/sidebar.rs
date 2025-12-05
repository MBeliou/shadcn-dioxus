use super::SidebarNav;
use dioxus::prelude::*;
#[component]
pub fn Sidebar(#[props(into, default)] active_slug: String) -> Element {
    rsx! {
        aside { class: "hidden md:block w-64 shrink-0 border-r border-border",
            div { class: "sticky top-[--header-height] h-[calc(100vh-var(--header-height))] overflow-y-auto py-6 px-4",
                div { class: "pb-4",
                    h4 { class: "text-sm font-semibold", "Components" }
                }
                SidebarNav { active_slug }
            }
        }
    }
}
