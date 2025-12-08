use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct AspectRatioProps {
    #[props(default = 1.0)]
    pub ratio: f64,
    #[props(into, default)]
    pub class: String,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}
#[component]
pub fn AspectRatio(props: AspectRatioProps) -> Element {
    let padding_bottom = if props.ratio > 0.0 { 100.0 / props.ratio } else { 100.0 };
    rsx! {
        div { style: "position: relative; width: 100%; padding-bottom: {padding_bottom}%",
            div {
                "data-slot": "aspect-ratio",
                style: "position: absolute; inset: 0",
                class: "{props.class}",
                ..props.attributes,
                {props.children}
            }
        }
    }
}
