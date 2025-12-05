use crate::cn;
use dioxus::prelude::*;
use super::primitive::ProgressPrimitive;
#[derive(Props, Clone, PartialEq)]
pub struct ProgressProps {
    /// Current progress value (None = indeterminate)
    #[props(default)]
    pub value: Option<f64>,
    /// Maximum value (default: 100)
    #[props(default = 100.0)]
    pub max: f64,
    #[props(into, default)]
    pub class: String,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}
#[component]
pub fn Progress(props: ProgressProps) -> Element {
    let percentage = match props.value {
        Some(v) => 100.0 - (100.0 * v / props.max),
        None => 100.0,
    };
    rsx! {
        ProgressPrimitive {
            value: props.value,
            max: props.max,
            class: cn("bg-primary/20 relative h-2 w-full overflow-hidden rounded-full", &props.class),
            attributes: props.attributes,
            div {
                "data-slot": "progress-indicator",
                class: "bg-primary h-full w-full flex-1 transition-all",
                style: "transform: translateX(-{percentage}%)",
            }
        }
    }
}
