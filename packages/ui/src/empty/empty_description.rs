use dioxus::prelude::*;

use crate::cn;

#[derive(Clone, PartialEq, Props)]
pub struct EmptyDescriptionProps {
    #[props(into, default)]
    pub class: String,

    pub children: Element,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn EmptyDescription(props: EmptyDescriptionProps) -> Element {
    rsx! {
        div {
            "data-slot": "empty-description",
            class: cn("text-muted-foreground [&>a:hover]:text-primary text-sm/relaxed [&>a]:underline [&>a]:underline-offset-4", &props.class),
            {props.children}
        }
    }
}
