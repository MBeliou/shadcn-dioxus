use crate::cn;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct ContextMenuSeparatorProps {
    #[props(into, default)]
    pub class: String,
}

#[component]
pub fn ContextMenuSeparator(props: ContextMenuSeparatorProps) -> Element {
    rsx! {
        div {
            "data-slot": "context-menu-separator",
            role: "separator",
            class: cn("-mx-1 my-1 h-px bg-muted", &props.class),
        }
    }
}
