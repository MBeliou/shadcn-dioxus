use crate::cn;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct DropdownMenuLabelProps {
    #[props(into, default)]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn DropdownMenuLabel(props: DropdownMenuLabelProps) -> Element {
    rsx! {
        div {
            "data-slot": "dropdown-menu-label",
            class: cn("px-2 py-1.5 text-sm font-semibold", &props.class),
            {props.children}
        }
    }
}
