use crate::cn;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct MenubarSeparatorProps {
    #[props(into, default)]
    pub class: String,
}

#[component]
pub fn MenubarSeparator(props: MenubarSeparatorProps) -> Element {
    rsx! {
        div {
            "data-slot": "menubar-separator",
            role: "separator",
            class: cn("-mx-1 my-1 h-px bg-muted", &props.class),
        }
    }
}
