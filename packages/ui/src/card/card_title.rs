use dioxus::prelude::*;
use crate::cn;
#[derive(Clone, PartialEq, Props)]
pub struct CardTitleProps {
    #[props(into, default)]
    pub class: String,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}
#[component]
pub fn CardTitle(props: CardTitleProps) -> Element {
    rsx! {
        div {
            "data-slot": "card-title",
            class: cn("font-semibold leading-none", &props.class),
            ..props.attributes,
            {props.children}
        }
    }
}
