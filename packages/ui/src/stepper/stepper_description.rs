use crate::cn;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct StepperDescriptionProps {
    #[props(into, default)]
    pub class: String,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn StepperDescription(props: StepperDescriptionProps) -> Element {
    rsx! {
        p { "data-slot": "stepper-description",
            class: cn("text-sm text-muted-foreground", &props.class),
            ..props.attributes, {props.children}
        }
    }
}
