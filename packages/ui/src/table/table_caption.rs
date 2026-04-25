use crate::cn;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct TableCaptionProps {
    #[props(into, default)]
    pub class: String,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn TableCaption(props: TableCaptionProps) -> Element {
    rsx! {
        caption {
            "data-slot": "table-caption",
            class: cn("mt-4 text-sm text-muted-foreground", &props.class),
            ..props.attributes,
            {props.children}
        }
    }
}
