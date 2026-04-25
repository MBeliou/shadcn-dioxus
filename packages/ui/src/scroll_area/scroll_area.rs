use crate::cn;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct ScrollAreaProps {
    #[props(into, default)]
    pub class: String,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn ScrollArea(props: ScrollAreaProps) -> Element {
    rsx! {
        div {
            "data-slot": "scroll-area",
            class: cn("relative overflow-auto rounded-[inherit] [&::-webkit-scrollbar]:w-2 [&::-webkit-scrollbar-track]:bg-transparent [&::-webkit-scrollbar-thumb]:rounded-full [&::-webkit-scrollbar-thumb]:bg-border", &props.class),
            ..props.attributes,
            div {
                class: "h-full w-full rounded-[inherit]",
                {props.children}
            }
        }
    }
}
