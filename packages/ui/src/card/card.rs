use dioxus::prelude::*;

use crate::cn;

#[derive(Clone, PartialEq, Props)]
pub struct CardProps {
    #[props(into, default)]
    pub class: String,

    pub children: Element,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn Card(props: CardProps) -> Element {
    rsx! {
        div {
            "data-slot": "card",
            class: cn("bg-card text-card-foreground flex flex-col gap-6 rounded-xl border py-6 shadow-sm", &props.class),
            ..props.attributes,
            {props.children}
        }
    }
}
