use crate::cn;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct ToastTitleProps {
    #[props(into, default)]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn ToastTitle(props: ToastTitleProps) -> Element {
    rsx! {
        div {
            "data-slot": "toast-title",
            class: cn("text-sm font-semibold", &props.class),
            {props.children}
        }
    }
}
