use crate::cn;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct DrawerTitleProps {
    #[props(into, default)]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn DrawerTitle(props: DrawerTitleProps) -> Element {
    rsx! {
        h2 {
            "data-slot": "drawer-title",
            class: cn("text-lg font-semibold leading-none tracking-tight", &props.class),
            {props.children}
        }
    }
}
