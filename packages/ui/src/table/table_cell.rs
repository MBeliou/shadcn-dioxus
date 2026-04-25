use crate::cn;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct TableCellProps {
    #[props(into, default)]
    pub class: String,
    /// HTML colspan attribute (use `column_span` in RSX)
    #[props(default = 1)]
    pub column_span: u32,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn TableCell(props: TableCellProps) -> Element {
    rsx! {
        td {
            "data-slot": "table-cell",
            colspan: "{props.column_span}",
            class: cn("p-2 align-middle [&:has([role=checkbox])]:pr-0 [&>[role=checkbox]]:translate-y-[2px]", &props.class),
            ..props.attributes,
            {props.children}
        }
    }
}
