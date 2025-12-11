use crate::cn;
use dioxus::prelude::*;

const FOOTER_BASE: &str = "flex flex-col-reverse gap-2 sm:flex-row sm:justify-end";

#[derive(Props, Clone, PartialEq)]
pub struct DialogFooterProps {
    #[props(into, default)]
    pub class: String,

    pub children: Element,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// A layout container for the dialog footer area.
/// Typically contains action buttons like Cancel and Save.
///
/// # Example
///
/// ```rust
/// use ui::{DialogContent, DialogFooter, DialogClose, Button};
///
/// rsx! {
///     DialogContent {
///         // ... content
///         DialogFooter {
///             DialogClose {
///                 Button { variant: ButtonVariant::Outline, "Cancel" }
///             }
///             Button { "Save changes" }
///         }
///     }
/// }
/// ```
#[component]
pub fn DialogFooter(props: DialogFooterProps) -> Element {
    rsx! {
        div {
            "data-slot": "dialog-footer",
            class: cn(FOOTER_BASE, &props.class),
            ..props.attributes,
            {props.children}
        }
    }
}
