use crate::cn;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct RadioGroupLabelProps {
    #[props(into, default)]
    pub class: String,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn RadioGroupLabel(props: RadioGroupLabelProps) -> Element {
    rsx! {
        label {
            "data-slot": "radio-group-label",
            class: cn("text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70", &props.class),
            ..props.attributes,
            {props.children}
        }
    }
}
