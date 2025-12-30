use crate::cn;
use crate::dialog::DialogContext;
use dioxus::prelude::*;

const OVERLAY_BASE: &str = "data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0  fixed inset-0 z-50 bg-black/50";

#[derive(Props, Clone, PartialEq)]
pub struct DialogOverlayProps {
    #[props(into, default)]
    pub class: String,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// A backdrop overlay that covers the screen behind the dialog.
/// Clicking on the overlay closes the dialog.
///
/// Must be used within a `DialogPortal` component.
///
/// # Example
///
/// ```rust
/// use ui::{Dialog, DialogPortal, DialogOverlay, DialogContent};
///
/// rsx! {
///     Dialog {
///         DialogPortal {
///             DialogOverlay {}
///             DialogContent {
///                 // content
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn DialogOverlay(props: DialogOverlayProps) -> Element {
    let ctx = use_context::<DialogContext>();
    let set_open = ctx.set_open;

    let data_state = "open";

    rsx! {
        div {
            "data-slot": "dialog-overlay",
            "data-state": data_state,
            class: cn(OVERLAY_BASE, &props.class),
            onclick: move |_| {
                set_open.call(false);
            },
            ..props.attributes,
        }
    }
}
