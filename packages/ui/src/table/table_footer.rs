use crate::cn;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct TableFooterProps {
    #[props(into, default)]
    pub class: String,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn TableFooter(props: TableFooterProps) -> Element {
    rsx! {
        tfoot {
            "data-slot": "table-footer",
            class: cn("border-t bg-muted/50 font-medium [&>tr]:last:border-b-0", &props.class),
            ..props.attributes,
            {props.children}
        }
    }
}
