use crate::cn;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct TableRowProps {
    #[props(into, default)]
    pub class: String,
    #[props(default = false)]
    pub header: bool,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn TableRow(props: TableRowProps) -> Element {
    let base = if props.header { "[&_td]:border-b" } else { "border-b transition-colors hover:bg-muted/50 data-[state=selected]:bg-muted" };
    rsx! {
        tr {
            "data-slot": "table-row",
            class: cn(base, &props.class),
            ..props.attributes,
            {props.children}
        }
    }
}
