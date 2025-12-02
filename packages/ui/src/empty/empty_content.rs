use dioxus::prelude::*;

use crate::cn;

#[derive(Clone, PartialEq, Props)]
pub struct EmptyContentProps {
    #[props(into, default)]
    pub class: String,

    pub children: Element,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn EmptyContent(props: EmptyContentProps) -> Element {
    rsx! {
        div {
            "data-slot": "empty-content",
            class: cn("flex w-full min-w-0 max-w-sm flex-col items-center gap-4 text-balance text-sm", &props.class),
        }
    }
}
