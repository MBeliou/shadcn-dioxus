use crate::cn;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct ScrollBarThumbProps {
    #[props(into, default)]
    pub class: String,
}

#[component]
pub fn ScrollBarThumb(props: ScrollBarThumbProps) -> Element {
    rsx! {
        div {
            "data-slot": "scroll-bar-thumb",
            class: cn("bg-border relative flex-1 rounded-full", &props.class),
        }
    }
}
