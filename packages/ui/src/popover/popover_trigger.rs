use crate::popover::PopoverContext;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct PopoverTriggerProps {
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn PopoverTrigger(props: PopoverTriggerProps) -> Element {
    let ctx = use_context::<PopoverContext>();
    let open = ctx.open;
    let set_open = ctx.set_open;

    rsx! {
        button {
            r#type: "button",
            "data-slot": "popover-trigger",
            "aria-expanded": open(),
            onclick: move |_| set_open.call(!open()),
            ..props.attributes,
            {props.children}
        }
    }
}
