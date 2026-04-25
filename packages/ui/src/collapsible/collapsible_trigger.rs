use crate::collapsible::CollapsibleContext;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct CollapsibleTriggerProps {
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn CollapsibleTrigger(props: CollapsibleTriggerProps) -> Element {
    let ctx = use_context::<CollapsibleContext>();
    let open = ctx.open;
    let set_open = ctx.set_open;

    rsx! {
        button {
            r#type: "button",
            "data-slot": "collapsible-trigger",
            "aria-expanded": open(),
            onclick: move |_| set_open.call(!open()),
            ..props.attributes,
            {props.children}
        }
    }
}
