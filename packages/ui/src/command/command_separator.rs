use crate::cn;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct CommandSeparatorProps {
    #[props(into, default)]
    pub class: String,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn CommandSeparator(props: CommandSeparatorProps) -> Element {
    rsx! {
        div {
            "data-slot": "command-separator",
            role: "separator",
            class: cn("-mx-1 h-px bg-border", &props.class),
            ..props.attributes,
        }
    }
}
