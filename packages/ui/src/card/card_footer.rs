use dioxus::prelude::*;
use crate::cn;
#[derive(Clone, PartialEq, Props)]
pub struct CardFooterProps {
    #[props(into, default)]
    pub class: String,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}
#[component]
pub fn CardFooter(props: CardFooterProps) -> Element {
    rsx! {
        div {
            "data-slot": "card-footer",
            class: cn("[.border-t]:pt-6 flex items-center px-6", &props.class),
            ..props.attributes,
            {props.children}
        }
    }
}
