use crate::cn;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct StepperTitleProps {
    #[props(into, default)]
    pub class: String,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn StepperTitle(props: StepperTitleProps) -> Element {
    rsx! {
        h2 { "data-slot": "stepper-title",
            class: cn("text-sm font-medium text-foreground", &props.class),
            ..props.attributes, {props.children}
        }
    }
}
