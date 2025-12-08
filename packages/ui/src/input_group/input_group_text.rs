use crate::cn;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct InputGroupTextProps {
    #[props(into, default)]
    pub class: String,

    pub children: Element,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn InputGroupText(props: InputGroupTextProps) -> Element {
    rsx! {
        span {
            class: cn(
                "text-muted-foreground flex items-center gap-2 text-sm [&_svg:not([class*='size-'])]:size-4 [&_svg]:pointer-events-none",
                &props.class
            ),
            ..props.attributes,
            {props.children}
        }
    }
}
