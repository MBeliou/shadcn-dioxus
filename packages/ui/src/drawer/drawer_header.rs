use crate::cn;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct DrawerHeaderProps {
    #[props(into, default)]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn DrawerHeader(props: DrawerHeaderProps) -> Element {
    rsx! {
        div {
            "data-slot": "drawer-header",
            class: cn("flex flex-col space-y-2 text-center sm:text-left", &props.class),
            {props.children}
        }
    }
}
