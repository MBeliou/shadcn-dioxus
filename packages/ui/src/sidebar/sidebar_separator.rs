use crate::cn;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct SidebarSeparatorProps {
    #[props(into, default)]
    pub class: String,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn SidebarSeparator(props: SidebarSeparatorProps) -> Element {
    rsx! {
        div {
            "data-slot": "sidebar-separator",
            role: "separator",
            class: cn("mx-2 h-px bg-sidebar-border", &props.class),
            ..props.attributes,
        }
    }
}
