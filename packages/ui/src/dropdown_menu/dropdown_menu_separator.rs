use crate::cn;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct DropdownMenuSeparatorProps {
    #[props(into, default)]
    pub class: String,
}

#[component]
pub fn DropdownMenuSeparator(props: DropdownMenuSeparatorProps) -> Element {
    rsx! {
        div {
            "data-slot": "dropdown-menu-separator",
            role: "separator",
            class: cn("-mx-1 my-1 h-px bg-muted", &props.class),
        }
    }
}
