use crate::Route;
use super::{SidebarLinkv2, SidebarNav};
use dioxus::prelude::*;
use ui::Button;

#[component]
pub fn Sidebar(#[props(into, default)] active_slug: String) -> Element {
    let mut mobile_open = use_signal(|| false);

    rsx! {
        // Mobile toggle button
        div { class: "md:hidden fixed top-2 left-2 z-50",
            Button {
                variant: ui::ButtonVariant::Outline,
                size: ui::ButtonSize::IconSm,
                onclick: move |_| mobile_open.set(!mobile_open()),
                span { class: "sr-only", "Toggle sidebar" }
                svg {
                    class: "h-4 w-4",
                    fill: "none",
                    view_box: "0 0 24 24",
                    stroke: "currentColor",
                    stroke_width: "2",
                    path { stroke_linecap: "round", stroke_linejoin: "round", d: "M4 6h16M4 12h16M4 18h16" }
                }
            }
        }

        // Desktop sidebar – sticky full height
        div { class: "hidden md:block w-64 shrink-0",
            div { class: "sticky top-0 h-screen border-r border-border bg-background overflow-y-auto",
                div { class: "flex flex-col h-full px-4 pt-6",
                    div { class: "pb-4", h4 { class: "text-sm font-semibold", "Sections" } }
                    div { class: "flex flex-col gap-1",
                        SidebarLinkv2 { to: Route::InstallationView {}.into(), is_active: active_slug == "installation", "Installation" }
                        SidebarLinkv2 { to: Route::ComponentView {}.into(), is_active: active_slug == "components", div { "Components" } }
                        SidebarLinkv2 { to: Route::ThemingView {}.into(), is_active: active_slug == "theming", div { "Theming" } }
                    }
                    div { class: "py-4", h4 { class: "text-sm font-semibold", "Components" } }
                    div { class: "flex-1 overflow-y-auto pb-6",
                        SidebarNav { active_slug: active_slug.clone() }
                    }
                }
            }
        }

        // Mobile overlay
        if mobile_open() {
            div {
                class: "fixed inset-0 z-40 bg-black/50 md:hidden",
                onclick: move |_| mobile_open.set(false),
            }
            div {
                class: "fixed inset-y-0 left-0 z-50 w-64 bg-background border-r border-border overflow-y-auto p-4 md:hidden",
                onclick: move |evt| evt.stop_propagation(),
                button {
                    class: "absolute top-3 right-3 p-1 rounded-sm opacity-70 hover:opacity-100",
                    onclick: move |_| mobile_open.set(false),
                    span { class: "sr-only", "Close" }
                    svg {
                        class: "h-4 w-4", fill: "none", view_box: "0 0 24 24", stroke: "currentColor", stroke_width: "2",
                        path { stroke_linecap: "round", stroke_linejoin: "round", d: "M6 18L18 6M6 6l12 12" }
                    }
                }
                div { class: "pb-4 mt-8", h4 { class: "text-sm font-semibold", "Sections" } }
                div { class: "flex flex-col gap-1",
                    SidebarLinkv2 { to: Route::InstallationView {}.into(), is_active: active_slug == "installation", "Installation" }
                    SidebarLinkv2 { to: Route::ComponentView {}.into(), is_active: active_slug == "components", div { "Components" } }
                    SidebarLinkv2 { to: Route::ThemingView {}.into(), is_active: active_slug == "theming", div { "Theming" } }
                }
                div { class: "py-4", h4 { class: "text-sm font-semibold", "Components" } }
                div { onclick: move |_| mobile_open.set(false),
                    SidebarNav { active_slug: active_slug.clone(), large_text: true }
                }
            }
        }
    }
}
