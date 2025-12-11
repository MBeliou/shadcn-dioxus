use crate::cn;
use crate::dialog::DialogContext;
use dioxus::prelude::*;

const DESCRIPTION_BASE: &str = "text-muted-foreground text-sm";

#[derive(Props, Clone, PartialEq)]
pub struct DialogDescriptionProps {
    #[props(into, default)]
    pub class: String,

    pub children: Element,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}


/// Description for a Dialog component. Usually placed within a Dialog Header.
///
/// Must be used within a `Dialog` component.
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
///             DialogDescription {
///                 "Make changes to your profile here. Click save when you're done."
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn DialogDescription(props: DialogDescriptionProps) -> Element {
    let ctx = use_context::<DialogContext>();

    rsx! {
        p {
            "data-slot": "dialog-description",
            id: ctx.description_id.clone(),
            class: cn(DESCRIPTION_BASE, &props.class),
            ..props.attributes,
            {props.children}
        }
    }
}
