use dioxus::prelude::*;
use crate::cn;
#[derive(Clone, PartialEq, Props)]
pub struct ItemDescriptionProps {
    #[props(into, default)]
    pub class: String,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}
#[component]
pub fn ItemDescription(props: ItemDescriptionProps) -> Element {
    rsx! {
        div {
            "data-slot": "item-description",
            class: cn(
                "text-muted-foreground line-clamp-2 text-sm leading-normal font-normal text-balance [&>a:hover]:text-primary [&>a]:underline [&>a]:underline-offset-4",
                &props.class,
            ),
            ..props.attributes,
            {props.children}
        }
    }
}
