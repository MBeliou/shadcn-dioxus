use crate::hover_card::HoverCardContext;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct HoverCardTriggerProps {
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn HoverCardTrigger(props: HoverCardTriggerProps) -> Element {
    let ctx = use_context::<HoverCardContext>();
    let set_open = ctx.set_open;

    rsx! {
        a {
            "data-slot": "hover-card-trigger",
            tabindex: "0",
            onmouseenter: move |_| set_open.call(true),
            onmouseleave: move |_| set_open.call(false),
            ..props.attributes,
            {props.children}
        }
    }
}
