use crate::cn;
use crate::dialog::DialogContext;
use dioxus::prelude::*;

const TITLE_BASE: &str = "text-lg font-semibold leading-none";

#[derive(Props, Clone, PartialEq)]
pub struct DialogTitleProps {
    #[props(into, default)]
    pub class: String,

    pub children: Element,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// Title for a Dialog component
///
/// Must be used within a `Dialog` component.
///
/// # Example
///
/// ```rust
/// use ui::{DialogContent, DialogHeader, DialogTitle};
///
/// rsx! {
///     DialogContent {
///         DialogHeader {
///             DialogTitle { "Edit Profile" }
///         }
///     }
/// }
/// ```
#[component]
pub fn DialogTitle(props: DialogTitleProps) -> Element {
    let ctx = use_context::<DialogContext>();

    rsx! {
        h2 {
            "data-slot": "dialog-title",
            id: ctx.title_id.clone(),
            class: cn(TITLE_BASE, &props.class),
            ..props.attributes,
            {props.children}
        }
    }
}
