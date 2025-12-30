use crate::cn;
use crate::dialog::DialogContext;
use dioxus::prelude::*;

const OVERLAY_BASE: &str = "fixed inset-0 z-50 bg-black/50";

#[derive(Props, Clone, PartialEq)]
pub struct SheetOverlayProps {
    #[props(into, default)]
    pub class: String,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn SheetOverlay(props: SheetOverlayProps) -> Element {
    let ctx = use_context::<DialogContext>();
    let set_open = ctx.set_open;

    let data_state = "open";

    rsx! {
        div {
            "data-slot": "sheet-overlay",
            "data-state": data_state,
            class: cn(OVERLAY_BASE, &props.class),
            onclick: move |_| {
                set_open.call(false);
            },
            ..props.attributes,
        }
    }
}
