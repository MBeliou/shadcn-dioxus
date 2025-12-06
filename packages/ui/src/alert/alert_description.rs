use dioxus::prelude::*;
use crate::cn;


#[derive(Clone, PartialEq, Props)]
pub struct AlertDescriptionProps {
    #[props(into, default)]
    pub class: String,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}
#[component]
pub fn AlertDescription(props: AlertDescriptionProps) -> Element {
    rsx! {
        div {
            "data-slot": "alert-description",
            class: cn("text-muted-foreground col-start-2 grid justify-items-start gap-1 text-sm [&_p]:leading-relaxed", &props.class),
            ..props.attributes,
            {props.children}
        }
    }
}
