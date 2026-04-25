use crate::cn;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct SidebarFooterProps {
    #[props(into, default)]
    pub class: String,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn SidebarFooter(props: SidebarFooterProps) -> Element {
    rsx! {
        div {
            "data-slot": "sidebar-footer",
            class: cn("flex flex-col gap-2 border-t border-sidebar-border p-2", &props.class),
            ..props.attributes,
            {props.children}
        }
    }
}
