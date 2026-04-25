use crate::cn;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct TableHeaderProps {
    #[props(into, default)]
    pub class: String,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn TableHeader(props: TableHeaderProps) -> Element {
    rsx! {
        thead {
            "data-slot": "table-header",
            class: cn("[&_tr]:border-b", &props.class),
            ..props.attributes,
            {props.children}
        }
    }
}
