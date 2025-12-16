use crate::Route;

use super::{SidebarNav, SidebarLinkv2};
use dioxus::prelude::*;
#[component]
pub fn Sidebar(#[props(into, default)] active_slug: String) -> Element {
    rsx! {
        aside { class: "hidden md:block w-64 shrink-0 border-r border-border",
            div { class: "sticky top-[--header-height] h-[calc(100vh-var(--header-height))] overflow-y-auto py-6 px-4",
            div { 
                class: "pb-4",
                h4 { class: "text-sm font-semibold", "Sections" }
            }
                SidebarLinkv2 {
                    to: Route::InstallationView {  }.into(),
                    // FIXME: active_slug references a docs component slug, which we don't have. We need to fix the whole implementation
                    is_active: active_slug == "installation",
                    "Get Started"
                }
                div { class: "py-4",
                    h4 { class: "text-sm font-semibold", "Components" }
                }
                SidebarNav { active_slug }
            }
        }
    }
}
