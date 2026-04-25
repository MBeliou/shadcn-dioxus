use crate::cn;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct DrawerFooterProps {
    #[props(into, default)]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn DrawerFooter(props: DrawerFooterProps) -> Element {
    rsx! {
        div {
            "data-slot": "drawer-footer",
            class: cn("flex flex-col-reverse sm:flex-row sm:justify-end sm:space-x-2", &props.class),
            {props.children}
        }
    }
}
