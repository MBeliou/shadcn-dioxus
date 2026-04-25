use crate::cn;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct DrawerDescriptionProps {
    #[props(into, default)]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn DrawerDescription(props: DrawerDescriptionProps) -> Element {
    rsx! {
        p {
            "data-slot": "drawer-description",
            class: cn("text-sm text-muted-foreground", &props.class),
            {props.children}
        }
    }
}
