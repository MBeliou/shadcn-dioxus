use crate::cn;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct StepperSeparatorProps {
    #[props(into, default)]
    pub class: String,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn StepperSeparator(props: StepperSeparatorProps) -> Element {
    rsx! {
        div { "data-slot": "stepper-separator",
            class: cn("mx-3 h-px w-full bg-border", &props.class),
            ..props.attributes,
        }
    }
}
