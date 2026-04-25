use crate::cn;
use dioxus::prelude::*;

const CONTENT_BASE: &str = "flex flex-1 overflow-auto";

#[derive(Clone, PartialEq, Props)]
pub struct SidebarContentProps {
    #[props(into, default)]
    pub class: String,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn SidebarContent(props: SidebarContentProps) -> Element {
    rsx! {
        div {
            "data-slot": "sidebar-content",
            class: cn(CONTENT_BASE, &props.class),
            ..props.attributes,
            {props.children}
        }
    }
}
