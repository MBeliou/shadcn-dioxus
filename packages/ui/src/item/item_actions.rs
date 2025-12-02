use dioxus::prelude::*;

use crate::cn;

#[derive(Clone, PartialEq, Props)]
pub struct ItemActionsProps {
    #[props(into, default)]
    pub class: String,

    pub children: Element,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn ItemActions(props: ItemActionsProps) -> Element {
    rsx! {
        div {
            "data-slot": "item-actions",
            class: cn("flex items-center gap-2", &props.class),
            ..props.attributes,
            {props.children}

        }
    }
}
