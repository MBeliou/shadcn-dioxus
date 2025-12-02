use dioxus::prelude::*;

use crate::cn;

#[derive(Clone, PartialEq, Props)]
pub struct CardActionProps {
    #[props(into, default)]
    pub class: String,

    pub children: Element,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn CardAction(props: CardActionProps) -> Element {
    rsx! {
        div {
            "data-slot": "card-action",
            class: cn("col-start-2 row-span-2 row-start-1 self-start justify-self-end", &props.class),
            ..props.attributes,
            {props.children}
        }
    }
}

    