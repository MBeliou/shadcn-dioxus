use crate::cn;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct ToastDescriptionProps {
    #[props(into, default)]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn ToastDescription(props: ToastDescriptionProps) -> Element {
    rsx! {
        div {
            "data-slot": "toast-description",
            class: cn("text-sm opacity-90", &props.class),
            {props.children}
        }
    }
}
