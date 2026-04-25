use crate::cn;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct AlertDialogHeaderProps {
    #[props(into, default)]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn AlertDialogHeader(props: AlertDialogHeaderProps) -> Element {
    rsx! {
        div {
            "data-slot": "alert-dialog-header",
            class: cn("flex flex-col space-y-2 text-center sm:text-left", &props.class),
            {props.children}
        }
    }
}
