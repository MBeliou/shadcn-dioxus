use dioxus::prelude::*;
use crate::cn;
#[derive(Clone, PartialEq, Props)]
pub struct EmptyHeaderProps {
    #[props(into, default)]
    pub class: String,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}
#[component]
pub fn EmptyHeader(props: EmptyHeaderProps) -> Element {
    rsx! {
        div {
            "data-slot": "empty-header",
            class: cn("flex max-w-sm flex-col items-center gap-2 text-center", &props.class),
            {props.children}
        }
    }
}
