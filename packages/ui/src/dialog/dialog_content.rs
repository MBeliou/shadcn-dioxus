use crate::cn;
use crate::dialog::DialogContext;
use dioxus::prelude::*;
use lucide_dioxus::X;

const CONTENT_BASE: &str = "bg-background fixed start-[50%] top-[50%] z-50 grid w-full max-w-[calc(100%-2rem)] translate-x-[-50%] translate-y-[-50%] gap-4 rounded-lg border p-6 shadow-lg sm:max-w-lg";

const CLOSE_BUTTON_BASE: &str = "ring-offset-background focus:ring-ring absolute right-4 top-4 rounded-sm opacity-70 transition-opacity hover:opacity-100 focus:ring-2 focus:ring-offset-2 focus:outline-hidden disabled:pointer-events-none";

#[derive(Props, Clone, PartialEq)]
pub struct DialogContentProps {
    #[props(into, default)]
    pub class: String,

    #[props(default = true)]
    pub show_close_button: bool,

    pub children: Element,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// The main content container for the dialog.
/// Handles keyboard events (Escape to close) and focus management.
///
/// Must be used within a `DialogPortal` component.
///
/// # Example
///
/// ```rust
/// use ui::{Dialog, DialogPortal, DialogOverlay, DialogContent, DialogTitle};
///
/// rsx! {
///     Dialog {
///         DialogPortal {
///             DialogOverlay {}
///             DialogContent {
///                 DialogTitle { "My Dialog" }
///                 p { "Some content here." }
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn DialogContent(props: DialogContentProps) -> Element {
    let ctx = use_context::<DialogContext>();
    let set_open = ctx.set_open;

    let data_state = "open";

    rsx! {
        div {
            "data-slot": "dialog-content",
            "data-state": data_state,
            id: ctx.content_id.clone(),
            role: "dialog",
            "aria-modal": "true",
            "aria-labelledby": ctx.title_id.clone(),
            "aria-describedby": ctx.description_id.clone(),
            tabindex: "-1",
            class: cn(CONTENT_BASE, &props.class),
            onclick: move |evt| {
                // Stop propagation to prevent closing when clicking inside content
                evt.stop_propagation();
            },
            onkeydown: move |evt: KeyboardEvent| {
                if evt.key() == Key::Escape {
                    set_open.call(false);
                }
            },
            onmounted: move |data: MountedEvent| {
                // Auto-focus the dialog content when it opens
                spawn(async move {
                    let _ = data.data().set_focus(true).await;
                });
            },
            ..props.attributes,
            {props.children}
            if props.show_close_button {
                button {
                    r#type: "button",
                    "data-slot": "dialog-close-button",
                    "data-state": data_state,
                    class: CLOSE_BUTTON_BASE,
                    onclick: move |_| {
                        set_open.call(false);
                    },
                    X { class: "size-4" }
                    span {
                        class: "sr-only",
                        "Close"
                    }
                }
            }
        }
    }
}
