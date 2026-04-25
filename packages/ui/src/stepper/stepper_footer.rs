use crate::cn;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct StepperFooterProps {
    #[props(into, default)]
    pub class: String,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn StepperFooter(props: StepperFooterProps) -> Element {
    rsx! {
        div { "data-slot": "stepper-footer",
            class: cn("flex items-center justify-between gap-2 pt-4", &props.class),
            ..props.attributes, {props.children}
        }
    }
}
