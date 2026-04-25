use crate::cn;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct TableBodyProps {
    #[props(into, default)]
    pub class: String,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn TableBody(props: TableBodyProps) -> Element {
    rsx! {
        tbody {
            "data-slot": "table-body",
            class: cn("[&_tr:last-child]:border-0", &props.class),
            ..props.attributes,
            {props.children}
        }
    }
}
