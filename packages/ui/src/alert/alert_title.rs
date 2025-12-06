use dioxus::prelude::*;
use crate::cn;

#[derive(Clone, PartialEq, Props)]
pub struct AlertTitleProps {
    #[props(into, default)]
    pub class: String,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}
#[component]
pub fn AlertTitle(props: AlertTitleProps) -> Element {
    rsx! {
        div {
            "data-slot": "alert-title",
            class: cn("col-start-2 line-clamp-1 min-h-4 font-medium tracking-tight", &props.class),
            ..props.attributes,
            {props.children}
        }
    }
}
