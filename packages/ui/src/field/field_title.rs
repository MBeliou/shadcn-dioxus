use crate::cn;
use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct FieldTitleProps {
    #[props(into, default)]
    pub class: String,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}
#[component]
pub fn FieldTitle(props: FieldTitleProps) -> Element {
    rsx! {
        div {
            "data-slot": "field-title",
            class: cn(
                "flex w-fit items-center gap-2 text-sm font-medium leading-snug group-data-[disabled=true]/field:opacity-50",
                &props.class,
            ),
            ..props.attributes,
            {props.children}
        }
    }
}
