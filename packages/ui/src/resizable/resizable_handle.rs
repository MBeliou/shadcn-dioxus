use crate::cn;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct ResizableHandleProps {
    #[props(default = "vertical".to_string())]
    pub orientation: String,
    #[props(into, default)]
    pub class: String,
    #[props(default = false)]
    pub disabled: bool,
}

#[component]
pub fn ResizableHandle(props: ResizableHandleProps) -> Element {
    let is_vertical = props.orientation == "vertical";
    let base_class = if is_vertical {
        "relative flex w-px items-center justify-center bg-border cursor-col-resize"
    } else {
        "relative flex h-px w-full items-center justify-center bg-border cursor-row-resize"
    };
    let classes = cn(base_class, &props.class);
    rsx! {
        div {
            "data-slot": "resizable-handle",
            "data-orientation": "{props.orientation}",
            class: "{classes}",
            tabindex: "0",
        }
    }
}
