use dioxus::prelude::*;
use super::resizable_panel_group::ResizeState;

#[derive(Clone, PartialEq, Props)]
pub struct ResizablePanelProps {
    #[props(default = 0)]
    pub index: usize, // 0 or 1
    #[props(into, default)]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn ResizablePanel(props: ResizablePanelProps) -> Element {
    let resize_state = use_context::<ResizeState>();
    let sizes = resize_state.sizes;
    let size = sizes().get(props.index).copied().unwrap_or(50.0);
    let style = format!("flex-basis: {}%; overflow: hidden;", size);

    rsx! {
        div {
            "data-slot": "resizable-panel",
            style: "{style}",
            class: "min-w-0 min-h-0 {props.class}",
            {props.children}
        }
    }
}
