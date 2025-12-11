use crate::cn;
use crate::dialog::DialogContext;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct DialogTriggerProps {
    #[props(into, default)]
    pub class: String,

    pub children: Element,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// A button that opens the dialog when clicked.
///
/// Must be used within a `Dialog` component.
///
/// # Example
///
/// ```rust
/// use ui::{Dialog, DialogTrigger};
///
/// rsx! {
///     Dialog {
///         DialogTrigger {
///             "Open Dialog"
///         }
///         // ... DialogPortal, etc.
///     }
/// }
/// ```
#[component]
pub fn DialogTrigger(props: DialogTriggerProps) -> Element {
    let ctx = use_context::<DialogContext>();
    let open = ctx.open;
    let set_open = ctx.set_open;

    let data_state = if open() { "open" } else { "closed" };

    rsx! {
        button {
            r#type: "button",
            "data-slot": "dialog-trigger",
            "data-state": data_state,
            "aria-haspopup": "dialog",
            "aria-expanded": open(),
            "aria-controls": ctx.content_id.clone(),
            class: cn("", &props.class),
            onclick: move |_| {
                set_open.call(!open());
            },
            ..props.attributes,
            {props.children}
        }
    }
}
