use dioxus::prelude::*;
use crate::cn;
#[derive(Clone, PartialEq, Props)]
pub struct CardDescriptionProps {
    #[props(into, default)]
    pub class: String,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}
#[component]
pub fn CardDescription(props: CardDescriptionProps) -> Element {
    rsx! {
        div {
            "data-slot": "card-description",
            class: cn("text-muted-foreground text-sm", &props.class),
            ..props.attributes,
            {props.children}
        }
    }
}
