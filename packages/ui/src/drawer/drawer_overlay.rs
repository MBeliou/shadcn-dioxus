use crate::cn;
use crate::drawer::DrawerContext;
use dioxus::prelude::*;

const OVERLAY_BASE: &str = "fixed inset-0 z-50 bg-black/80 data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0";

#[derive(Clone, PartialEq, Props)]
pub struct DrawerOverlayProps {
    #[props(into, default)]
    pub class: String,
}

#[component]
pub fn DrawerOverlay(props: DrawerOverlayProps) -> Element {
    let ctx = use_context::<DrawerContext>();
    let open = ctx.open;
    let set_open = ctx.set_open;

    if !open() {
        return rsx! {};
    }

    let data_state = "open";
    let classes = cn(OVERLAY_BASE, &props.class);

    rsx! {
        div {
            "data-slot": "drawer-overlay",
            "data-state": data_state,
            class: "{classes}",
            onclick: move |_| set_open.call(false),
        }
    }
}
