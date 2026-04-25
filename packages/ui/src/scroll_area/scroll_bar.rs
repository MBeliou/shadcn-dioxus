use crate::cn;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct ScrollBarProps {
    #[props(into, default)]
    pub class: String,
    #[props(default = "vertical".to_string())]
    pub orientation: String,
    pub children: Element,
}

#[component]
pub fn ScrollBar(props: ScrollBarProps) -> Element {
    let is_vertical = props.orientation == "vertical";
    let base = if is_vertical {
        "flex touch-none select-none transition-colors h-full w-2.5 border-l border-l-transparent p-[1px]"
    } else {
        "flex touch-none select-none transition-colors h-2.5 flex-col border-t border-t-transparent p-[1px]"
    };
    rsx! {
        div {
            "data-slot": "scroll-bar",
            "data-orientation": "{props.orientation}",
            class: cn(base, &props.class),
            {props.children}
        }
    }
}
