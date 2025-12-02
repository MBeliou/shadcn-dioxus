use dioxus::prelude::*;

use crate::cn;

#[derive(Clone, PartialEq, Props)]
pub struct EmptyTitleProps {
    #[props(into, default)]
    pub class: String,

    pub children: Element,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn EmptyTitle(props: EmptyTitleProps) -> Element {
    rsx! {
        div {
            "data-slot": "empty-title",
            class: cn("text-lg font-medium tracking-tight", &props.class),
        }
    }
}
