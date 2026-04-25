use crate::cn;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct SidebarHeaderProps {
    #[props(into, default)]
    pub class: String,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn SidebarHeader(props: SidebarHeaderProps) -> Element {
    rsx! {
        div {
            "data-slot": "sidebar-header",
            class: cn("flex flex-col gap-2 p-2", &props.class),
            ..props.attributes,
            {props.children}
        }
    }
}
