use crate::tooltip::TooltipContext;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct TooltipTriggerProps {
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn TooltipTrigger(props: TooltipTriggerProps) -> Element {
    let ctx = use_context::<TooltipContext>();
    let set_open = ctx.set_open;

    rsx! {
        button {
            r#type: "button",
            "data-slot": "tooltip-trigger",
            onmouseenter: move |_| set_open.call(true),
            onmouseleave: move |_| set_open.call(false),
            onfocus: move |_| set_open.call(true),
            onblur: move |_| set_open.call(false),
            ..props.attributes,
            {props.children}
        }
    }
}
