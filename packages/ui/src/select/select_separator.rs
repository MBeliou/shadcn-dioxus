use crate::cn;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct SelectSeparatorProps {
    #[props(into, default)]
    pub class: String,
}

#[component]
pub fn SelectSeparator(props: SelectSeparatorProps) -> Element {
    rsx! {
        div {
            "data-slot": "select-separator",
            class: cn("-mx-1 my-1 h-px bg-muted", &props.class),
        }
    }
}
