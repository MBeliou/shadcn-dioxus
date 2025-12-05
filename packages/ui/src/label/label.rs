use crate::cn;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct LabelProps {
    #[props(into, default)]
    pub class: String,

    pub children: Element,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn Label(props: LabelProps) -> Element {
    rsx! {
        label {
            "data-slot": "label",
            class: cn(
                "flex select-none items-center gap-2 text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-50 group-data-[disabled=true]:pointer-events-none group-data-[disabled=true]:opacity-50",
                &props.class
            ),
            ..props.attributes,
            {props.children}
        }
    }
}
