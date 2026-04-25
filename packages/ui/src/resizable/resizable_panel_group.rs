use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum Direction {
    #[default]
    Horizontal,
    Vertical,
}

impl Direction {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }

    pub fn flex_dir(&self) -> &'static str {
        match self {
            Self::Horizontal => "flex-row",
            Self::Vertical => "flex-col",
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct ResizeState {
    pub sizes: Signal<Vec<f64>>,
}

#[derive(Clone, PartialEq, Props)]
pub struct ResizablePanelGroupProps {
    #[props(default)]
    pub direction: Direction,
    #[props(into, default)]
    pub class: String,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn ResizablePanelGroup(props: ResizablePanelGroupProps) -> Element {
    // Assume two panels; sizes[0] = left/top, sizes[1] = right/bottom
    let sizes = use_signal(|| vec![50.0, 50.0]); // percent
    use_context_provider(|| ResizeState { sizes: sizes.clone() });

    rsx! {
        div {
            "data-slot": "resizable-panel-group",
            "data-orientation": props.direction.as_str(),
            class: "flex {props.direction.flex_dir()} {props.class}",
            ..props.attributes,
            {props.children}
        }
    }
}
