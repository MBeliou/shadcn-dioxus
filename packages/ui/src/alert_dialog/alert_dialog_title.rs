use crate::cn;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct AlertDialogTitleProps {
    #[props(into, default)]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn AlertDialogTitle(props: AlertDialogTitleProps) -> Element {
    rsx! {
        h2 {
            "data-slot": "alert-dialog-title",
            class: cn("text-lg font-semibold", &props.class),
            {props.children}
        }
    }
}
