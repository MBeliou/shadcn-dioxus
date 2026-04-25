use super::sidebar_context::SidebarContext;
use dioxus::prelude::*;

#[component]
pub fn SidebarMobileOverlay() -> Element {
    let ctx = use_context::<SidebarContext>();
    let open_mobile = ctx.open_mobile;
    let set_open_mobile = ctx.set_open_mobile;

    if !open_mobile() {
        return rsx! {};
    }

    rsx! {
        div {
            "data-slot": "sidebar-mobile-overlay",
            class: "fixed inset-0 z-50 bg-black/50 md:hidden",
            onclick: move |_| set_open_mobile.call(false),
        }
    }
}
