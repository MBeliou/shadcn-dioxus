use dioxus::prelude::*;
use lucide_dioxus::LoaderCircle;
use crate::cn;
#[derive(Clone, PartialEq, Props)]
pub struct SpinnerProps {
    #[props(into, default)]
    pub class: String,
    #[props(default = 24)]
    pub size: usize,
    #[props(default = "currentColor".to_owned())]
    pub color: String,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}
#[component]
pub fn Spinner(props: SpinnerProps) -> Element {
    rsx! {
        LoaderCircle {
            class: cn("size-4 animate-spin", &props.class),
            color: props.color,
            size: props.size,
        }
    }
}
