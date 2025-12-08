use crate::cn;
use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct FieldContentProps {
    #[props(into, default)]
    pub class: String,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}
#[component]
pub fn FieldContent(props: FieldContentProps) -> Element {
    rsx! {
        div {
            "data-slot": "field-content",
            class: cn("group/field-content flex flex-1 flex-col gap-1.5 leading-snug", &props.class),
            ..props.attributes,
            {props.children}
        }
    }
}
