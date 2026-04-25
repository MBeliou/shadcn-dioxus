use crate::cn;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct SidebarInsetProps {
    #[props(into, default)]
    pub class: String,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn SidebarInset(props: SidebarInsetProps) -> Element {
    rsx! {
        main {
            "data-slot": "sidebar-inset",
            class: cn("relative flex-1", &props.class),
            ..props.attributes,
            {props.children}
        }
    }
}
