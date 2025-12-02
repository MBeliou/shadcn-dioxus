use dioxus::prelude::*;
use crate::cn;

#[derive(Props, Clone, PartialEq)]
pub struct SeparatorProps {
    #[props(into, default)]
    pub class: String,

    #[props(default = true)]
    pub horizontal: bool,

    #[props(default = false)]
    pub decorative: bool,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn Separator(props: SeparatorProps) -> Element {
    let orientation = match props.horizontal {
        true => "horizontal",
        false => "vertical",
    };

    rsx! {
        div {
            "data-slot":"separator",
            "data-orientation":orientation,
            role: if props.decorative {"separator"} else {"none"},
            class: cn("bg-border shrink-0 data-[orientation=horizontal]:h-px data-[orientation=horizontal]:w-full data-[orientation=vertical]:h-full data-[orientation=vertical]:w-px", &props.class),
            ..props.attributes
        }
    }
}
