use crate::cn;
use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum SeparatorOrientation {
    #[default]
    Vertical,
    Horizontal,
}

impl SeparatorOrientation {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Vertical => "vertical",
            Self::Horizontal => "horizontal",
        }
    }

    pub fn base_class(&self) -> &'static str {
        match self {
            Self::Vertical => "w-px",
            Self::Horizontal => "h-px w-full",
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct SeparatorProps {
    #[props(into, default)]
    pub class: String,

    #[props(default)]
    pub orientation: SeparatorOrientation,

    #[props(default = false)]
    pub decorative: bool,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn Separator(props: SeparatorProps) -> Element {
    rsx! {
        div {
            "data-slot":"separator",
            "data-orientation":props.orientation.as_str(),
            role: if props.decorative {"separator"} else {"none"},
            class: cn("bg-border shrink-0 data-[orientation=horizontal]:h-px data-[orientation=horizontal]:w-full data-[orientation=vertical]:h-full data-[orientation=vertical]:w-px", &props.class),
            ..props.attributes
        }
    }
}
