use crate::cn;
use dioxus::prelude::*;

const HEADER_BASE: &str = "flex flex-col gap-2 text-center sm:text-left";

#[derive(Props, Clone, PartialEq)]
pub struct DialogHeaderProps {
    #[props(into, default)]
    pub class: String,

    pub children: Element,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// A layout container for the dialog header area.
/// Typically contains DialogTitle and DialogDescription.
///
/// # Example
///
/// ```rust
/// use ui::{DialogContent, DialogHeader, DialogTitle, DialogDescription};
///
/// rsx! {
///     DialogContent {
///         DialogHeader {
///             DialogTitle { "Edit Profile" }
///             DialogDescription { "Make changes to your profile." }
///         }
///     }
/// }
/// ```
#[component]
pub fn DialogHeader(props: DialogHeaderProps) -> Element {
    rsx! {
        div {
            "data-slot": "dialog-header",
            class: cn(HEADER_BASE, &props.class),
            ..props.attributes,
            {props.children}
        }
    }
}
