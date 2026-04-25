use crate::cn;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct SelectLabelProps {
    #[props(into, default)]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn SelectLabel(props: SelectLabelProps) -> Element {
    rsx! {
        div {
            "data-slot": "select-label",
            class: cn("py-1.5 pl-8 pr-2 text-sm font-semibold", &props.class),
            {props.children}
        }
    }
}
