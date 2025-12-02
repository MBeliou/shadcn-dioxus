use dioxus::prelude::*;

use crate::cn;

#[derive(Clone, PartialEq, Props)]
pub struct ItemTitleProps {
    #[props(into, default)]
    pub class: String,

    pub children: Element,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn ItemTitle(props: ItemTitleProps) -> Element {
    rsx! {
        div {
            "data-slot": "item-title",
            class: cn("flex w-fit items-center gap-2 text-sm leading-snug font-medium", &props.class),
            ..props.attributes,
            {props.children}

        }
    }
}
