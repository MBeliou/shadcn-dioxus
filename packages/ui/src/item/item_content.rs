use dioxus::prelude::*;

use crate::cn;

#[derive(Clone, PartialEq, Props)]
pub struct ItemContentProps {
    #[props(into, default)]
    pub class: String,

    pub children: Element,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn ItemContent(props: ItemContentProps) -> Element {
    rsx! {
        div {
            "data-slot": "item-content",
            class: cn("flex flex-1 flex-col gap-1 [&+[data-slot=item-content]]:flex-none", &props.class),
            ..props.attributes,
            {props.children}

        }
    }
}
