use crate::cn;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct TableProps {
    #[props(into, default)]
    pub class: String,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn Table(props: TableProps) -> Element {
    rsx! {
        div { class: "relative w-full overflow-auto",
            table {
                "data-slot": "table",
                class: cn("w-full table-fixed caption-bottom text-sm", &props.class),
                ..props.attributes,
                {props.children}
            }
        }
    }
}
