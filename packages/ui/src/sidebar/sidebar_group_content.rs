use super::sidebar_group::SidebarGroupContext;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct SidebarGroupContentProps {
    #[props(into, default)]
    pub class: String,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn SidebarGroupContent(props: SidebarGroupContentProps) -> Element {
    let ctx = use_context::<SidebarGroupContext>();
    let is_open = ctx.is_open;

    if !is_open() {
        return rsx! {};
    }

    rsx! {
        div {
            "data-slot": "sidebar-group-content",
            class: "flex flex-col gap-0.5 {props.class}",
            ..props.attributes,
            {props.children}
        }
    }
}
