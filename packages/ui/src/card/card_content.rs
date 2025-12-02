use dioxus::prelude::*;

use crate::cn;

#[derive(Clone, PartialEq, Props)]
pub struct CardContentProps {
    #[props(into, default)]
    pub class: String,

    pub children: Element,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn CardContent(props: CardContentProps) -> Element {
    rsx! {
        div {
            "data-slot": "card-content",
            class: cn("px-6", &props.class),
            ..props.attributes,
            {props.children}
        }
    }
}

    