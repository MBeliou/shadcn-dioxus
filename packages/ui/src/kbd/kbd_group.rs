use crate::cn;
use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct KbdGroupProps {
    #[props(into, default)]
    pub class: String,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}
#[component]
pub fn KbdGroup(props: KbdGroupProps) -> Element {
    rsx! {
        div {
            class: cn("inline-flex items-center gap-1", &props.class),
            "data-slot": "kbd-group",
            ..props.attributes,
            {props.children}
        }
    }
}
