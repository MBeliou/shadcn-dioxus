use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct SidebarMenuProps {
    #[props(into, default)]
    pub class: String,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn SidebarMenu(props: SidebarMenuProps) -> Element {
    rsx! {
        div {
            "data-slot": "sidebar-menu",
            role: "menu",
            class: "flex flex-col gap-0.5 {props.class}",
            ..props.attributes,
            {props.children}
        }
    }
}
