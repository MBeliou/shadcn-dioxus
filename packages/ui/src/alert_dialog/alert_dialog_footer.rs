use crate::cn;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct AlertDialogFooterProps {
    #[props(into, default)]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn AlertDialogFooter(props: AlertDialogFooterProps) -> Element {
    rsx! {
        div {
            "data-slot": "alert-dialog-footer",
            class: cn("flex flex-col-reverse sm:flex-row sm:justify-end sm:space-x-2", &props.class),
            {props.children}
        }
    }
}
