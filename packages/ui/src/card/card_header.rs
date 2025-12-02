use dioxus::prelude::*;

use crate::cn;

#[derive(Clone, PartialEq, Props)]
pub struct CardHeaderProps {
    #[props(into, default)]
    pub class: String,

    pub children: Element,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn CardHeader(props: CardHeaderProps) -> Element {
    rsx! {
        div {
            "data-slot": "card-header",
            class: cn("@container/card-header has-data-[slot=card-action]:grid-cols-[1fr_auto] [.border-b]:pb-6 grid auto-rows-min grid-rows-[auto_auto] items-start gap-1.5 px-6", &props.class),
            ..props.attributes,
            {props.children}
        }
    }
}

    