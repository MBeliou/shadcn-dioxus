use crate::alert_dialog::AlertDialogContext;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct AlertDialogTriggerProps {
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn AlertDialogTrigger(props: AlertDialogTriggerProps) -> Element {
    let ctx = use_context::<AlertDialogContext>();
    let open = ctx.open;
    let set_open = ctx.set_open;

    rsx! {
        button {
            r#type: "button",
            "data-slot": "alert-dialog-trigger",
            "aria-expanded": open(),
            onclick: move |_| set_open.call(!open()),
            ..props.attributes,
            {props.children}
        }
    }
}
