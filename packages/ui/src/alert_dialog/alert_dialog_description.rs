use crate::cn;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct AlertDialogDescriptionProps {
    #[props(into, default)]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn AlertDialogDescription(props: AlertDialogDescriptionProps) -> Element {
    rsx! {
        p {
            "data-slot": "alert-dialog-description",
            class: cn("text-sm text-muted-foreground", &props.class),
            {props.children}
        }
    }
}
