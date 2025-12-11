use crate::cn;
use crate::dialog::DialogContext;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct DialogCloseProps {
    #[props(into, default)]
    pub class: String,

    pub children: Element,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// A button that closes the dialog when clicked.
/// Typically used in DialogFooter to wrap a Cancel button.
///
/// Must be used within a `Dialog` component.
///
/// # Example
///
/// ```rust
/// use ui::{DialogContent, DialogFooter, DialogClose, Button, ButtonVariant};
///
/// rsx! {
///     DialogContent {
///         // ... content
///         DialogFooter {
///             DialogClose {
///                 Button { variant: ButtonVariant::Outline, "Cancel" }
///             }
///             Button { "Save" }
///         }
///     }
/// }
/// ```
#[component]
pub fn DialogClose(props: DialogCloseProps) -> Element {
    let ctx = use_context::<DialogContext>();
    let set_open = ctx.set_open;

    let data_state = "open";

    rsx! {
        button {
            r#type: "button",
            "data-slot": "dialog-close",
            "data-state": data_state,
            class: cn("", &props.class),
            onclick: move |_| {
                set_open.call(false);
            },
            ..props.attributes,
            {props.children}
        }
    }
}
