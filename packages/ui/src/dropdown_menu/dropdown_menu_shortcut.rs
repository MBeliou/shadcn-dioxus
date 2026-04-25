use crate::cn;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct DropdownMenuShortcutProps {
    #[props(into, default)]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn DropdownMenuShortcut(props: DropdownMenuShortcutProps) -> Element {
    rsx! {
        span {
            "data-slot": "dropdown-menu-shortcut",
            class: cn("ml-auto text-xs tracking-widest opacity-60", &props.class),
            {props.children}
        }
    }
}
