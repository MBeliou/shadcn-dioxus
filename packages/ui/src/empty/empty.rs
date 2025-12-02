use dioxus::prelude::*;

use crate::cn;

#[derive(Clone, PartialEq, Props)]
pub struct EmptyProps {
    #[props(into, default)]
    pub class: String,

    pub children: Element,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn Empty(props: EmptyProps) -> Element {
    rsx! {
        div {
            "data-slot": "empty",
            class: cn("flex min-w-0 flex-1 border-border flex-col items-center justify-center gap-6 text-balance rounded-lg border-dashed p-6 text-center md:p-12", &props.class),
            {props.children}
        }
    }
}
